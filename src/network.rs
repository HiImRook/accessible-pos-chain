use tokio::net::{TcpListener, TcpStream};
use tokio::io::{AsyncRead, AsyncWrite, AsyncReadExt, AsyncWriteExt};
use tokio::sync::mpsc;
use tokio::time::Duration;
use crate::types::NetworkMessage;
use crate::tpi::TpiHashMessage;
use crate::crypto::peer_addr_hash;
use crate::address::{canonicalize_peer_addr, is_valid_peer_addr};
use crate::tls::validate_peer_certificate;
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio_rustls::TlsAcceptor;
use tokio_rustls::TlsConnector;
use rustls::pki_types::ServerName;
use rustls::ServerConfig;
use rustls::ClientConfig;
use crate::peer_manager::PeerManager;
use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};

const MAX_MESSAGE_SIZE: usize = 256 * 1024;
const MAX_INBOUND_CONNECTIONS_PER_IP: usize = 5;
const CONNECTION_RATE_WINDOW_SECS: u64 = 60;

fn current_timestamp_secs() -> u64 {
    SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs()
}

pub fn allow_inbound_connection(
    rate_state: &mut HashMap<String, Vec<u64>>,
    ip: &str,
) -> bool {
    let now = current_timestamp_secs();
    let attempts = rate_state.entry(ip.to_string()).or_default();
    attempts.retain(|&t| now.saturating_sub(t) < CONNECTION_RATE_WINDOW_SECS);
    if attempts.len() >= MAX_INBOUND_CONNECTIONS_PER_IP {
        return false;
    }
    attempts.push(now);
    true
}

async fn send_framed_message<S>(stream: &mut S, msg: &NetworkMessage) -> Result<(), std::io::Error>
where
    S: AsyncRead + AsyncWrite + Unpin,
{
    let data = serde_json::to_vec(msg)?;
    let len = data.len() as u32;

    if len > MAX_MESSAGE_SIZE as u32 {
        return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            "message too large"
        ));
    }

    stream.write_all(&len.to_be_bytes()).await?;
    stream.write_all(&data).await?;
    Ok(())
}

async fn read_framed_message<S>(stream: &mut S) -> Result<NetworkMessage, std::io::Error>
where
    S: AsyncRead + AsyncWrite + Unpin,
{
    let mut len_buf = [0u8; 4];
    tokio::time::timeout(
        Duration::from_secs(30),
        stream.read_exact(&mut len_buf)
    ).await.map_err(|_| std::io::Error::new(std::io::ErrorKind::TimedOut, "read timeout"))??;

    let len = u32::from_be_bytes(len_buf) as usize;

    if len > MAX_MESSAGE_SIZE {
        return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            "message too large"
        ));
    }

    let mut msg_buf = vec![0u8; len];
    tokio::time::timeout(
        Duration::from_secs(30),
        stream.read_exact(&mut msg_buf)
    ).await.map_err(|_| std::io::Error::new(std::io::ErrorKind::TimedOut, "read timeout"))??;

    serde_json::from_slice(&msg_buf)
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))
}

pub async fn start_listener(
    addr: &str,
    tx: mpsc::Sender<(NetworkMessage, String)>,
    tpi_tx: mpsc::Sender<TpiHashMessage>,
    peer_manager: Arc<Mutex<PeerManager>>,
    genesis_hash: String,
    tls_config: Arc<ServerConfig>,
    connection_rate_state: Arc<Mutex<HashMap<String, Vec<u64>>>>,
) {
    let acceptor = TlsAcceptor::from(tls_config);
    let listener = TcpListener::bind(addr).await.unwrap();
    println!("Listening on {}", addr);

    loop {
        let (socket, peer_addr) = listener.accept().await.unwrap();
        let transport_ip = peer_addr.ip().to_string();

        {
            let mut rate_state = connection_rate_state.lock().await;
            if !allow_inbound_connection(&mut rate_state, &transport_ip) {
                println!("[RATE] Inbound connection limit exceeded for {} — dropping", transport_ip);
                continue;
            }
        }

        let tx = tx.clone();
        let tpi_tx = tpi_tx.clone();
        let peer_manager = Arc::clone(&peer_manager);
        let genesis_hash = genesis_hash.clone();
        let acceptor = acceptor.clone();

        tokio::spawn(async move {
            let tls_stream = match acceptor.accept(socket).await {
                Ok(s) => s,
                Err(e) => {
                    println!("[TLS] Inbound handshake failed from {}: {}", transport_ip, e);
                    return;
                }
            };
            handle_inbound_peer(tls_stream, transport_ip, tx, tpi_tx, peer_manager, genesis_hash).await;
        });
    }
}

