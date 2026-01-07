use pos_chain::{types::*, consensus::Consensus, network, config::Config, peer_manager::PeerManager, metrics::Metrics};
use tokio::sync::mpsc;
use tokio::time::{interval, Duration};
use std::sync::{Arc, Mutex};
use std::time::{SystemTime, UNIX_EPOCH};

fn timestamp() -> String {
    let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
    let hours = (now / 3600) % 24;
    let minutes = (now / 60) % 60;
    let seconds = now % 60;
    format!("{:02}:{:02}:{:02}", hours, minutes, seconds)
}

fn get_memory_usage() -> u64 {
    #[cfg(target_os = "linux")]
    {
        if let Ok(status) = std::fs::read_to_string("/proc/self/status") {
            for line in status.lines() {
                if line.starts_with("VmRSS:") {
                    if let Some(kb) = line.split_whitespace().nth(1) {
                        if let Ok(kb_val) = kb.parse::<u64>() {
                            return kb_val / 1024;
                        }
                    }
                }
            }
        }
    }
    0
}

fn get_cpu_usage() -> f64 {
    0.0
}

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
    let metrics = Metrics::new();

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
    let metrics_rpc = Arc::clone(&metrics);
    tokio::spawn(async move {
        pos_chain::rpc::start_rpc_server(&rpc_addr, state_rpc, mempool_rpc, metrics_rpc).await;
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
    let metrics_clone = Arc::clone(&metrics);
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
                    transactions: transactions.clone(),
                };

                let mut s = state_clone.lock().unwrap();
                if s.add_block(block.clone()) {
                    let producer_short = if producer.len() > 12 { &producer[..12] } else { &producer };
                    println!("[{}] Slot {}: Producer {} ({} tx)",
                        timestamp(), slot, producer_short, block.transactions.len());

                    drop(s);

                    {
                        let mempool_size = {
                            let mp = mempool_clone.lock().unwrap();
                            mp.len()
                        };
                        
                        let mut m = metrics_clone.lock().unwrap();
                        m.record_block(pos_chain::metrics::BlockMetric {
                            slot: block.slot,
                            hash: block.hash.clone(),
                            producer: block.producer.clone(),
                            tx_count: block.transactions.len(),
                            time_ms: 10000,
                            timestamp: block.timestamp,
                        });
                        m.set_mempool_size(mempool_size);
                        
                        let memory_mb = get_memory_usage();
                        let cpu_percent = get_cpu_usage();
                        m.update_system_stats(memory_mb, cpu_percent);
                    }

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
                    let peer_id = generate_peer_id(&peer_addr);
                    let peer_id_short = if peer_id.len() > 12 { &peer_id[..12] } else { &peer_id };
                    println!("[{}] Handshake from {} ({} peers)",
                        timestamp(), peer_id_short, known_peers.len());
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
                        let peer_id = generate_peer_id(&peer_addr);
                        let peer_id_short = if peer_id.len() > 12 { &peer_id[..12] } else { &peer_id };
                        println!("[{}] Block from {}: slot {}",
                            timestamp(), peer_id_short, block.slot);

                        drop(s);

                        let msg = NetworkMessage::NewBlock(block);
                        let pm = Arc::clone(&peer_manager);
                        tokio::spawn(async move {
                            network::broadcast_message(msg, pm).await;
                        });
                    }
                }
                NetworkMessage::Ping => {
                    let peer_id = generate_peer_id(&peer_addr);
                    let peer_id_short = if peer_id.len() > 12 { &peer_id[..12] } else { &peer_id };
                    println!("[{}] Ping from {}", timestamp(), peer_id_short);
                }
                _ => {}
            }
        }
    }
}
