use pos_chain::{types::*, consensus::Consensus, network, config::Config, peer_manager::PeerManager};
use tokio::sync::mpsc;
use tokio::time::{interval, Duration};
use std::sync::{Arc, Mutex};

#[tokio::main]
async fn main() {
    let config = Config::load().expect("Failed to load config.toml");
    
    let listen_addr = config.listen_addr.clone();
    let rpc_addr = config.rpc_addr.clone();
    let my_addr = listen_addr.clone();
    
    let state = Arc::new(Mutex::new(ChainState::new()));
    let consensus = Arc::new(Mutex::new(Consensus::new()));
    let peer_manager = Arc::new(Mutex::new(PeerManager::new(config.bootstrap_nodes.clone())));
    let mempool = Arc::new(Mutex::new(Mempool::new()));
    
    {
        let mut s = state.lock().unwrap();
        for (address, balance) in config.genesis {
            s.accounts.insert(address, balance);
        }
    }
    
    {
        let mut c = consensus.lock().unwrap();
        for (address, stake) in config.validators {
            c.register_validator(address, stake);
        }
    }
    
    let (tx, mut rx) = mpsc::channel::<(NetworkMessage, String)>(100);
    
    let peer_manager_clone = Arc::clone(&peer_manager);
    let tx_listener = tx.clone();
    tokio::spawn(async move {
        network::start_listener(&listen_addr, tx_listener, peer_manager_clone).await;
    });
    
    let state_rpc = Arc::clone(&state);
    let mempool_rpc = Arc::clone(&mempool);
    tokio::spawn(async move {
        pos_chain::rpc::start_rpc_server(&rpc_addr, state_rpc, mempool_rpc).await;
    });
    
    let peer_manager_clone = Arc::clone(&peer_manager);
    let my_addr_clone = my_addr.clone();
    let tx_clone = tx.clone();
    tokio::spawn(async move {
        let mut connect_interval = interval(Duration::from_secs(30));
        
        loop {
            connect_interval.tick().await;
            
            let bootstrap = {
                let pm = peer_manager_clone.lock().unwrap();
                pm.get_bootstrap_nodes()
            };
            
            for node in bootstrap {
                if node != my_addr_clone {
                    let pm = Arc::clone(&peer_manager_clone);
                    let addr = my_addr_clone.clone();
                    let tx = tx_clone.clone();
                    let node = node.clone();
                    tokio::spawn(async move {
                        network::connect_and_handle_peer(node, addr, tx, pm).await;
                    });
                }
            }
            
            let to_connect = {
                let pm = peer_manager_clone.lock().unwrap();
                pm.get_peers_to_connect()
            };
            
            for peer in to_connect {
                if peer != my_addr_clone {
                    let pm = Arc::clone(&peer_manager_clone);
                    let addr = my_addr_clone.clone();
                    let tx = tx_clone.clone();
                    tokio::spawn(async move {
                        network::connect_and_handle_peer(peer, addr, tx, pm).await;
                    });
                }
            }
            
            {
                let mut pm = peer_manager_clone.lock().unwrap();
                pm.cleanup_stale_peers();
            }
        }
    });
    
    let state_clone = Arc::clone(&state);
    let consensus_clone = Arc::clone(&consensus);
    let peer_manager_clone = Arc::clone(&peer_manager);
    let mempool_clone = Arc::clone(&mempool);
    tokio::spawn(async move {
        let mut block_interval = interval(Duration::from_secs(10));
        let mut slot = 0u64;
        
        loop {
            block_interval.tick().await;
            
            let producer = {
                let c = consensus_clone.lock().unwrap();
                c.select_producer(slot)
            };
            
            if let Some(producer) = producer {
                let transactions = {
                    let mut mp = mempool_clone.lock().unwrap();
                    mp.get_pending(100)
                };
                
                let block = Block {
                    slot,
                    parent_hash: if slot > 0 { format!("block_{}", slot - 1) } else { "genesis".to_string() },
                    hash: format!("block_{}", slot),
                    producer: producer.clone(),
                    timestamp: slot * 10,
                    transactions,
                };
                
                let mut s = state_clone.lock().unwrap();
                if s.add_block(block.clone()) {
                    println!("Slot {}: Producer {} | {} transactions", 
                        slot, producer, block.transactions.len());
                    
                    drop(s);
                    
                    let msg = NetworkMessage::NewBlock(block);
                    let pm = Arc::clone(&peer_manager_clone);
                    tokio::spawn(async move {
                        network::broadcast_message(msg, pm).await;
                    });
                }
            }
            
            slot += 1;
        }
    });
    
    loop {
        if let Some((msg, peer_addr)) = rx.recv().await {
            match msg {
                NetworkMessage::Handshake { peer_addr: _their_addr, known_peers } => {
                    println!("Handshake from {} with {} known peers", peer_addr, known_peers.len());
                    let mut pm = peer_manager.lock().unwrap();
                    for peer in known_peers {
                        if peer != my_addr {
                            pm.add_peer(peer);
                        }
                    }
                }
                NetworkMessage::NewBlock(block) => {
                    let mut s = state.lock().unwrap();
                    if s.add_block(block.clone()) {
                        println!("Received block from {}: slot {}", peer_addr, block.slot);
                        
                        drop(s);
                        
                        let msg = NetworkMessage::NewBlock(block);
                        let pm = Arc::clone(&peer_manager);
                        tokio::spawn(async move {
                            network::broadcast_message(msg, pm).await;
                        });
                    }
                }
                NetworkMessage::Ping => {
                    println!("Ping from {}", peer_addr);
                }
                _ => {}
            }
        }
    }
}
