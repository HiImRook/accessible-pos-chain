use pos_chain::{types::*, consensus::Consensus, network, config::Config, peer_manager::PeerManager, metrics::Metrics, tpi::TpiHashMessage};
use pos_chain::tpi_production::produce_block_with_tpi;
use tokio::sync::mpsc;
use tokio::time::{interval, Duration};
use std::sync::Arc;
use tokio::sync::{Mutex, RwLock};
use std::time::{SystemTime, UNIX_EPOCH};
use std::collections::HashMap;

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
    let args: Vec<String> = std::env::args().collect();
    let my_validator_id = if args.len() > 1 {
        args[1].clone()
    } else {
        "validator_1".to_string()
    };

    println!("Starting validator: {}", my_validator_id);

    let config = Config::load().expect("Failed to load config.toml");

    if config.validators.is_empty() {
        eprintln!("ERROR: No validators configured in config.toml");
        eprintln!("Add at least one validator to the [validators] section");
        std::process::exit(1);
    }

    if config.bootstrap_nodes.is_empty() {
        eprintln!("WARNING: No bootstrap nodes configured");
        eprintln!("This validator will not connect to any peers");
    }

    for node in &config.bootstrap_nodes {
        if !node.contains(':') {
            eprintln!("ERROR: Malformed bootstrap node: {}", node);
            eprintln!("Expected format: 'host:port' (e.g., '127.0.0.1:8000')");
            std::process::exit(1);
        }
    }

    if config.genesis_timestamp > 0 {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        
        if config.genesis_timestamp > now + 300 {
            eprintln!("ERROR: Genesis timestamp is more than 5 minutes in the future");
            eprintln!("genesis_timestamp: {}, current time: {}", config.genesis_timestamp, now);
            std::process::exit(1);
        }
        
        if config.genesis_timestamp < now.saturating_sub(86400 * 365) {
            eprintln!("ERROR: Genesis timestamp is more than 1 year in the past");
            eprintln!("genesis_timestamp: {}, current time: {}", config.genesis_timestamp, now);
            std::process::exit(1);
        }
    }

    let listen_addr = config.listen_addr.clone();
    let rpc_addr = config.rpc_addr.clone();
    let my_addr = listen_addr.clone();

    let state = Arc::new(RwLock::new(ChainState::new()));
    let consensus = Arc::new(RwLock::new(Consensus::new()));
    let peer_manager = Arc::new(Mutex::new(PeerManager::new(config.bootstrap_nodes.clone())));
    let mempool = Arc::new(Mutex::new(Mempool::new()));
    let metrics = Metrics::new();

    let my_genesis = if config.genesis_timestamp == 0 {
        let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
        println!("First validator - setting genesis to current time: {}", now);
        now
    } else {
        println!("Using genesis from config: {}", config.genesis_timestamp);
        config.genesis_timestamp
    };
    let genesis_timestamp = Arc::new(Mutex::new(my_genesis));
    let genesis_ms = my_genesis * 1000;

    {
        let mut s = state.write().await;
        for (address, balance) in config.genesis {
            s.accounts.insert(address, balance);
        }
    }

    {
        let mut c = consensus.write().await;
        for (address, stake) in config.validators {
            c.register_validator(address, stake);
        }
    }

    let (tx, mut rx) = mpsc::channel::<(NetworkMessage, String)>(100);
    let (tpi_tx, tpi_rx) = mpsc::channel::<TpiHashMessage>(100);

    let peer_manager_clone = Arc::clone(&peer_manager);
    let tx_listener = tx.clone();
    let tpi_tx_listener = tpi_tx.clone();
    tokio::spawn(async move {
        network::start_listener(&listen_addr, tx_listener, tpi_tx_listener, peer_manager_clone).await;
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
    let tpi_tx_clone = tpi_tx.clone();
    let genesis_timestamp_connect = Arc::clone(&genesis_timestamp);
    tokio::spawn(async move {
        let mut connect_interval = interval(Duration::from_secs(30));

        loop {
            connect_interval.tick().await;

            let genesis_ts = *genesis_timestamp_connect.lock().await;

            let bootstrap = {
                let pm = peer_manager_clone.lock().await;
                pm.get_bootstrap_nodes()
            };

            for node in bootstrap {
                if node != my_addr_clone {
                    let pm = Arc::clone(&peer_manager_clone);
                    let addr = my_addr_clone.clone();
                    let tx = tx_clone.clone();
                    let tpi_tx = tpi_tx_clone.clone();
                    let node = node.clone();
                    tokio::spawn(async move {
                        network::connect_and_handle_peer(node, addr, tx, tpi_tx, pm, genesis_ts).await;
                    });
                }
            }

            let to_connect = {
                let pm = peer_manager_clone.lock().await;
                pm.get_peers_to_connect()
            };

            for peer in to_connect {
                if peer != my_addr_clone {
                    let pm = Arc::clone(&peer_manager_clone);
                    let addr = my_addr_clone.clone();
                    let tx = tx_clone.clone();
                    let tpi_tx = tpi_tx_clone.clone();
                    tokio::spawn(async move {
                        network::connect_and_handle_peer(peer, addr, tx, tpi_tx, pm, genesis_ts).await;
                    });
                }
            }

            {
                let mut pm = peer_manager_clone.lock().await;
                pm.cleanup_stale_peers();
            }
        }
    });

    let state_clone = Arc::clone(&state);
    let consensus_clone = Arc::clone(&consensus);
    let peer_manager_clone = Arc::clone(&peer_manager);
    let mempool_clone = Arc::clone(&mempool);
    let metrics_clone = Arc::clone(&metrics);
    let tpi_rx = Arc::new(Mutex::new(tpi_rx));
    let tpi_tx_block = tpi_tx.clone();
    let validator_id_for_block = my_validator_id.clone();

    let mut current_slot = 0u64;
    let mut slot_deadline = tokio::time::Instant::now();

    loop {
        tokio::select! {
            Some((msg, peer_addr)) = rx.recv() => {
                match msg {
                    NetworkMessage::Handshake { peer_addr: _their_addr, known_peers, genesis_timestamp: their_genesis } => {
                        let peer_id = generate_peer_id(&peer_addr);
                        let peer_id_short = if peer_id.len() > 12 { &peer_id[..12] } else { &peer_id };
                        println!("[{}] Handshake from {} ({} peers, genesis: {})",
                            timestamp(), peer_id_short, known_peers.len(), their_genesis);

                        let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
                        let mut our_genesis = genesis_timestamp.lock().await;
                        if their_genesis > 0
                            && their_genesis < *our_genesis
                            && their_genesis > now.saturating_sub(86400)
                            && their_genesis < now + 300
                        {
                            println!("[{}] Adopting earlier genesis: {} -> {}", timestamp(), *our_genesis, their_genesis);
                            *our_genesis = their_genesis;
                        }
                        drop(our_genesis);

                        let mut pm = peer_manager.lock().await;
                        for peer in known_peers {
                            if peer != my_addr {
                                pm.add_peer(peer);
                            }
                        }
                    }
                    NetworkMessage::NewBlock(block) => {
                        let mut s = state_clone.write().await;
                        if s.add_block(block.clone()) {
                            current_slot = block.slot;
                            slot_deadline = tokio::time::Instant::now() + Duration::from_secs(10);

                            let peer_id = generate_peer_id(&peer_addr);
                            let peer_id_short = if peer_id.len() > 12 { &peer_id[..12] } else { &peer_id };
                            println!("[{}] Block from {}: slot {}, next slot at +10s",
                                timestamp(), peer_id_short, block.slot);

                            drop(s);

                            {
                                let mempool_size = {
                                    let mp = mempool_clone.lock().await;
                                    mp.len()
                                };

                                let mut m = metrics_clone.lock().await;
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

            _ = tokio::time::sleep_until(slot_deadline) => {
                current_slot += 1;

                let all_validators: Vec<String> = {
                    let c = consensus_clone.read().await;
                    c.validators.keys().cloned().collect()
                };

                let validator_merit: HashMap<String, u64> = {
                    let c = consensus_clone.read().await;
                    c.validators.iter().map(|(k, v)| (k.clone(), *v)).collect()
                };

                let state_clone_spawn = Arc::clone(&state_clone);
                let mempool_clone_spawn = Arc::clone(&mempool_clone);
                let tpi_rx_clone = Arc::clone(&tpi_rx);
                let tpi_tx_clone = tpi_tx_block.clone();
                let peer_manager_clone_spawn = Arc::clone(&peer_manager_clone);
                let metrics_clone_spawn = Arc::clone(&metrics_clone);
                let my_id = validator_id_for_block.clone();

                tokio::spawn(async move {
                    if let Some(block) = produce_block_with_tpi(
                        current_slot,
                        my_id.clone(),
                        all_validators,
                        validator_merit,
                        state_clone_spawn.clone(),
                        mempool_clone_spawn.clone(),
                        tpi_rx_clone,
                        tpi_tx_clone,
                        peer_manager_clone_spawn.clone(),
                        genesis_ms,
                    ).await {
                        let mut s = state_clone_spawn.write().await;
                        if s.add_block(block.clone()) {
                            let producer_short = if block.producer.len() > 12 {
                                &block.producer[..12]
                            } else {
                                &block.producer
                            };
                            println!("[PRODUCE] Slot {}: Producer {} ({} tx)",
                                current_slot, producer_short, block.transactions.len());

                            drop(s);

                            {
                                let mempool_size = {
                                    let mp = mempool_clone_spawn.lock().await;
                                    mp.len()
                                };

                                let mut m = metrics_clone_spawn.lock().await;
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
                            tokio::spawn(async move {
                                network::broadcast_message(msg, peer_manager_clone_spawn).await;
                            });
                        }
                    }
                });

                slot_deadline = tokio::time::Instant::now() + Duration::from_secs(10);
            }
        }
    }
}
