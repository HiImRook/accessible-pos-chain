use tokio::net::{TcpListener, TcpStream};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::sync::mpsc;
use tokio::time::Duration;
use crate::types::NetworkMessage;
use crate::tpi::TpiHashMessage;
use std::sync::Arc;
use tokio::sync::Mutex;
use crate::peer_manager::PeerManager;

const MAX_MESSAGE_SIZE: usize = 256 * 1024;

async fn send_framed_message(stream: &mut TcpStream, msg: &NetworkMessage) -> Result<(), std::io::Error> {
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

async fn read_framed_message(stream: &mut TcpStream) -> Result<NetworkMessage, std::io::Error> {
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
    peer_manager: Arc<Mutex<PeerManager>>
) {
    let listener = TcpListener::bind(addr).await.unwrap();
    println!("Listening on {}", addr);

    loop {
        let (socket, peer_addr) = listener.accept().await.unwrap();
        let peer_str = format!("{}:{}", peer_addr.ip(), peer_addr.port());
        println!("Accepted connection from {}", peer_str);

        {
            let mut pm = peer_manager.lock().await;
            pm.add_peer(peer_str.clone());
            pm.mark_connected(&peer_str);
        }

        let tx = tx.clone();
        let tpi_tx = tpi_tx.clone();
        let peer_manager = Arc::clone(&peer_manager);
        tokio::spawn(handle_peer(socket, peer_str, tx, tpi_tx, peer_manager));
    }
}

async fn handle_peer(
    mut socket: TcpStream,
    peer_addr: String,
    tx: mpsc::Sender<(NetworkMessage, String)>,
    tpi_tx: mpsc::Sender<TpiHashMessage>,
    peer_manager: Arc<Mutex<PeerManager>>
) {
    loop {
        match read_framed_message(&mut socket).await {
            Ok(msg) => {
                {
                    let mut pm = peer_manager.lock().await;
                    pm.update_seen(&peer_addr);
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
                        let _ = tx.send((msg, peer_addr.clone())).await;
                    }
                }
            }
            Err(e) => {
                println!("Error reading from {}: {}", peer_addr, e);
                let mut pm = peer_manager.lock().await;
                pm.mark_disconnected(&peer_addr);
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
) {
    match TcpStream::connect(&addr).await {
        Ok(mut stream) => {
            println!("Connected to peer {}", addr);

            let known_peers = {
                let pm = peer_manager.lock().await;
                pm.get_all_known_peers()
            };

            let handshake = NetworkMessage::Handshake {
                peer_addr: my_addr.clone(),
                known_peers,
                genesis_timestamp,
            };

            if let Err(e) = send_framed_message(&mut stream, &handshake).await {
                println!("Failed to send handshake to {}: {}", addr, e);
                return;
            }

            {
                let mut pm = peer_manager.lock().await;
                pm.add_peer(addr.clone());
                pm.mark_connected(&addr);
            }

            handle_peer(stream, addr, tx, tpi_tx, peer_manager).await;
        }
        Err(e) => {
            println!("Failed to connect to {}: {}", addr, e);
        }
    }
}

pub async fn broadcast_message(msg: NetworkMessage, peer_manager: Arc<Mutex<PeerManager>>) {
    let peers = {
        let pm = peer_manager.lock().await;
        pm.get_connected_peers()
    };

    for peer in peers {
        match TcpStream::connect(&peer).await {
            Ok(mut stream) => {
                if let Err(e) = send_framed_message(&mut stream, &msg).await {
                    println!("Failed to broadcast to {}: {}", peer, e);
                }
            }
            Err(e) => {
                println!("Failed to connect for broadcast to {}: {}", peer, e);
            }
        }
    }
}