async fn handle_inbound_peer<S>(
    mut socket: S,
    transport_ip: String,
    tx: mpsc::Sender<(NetworkMessage, String)>,
    tpi_tx: mpsc::Sender<TpiHashMessage>,
    peer_manager: Arc<Mutex<PeerManager>>,
    genesis_hash: String,
) where
    S: AsyncRead + AsyncWrite + Unpin,
{
    let first_msg = match read_framed_message(&mut socket).await {
        Ok(msg) => msg,
        Err(e) => {
            println!("Failed to read handshake from inbound peer: {}", e);
            return;
        }
    };

    let (peer_hash, dial_addr) = match &first_msg {
        NetworkMessage::Handshake { peer_addr, .. } if !peer_addr.is_empty() => {
            let canonical = canonicalize_peer_addr(peer_addr, &transport_ip);
            if !is_valid_peer_addr(&canonical) {
                println!("Inbound peer sent invalid or malformed handshake address — dropping");
                return;
            }
            let hash = peer_addr_hash(&canonical, &genesis_hash);
            (hash, canonical)
        }
        _ => {
            println!("Inbound peer sent invalid or malformed handshake address — dropping");
            return;
        }
    };

    {
        let mut pm = peer_manager.lock().await;
        pm.add_peer(peer_hash.clone(), dial_addr);
        pm.mark_connected(&peer_hash);
        if !pm.record_inbound_message(&peer_hash) {
            println!("[RATE] Message rate exceeded during handshake for {} — disconnecting", peer_hash);
            pm.mark_disconnected(&peer_hash);
            return;
        }
    }

    println!("Inbound peer registered: {}", peer_hash);
    let _ = tx.send((first_msg, peer_hash.clone())).await;

    loop {
        match read_framed_message(&mut socket).await {
            Ok(msg) => {
                {
                    let mut pm = peer_manager.lock().await;
                    if !pm.record_inbound_message(&peer_hash) {
                        println!("[RATE] Message rate exceeded for {} — disconnecting", peer_hash);
                        pm.mark_disconnected(&peer_hash);
                        break;
                    }
                    pm.update_seen(&peer_hash);
                }

                match &msg {
                    NetworkMessage::TpiHash { slot, validator_id, block_hash, signature } => {
                        let tpi_msg = TpiHashMessage {
                            slot: *slot,
                            validator_id: validator_id.clone(),
                            block_hash: block_hash.clone(),
                            signature: signature.as_bytes().to_vec(),
                        };
                        let _ = tpi_tx.send(tpi_msg).await;
                    }
                    _ => {
                        let _ = tx.send((msg, peer_hash.clone())).await;
                    }
                }
            }
            Err(e) => {
                println!("Error reading from {}: {}", peer_hash, e);
                let mut pm = peer_manager.lock().await;
                pm.mark_disconnected(&peer_hash);
                break;
            }
        }
    }
}

