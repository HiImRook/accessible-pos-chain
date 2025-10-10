use tokio::net::{TcpListener, TcpStream};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::sync::mpsc;
use crate::types::NetworkMessage;
use std::sync::{Arc, Mutex};
use crate::peer_manager::PeerManager;

pub async fn start_listener(
    addr: &str, 
    tx: mpsc::Sender<(NetworkMessage, String)>,
    peer_manager: Arc<Mutex<PeerManager>>
) {
    let listener = TcpListener::bind(addr).await.unwrap();
    println!("Listening on {}", addr);

    loop {
        let (socket, peer_addr) = listener.accept().await.unwrap();
        let peer_str = format!("{}:{}", peer_addr.ip(), peer_addr.port());
        println!("Accepted connection from {}", peer_str);
        
        {
            let mut pm = peer_manager.lock().unwrap();
            pm.add_peer(peer_str.clone());
            pm.mark_connected(&peer_str);
        }
        
        let tx = tx.clone();
        let peer_manager = Arc::clone(&peer_manager);
        tokio::spawn(handle_peer(socket, peer_str, tx, peer_manager));
    }
}

async fn handle_peer(
    mut socket: TcpStream,
    peer_addr: String,
    tx: mpsc::Sender<(NetworkMessage, String)>,
    peer_manager: Arc<Mutex<PeerManager>>
) {
    let mut buf = vec![0; 16384];

    loop {
        match socket.read(&mut buf).await {
            Ok(0) => {
                println!("Peer {} disconnected", peer_addr);
                let mut pm = peer_manager.lock().unwrap();
                pm.mark_disconnected(&peer_addr);
                break;
            }
            Ok(n) => {
                if let Ok(msg) = serde_json::from_slice::<NetworkMessage>(&buf[..n]) {
                    {
                        let mut pm = peer_manager.lock().unwrap();
                        pm.update_seen(&peer_addr);
                    }
                    let _ = tx.send((msg, peer_addr.clone())).await;
                }
            }
            Err(e) => {
                println!("Read error from {}: {}", peer_addr, e);
                let mut pm = peer_manager.lock().unwrap();
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
    peer_manager: Arc<Mutex<PeerManager>>
) {
    match TcpStream::connect(&addr).await {
        Ok(mut stream) => {
            println!("Connected to peer {}", addr);
            
            let known_peers = {
                let pm = peer_manager.lock().unwrap();
                pm.get_all_known_peers()
            };
            
            let handshake = NetworkMessage::Handshake {
                peer_addr: my_addr,
                known_peers,
            };
            
            if let Ok(data) = serde_json::to_vec(&handshake) {
                if let Err(e) = stream.write_all(&data).await {
                    println!("Failed to send handshake to {}: {}", addr, e);
                    return;
                }
            }
            
            {
                let mut pm = peer_manager.lock().unwrap();
                pm.mark_connected(&addr);
            }
            
            handle_peer(stream, addr, tx, peer_manager).await;
        }
        Err(e) => {
            println!("Failed to connect to {}: {}", addr, e);
        }
    }
}

pub async fn broadcast_message(
    msg: NetworkMessage,
    peer_manager: Arc<Mutex<PeerManager>>
) {
    let peers = {
        let pm = peer_manager.lock().unwrap();
        pm.get_connected_peers()
    };

    for peer in peers {
        let msg = msg.clone();
        tokio::spawn(async move {
            if let Ok(mut stream) = TcpStream::connect(&peer).await {
                if let Ok(data) = serde_json::to_vec(&msg) {
                    let _ = stream.write_all(&data).await;
                }
            }
        });
    }
}