pub async fn connect_and_handle_peer(
    addr: String,
    my_addr: String,
    tx: mpsc::Sender<(NetworkMessage, String)>,
    tpi_tx: mpsc::Sender<TpiHashMessage>,
    peer_manager: Arc<Mutex<PeerManager>>,
    genesis_timestamp: u64,
    my_rpc_addr: Option<String>,
    genesis_hash: String,
    client_tls_config: Arc<ClientConfig>,
    trusted_fingerprints: Vec<String>,
) {
    match TcpStream::connect(&addr).await {
        Ok(tcp_stream) => {
            let connector = TlsConnector::from(client_tls_config);
            let server_name = ServerName::try_from("valid-blockchain").unwrap().to_owned();

            let stream = match connector.connect(server_name, tcp_stream).await {
                Ok(s) => s,
                Err(e) => {
                    println!("[TLS] Outbound handshake failed to {}: {}", addr, e);
                    return;
                }
            };

            let fingerprint = match validate_peer_certificate(
                stream.get_ref().1.peer_certificates(),
                &trusted_fingerprints,
            ) {
                Ok(fp) => fp,
                Err(e) => {
                    println!("[TLS] Peer certificate validation failed for {}: {}", addr, e);
                    return;
                }
            };

            println!("[TLS] Trusted cert fingerprint from {}: {}", addr, fingerprint);

            let mut stream = stream;
            let peer_hash = peer_addr_hash(&addr, &genesis_hash);

            let known_peers = {
                let pm = peer_manager.lock().await;
                pm.get_all_known_peers()
            };

            let handshake = NetworkMessage::Handshake {
                peer_addr: my_addr.clone(),
                known_peers,
                genesis_timestamp,
                rpc_addr: my_rpc_addr,
            };

            if let Err(e) = send_framed_message(&mut stream, &handshake).await {
                println!("Failed to send handshake to {}: {}", peer_hash, e);
                return;
            }

            {
                let mut pm = peer_manager.lock().await;
                pm.add_peer(peer_hash.clone(), addr.clone());
                pm.mark_connected(&peer_hash);
            }

            println!("Connected to peer {}", peer_hash);

            loop {
                match read_framed_message(&mut stream).await {
                    Ok(msg) => {
                        {
                            let mut pm = peer_manager.lock().await;
                            pm.update_seen(&peer_hash);
                        }

                        match &msg {
                            NetworkMessage::TpiHash { slot, validator_id, block_hash, signature } => {
                                let tpi_msg = TpiHashMessage {
                                    slot: *slot,
                                    validator_id: validator_id.clone(),
                                    block_hash: block_hash.clone(),
                                    signature: signature.as_bytes().to_vec(),
                                };
                                let _ = tpi_tx.send(tpi_msg).await;
                            }
                            _ => {
                                let _ = tx.send((msg, peer_hash.clone())).await;
                            }
                        }
                    }
                    Err(e) => {
                        println!("Error reading from {}: {}", peer_hash, e);
                        let mut pm = peer_manager.lock().await;
                        pm.mark_disconnected(&peer_hash);
                        break;
                    }
                }
            }
        }
        Err(e) => {
            println!("Failed to connect to {}: {}", addr, e);
        }
    }
}

pub async fn broadcast_message(
    msg: NetworkMessage,
    peer_manager: Arc<Mutex<PeerManager>>,
    client_tls_config: Arc<ClientConfig>,
    trusted_fingerprints: Vec<String>,
) {
    let targets = {
        let pm = peer_manager.lock().await;
        pm.get_connected_peer_dial_targets()
    };

    for (peer_hash, dial_addr) in targets {
        match TcpStream::connect(&dial_addr).await {
            Ok(tcp_stream) => {
                let connector = TlsConnector::from(Arc::clone(&client_tls_config));
                let server_name = ServerName::try_from("valid-blockchain").unwrap().to_owned();

                match connector.connect(server_name, tcp_stream).await {
                    Ok(mut stream) => {
                        let fingerprint = match validate_peer_certificate(
                            stream.get_ref().1.peer_certificates(),
                            &trusted_fingerprints,
                        ) {
                            Ok(fp) => fp,
                            Err(e) => {
                                println!("[TLS] Peer certificate validation failed for {}: {}", peer_hash, e);
                                continue;
                            }
                        };

                        println!("[TLS] Trusted cert fingerprint from {}: {}", peer_hash, fingerprint);

                        if let Err(e) = send_framed_message(&mut stream, &msg).await {
                            println!("Failed to broadcast to {}: {}", peer_hash, e);
                        }
                    }
                    Err(e) => {
                        println!("[TLS] Broadcast handshake failed to {}: {}", peer_hash, e);
                    }
                }
            }
            Err(e) => {
                println!("Failed to connect for broadcast to {}: {}", peer_hash, e);
            }
        }
    }
}
