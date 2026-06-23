use pos_chain::{types::*, consensus::Consensus, network, config::Config, peer_manager::PeerManager, metrics::Metrics, tpi::TpiHashMessage};
use pos_chain::tpi_production::produce_block_with_tpi;
use pos_chain::archive::{build_archive_segment, write_archive_segment, load_verified_archive_segment, segment_archive_path, blocks_per_segment, ArchiveSegment};
use pos_chain::publication::{build_publication_manifest, write_publication_manifest, read_publication_manifest, write_publication_receipt, read_publication_receipt, PublicationStatus, PUBLISH_QUEUE_DIR, PUBLISH_RECEIPTS_DIR};
use pos_chain::arweave::ArweaveClient;
use pos_chain::snapshot::compute_genesis_hash;
use tokio::sync::mpsc;
use tokio::time::{interval, Duration};
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use tokio::sync::{Mutex, RwLock};
use std::time::{SystemTime, UNIX_EPOCH};
use std::collections::{HashMap, HashSet};

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

fn archive_segment_to_disk(
    blocks: Vec<Block>,
    genesis_hash: String,
    archive_start: u64,
    seg: u64,
    path: String,
) -> Result<ArchiveSegment, String> {
    let previous_segment_checksum = if archive_start > 1 {
        let prev_end = archive_start - 1;
        let prev_start = prev_end - seg + 1;
        let prev_path = segment_archive_path(prev_start, prev_end);
        match load_verified_archive_segment(&prev_path) {
            Ok(prev_seg) => prev_seg.metadata.payload_checksum,
            Err(_) => String::new(),
        }
    } else {
        String::new()
    };

    let segment = build_archive_segment(blocks, &genesis_hash, &previous_segment_checksum)
        .ok_or_else(|| "failed to build archive segment".to_string())?;

    write_archive_segment(&segment, &path)
        .map_err(|e| format!("write failed: {}", e))?;

    load_verified_archive_segment(&path)
        .map_err(|e| {
            let _ = std::fs::remove_file(&path);
            format!("verify failed: {}", e)
        })?;

    Ok(segment)
}

async fn maybe_archive_and_prune(
    state: Arc<RwLock<ChainState>>,
    genesis_hash: String,
    latest_slot: u64,
    archiving_in_progress: Arc<Mutex<HashSet<String>>>,
) {
    let seg = blocks_per_segment();
    if latest_slot < seg * 2 || latest_slot % seg != 0 {
        return;
    }

    let archive_start = latest_slot - (seg * 2) + 1;
    let archive_end = latest_slot - seg;
    let path = segment_archive_path(archive_start, archive_end);

    if std::path::Path::new(&path).exists() {
        return;
    }

    {
        let mut in_progress = archiving_in_progress.lock().await;
        if !in_progress.insert(path.clone()) {
            return;
        }
    }

    let blocks: Vec<Block> = {
        let s = state.read().await;
        (archive_start..=archive_end)
            .filter_map(|slot| s.blocks.get(&slot).cloned())
            .collect()
    };

    if blocks.len() != seg as usize {
        println!("[ARCHIVE] Incomplete segment {}-{}: expected {}, got {} blocks",
            archive_start, archive_end, seg, blocks.len());
        let mut in_progress = archiving_in_progress.lock().await;
        in_progress.remove(&path);
        return;
    }

    println!("[ARCHIVE] Starting archive for segment {}-{}", archive_start, archive_end);

    let path_for_blocking = path.clone();
    let result = tokio::task::spawn_blocking(move || {
        archive_segment_to_disk(blocks, genesis_hash, archive_start, seg, path_for_blocking)
    }).await;

    match result {
        Ok(Ok(segment)) => {
            let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
            let manifest = build_publication_manifest(
                path.clone(),
                segment.metadata,
                "arweave".to_string(),
                now,
            );
            match write_publication_manifest(&manifest) {
                Ok(_) => println!("[PUBLISH] Manifest queued for segment {}-{}", archive_start, archive_end),
                Err(e) => println!("[PUBLISH] Manifest write failed for segment {}-{}: {}", archive_start, archive_end, e),
            }

            {
                let mut s = state.write().await;
                for slot in archive_start..=archive_end {
                    s.blocks.remove(&slot);
                }
            }
            println!("[ARCHIVE] Segment {}-{} written, verified, and pruned",
                archive_start, archive_end);
        }
        Ok(Err(e)) => {
            println!("[ARCHIVE] Segment {}-{} failed: {}", archive_start, archive_end, e);
        }
        Err(e) => {
            println!("[ARCHIVE] Blocking task join error for segment {}-{}: {}",
                archive_start, archive_end, e);
        }
    }

    let mut in_progress = archiving_in_progress.lock().await;
    in_progress.remove(&path);
}

async fn run_publisher_loop() {
    let mut tick = interval(Duration::from_secs(300));
    loop {
        tick.tick().await;

        let client = match ArweaveClient::from_env() {
            Ok(c) => c,
            Err(e) => {
                println!("[PUBLISH] Arweave client unavailable — skipping: {}", e);
                continue;
            }
        };

        let entries = match std::fs::read_dir(PUBLISH_QUEUE_DIR) {
            Ok(e) => e,
            Err(_) => continue,
        };

        for entry in entries.flatten() {
            let path = entry.path();
            let filename = match path.file_name().and_then(|n| n.to_str()) {
                Some(f) => f.to_string(),
                None => continue,
            };

            if !filename.ends_with(".manifest.json") {
                continue;
            }

            let stem = filename.trim_end_matches(".manifest.json");
            let parts: Vec<&str> = stem.split('_').collect();
            if parts.len() != 3 || parts[0] != "segment" {
                continue;
            }

            let segment_start: u64 = match parts[1].parse() {
                Ok(v) => v,
                Err(_) => continue,
            };
            let segment_end: u64 = match parts[2].parse() {
                Ok(v) => v,
                Err(_) => continue,
            };

            let receipt_path = format!("{}/segment_{}_{}.receipt.json", PUBLISH_RECEIPTS_DIR, segment_start, segment_end);
            if std::path::Path::new(&receipt_path).exists() {
                match read_publication_receipt(segment_start, segment_end) {
                    Ok(receipt) => match receipt.status {
                        PublicationStatus::Submitted | PublicationStatus::DeferredChunkingRequired => continue,
                        _ => {}
                    },
                    Err(e) => {
                        println!("[PUBLISH] Failed to read receipt for segment {}-{}: {} — retrying",
                            segment_start, segment_end, e);
                    }
                }
            }

            let manifest = match read_publication_manifest(segment_start, segment_end) {
                Ok(m) => m,
                Err(e) => {
                    println!("[PUBLISH] Failed to read manifest {}: {}", filename, e);
                    continue;
                }
            };

            println!("[PUBLISH] Processing manifest for segment {}-{}", segment_start, segment_end);
            let receipt = client.upload_manifest(&manifest).await;

            if let Err(e) = write_publication_receipt(&receipt) {
                println!("[PUBLISH] Failed to write receipt for segment {}-{}: {}", segment_start, segment_end, e);
            }
        }
    }
}

fn normalize_rpc_addr(rpc_addr: &str, peer_transport_addr: &str) -> String {
    if rpc_addr.starts_with("0.0.0.0:") {
        let port = rpc_addr.split(':').last().unwrap_or("3000");
        let peer_ip = peer_transport_addr.split(':').next().unwrap_or(peer_transport_addr);
        format!("{}:{}", peer_ip, port)
    } else {
        rpc_addr.to_string()
    }
}

async fn perform_startup_sync(
    state: Arc<RwLock<ChainState>>,
    peer_manager: Arc<Mutex<PeerManager>>,
    production_ready: Arc<AtomicBool>,
) {
    let rpc_addrs = {
        let pm = peer_manager.lock().await;
        pm.get_connected_peer_rpc_addrs()
    };

    if rpc_addrs.is_empty() {
        println!("[SYNC] No peer RPC addresses available — skipping catch-up");
        production_ready.store(true, Ordering::SeqCst);
        return;
    }

    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(10))
        .build()
        .unwrap_or_else(|_| reqwest::Client::new());

    let mut best_peer_slot = 0u64;
    let mut best_peer_rpc = String::new();

    for rpc_addr in &rpc_addrs {
        let url = format!("http://{}/head", rpc_addr);
        match client.get(&url).send().await {
            Ok(resp) => {
                if let Ok(text) = resp.text().await {
                    if let Ok(value) = serde_json::from_str::<serde_json::Value>(&text) {
                        if let Some(slot) = value["latest_slot"].as_u64() {
                            if slot > best_peer_slot {
                                best_peer_slot = slot;
                                best_peer_rpc = rpc_addr.clone();
                            }
                        }
                    }
                }
            }
            Err(e) => println!("[SYNC] Failed to query head from {}: {}", rpc_addr, e),
        }
    }

    let local_slot = {
        let s = state.read().await;
        s.latest_slot
    };

    if local_slot >= best_peer_slot || best_peer_rpc.is_empty() {
        println!("[SYNC] No catch-up needed (local={}, peer={})", local_slot, best_peer_slot);
        production_ready.store(true, Ordering::SeqCst);
        return;
    }

    println!("[SYNC] Catching up from slot {} to {}", local_slot + 1, best_peer_slot);

    let mut sync_ok = true;
    for slot in (local_slot + 1)..=best_peer_slot {
        let url = format!("http://{}/block/{}", best_peer_rpc, slot);
        match client.get(&url).send().await {
            Ok(resp) => {
                match resp.text().await {
                    Ok(text) => {
                        match serde_json::from_str::<Option<Block>>(&text) {
                            Ok(Some(block)) => {
                                let mut s = state.write().await;
                                if !s.add_block(block) {
                                    println!("[SYNC] Failed to apply block at slot {} — stopping", slot);
                                    sync_ok = false;
                                    break;
                                }
                                if slot % 100 == 0 {
                                    println!("[SYNC] Applied through slot {}", slot);
                                }
                            }
                            Ok(None) => {
                                println!("[SYNC] Peer has no block at slot {} — stopping", slot);
                                sync_ok = false;
                                break;
                            }
                            Err(e) => {
                                println!("[SYNC] Failed to deserialize block at slot {}: {} — stopping", slot, e);
                                sync_ok = false;
                                break;
                            }
                        }
                    }
                    Err(e) => {
                        println!("[SYNC] Failed to read response at slot {}: {} — stopping", slot, e);
                        sync_ok = false;
                        break;
                    }
                }
            }
            Err(e) => {
                println!("[SYNC] Failed to fetch block at slot {}: {} — stopping", slot, e);
                sync_ok = false;
                break;
            }
        }
    }

    if sync_ok {
        let final_slot = {
            let s = state.read().await;
            s.latest_slot
        };
        println!("[SYNC] Catch-up complete at slot {}", final_slot);
        production_ready.store(true, Ordering::SeqCst);
    } else {
        eprintln!("[SYNC] Partial sync failure — node will not produce to protect chain integrity");
        std::process::exit(1);
    }
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
    let my_rpc_addr = rpc_addr.clone();

    let my_genesis = if config.genesis_timestamp == 0 {
        let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
        println!("First validator - setting genesis to current time: {}", now);
        now
    } else {
        println!("Using genesis from config: {}", config.genesis_timestamp);
        config.genesis_timestamp
    };

    let genesis_hash = compute_genesis_hash(
        my_genesis,
        &config.genesis,
        &config.validators,
    );

    let validator_set: HashMap<String, u64> = config.validators.clone();
    let validator_count = validator_set.len();
    let solo_node = config.bootstrap_nodes.is_empty();

    let production_ready = Arc::new(AtomicBool::new(solo_node));
    let sync_triggered = Arc::new(AtomicBool::new(solo_node));

    if solo_node {
        println!("[STARTUP] Solo node — production enabled immediately");
    } else {
        println!("[STARTUP] Waiting for validator quorum ({} validators required)", validator_count);
    }

    let genesis_timestamp = Arc::new(Mutex::new(my_genesis));
    let genesis_ms = my_genesis * 1000;

    let state = Arc::new(RwLock::new(ChainState::new()));
    let consensus = Arc::new(RwLock::new(Consensus::new()));
    let peer_manager = Arc::new(Mutex::new(PeerManager::new(config.bootstrap_nodes.clone())));
    let mempool = Arc::new(Mutex::new(Mempool::new()));
    let metrics = Metrics::new();
    let archiving_in_progress: Arc<Mutex<HashSet<String>>> = Arc::new(Mutex::new(HashSet::new()));

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

    tokio::spawn(async move {
        run_publisher_loop().await;
    });

    if !solo_node {
        let production_ready_timeout = Arc::clone(&production_ready);
        tokio::spawn(async move {
            tokio::time::sleep(Duration::from_secs(120)).await;
            if !production_ready_timeout.load(Ordering::SeqCst) {
                eprintln!("[STARTUP] Validator quorum not reached after 120 seconds. Exiting.");
                std::process::exit(1);
            }
        });
    }

    let peer_manager_clone = Arc::clone(&peer_manager);
    let my_addr_clone = my_addr.clone();
    let tx_clone = tx.clone();
    let tpi_tx_clone = tpi_tx.clone();
    let genesis_timestamp_connect = Arc::clone(&genesis_timestamp);
    let my_validator_id_connect = my_validator_id.clone();
    let my_rpc_addr_connect = my_rpc_addr.clone();
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
                    let my_id = my_validator_id_connect.clone();
                    let my_rpc = Some(my_rpc_addr_connect.clone());
                    tokio::spawn(async move {
                        network::connect_and_handle_peer(node, addr, tx, tpi_tx, pm, genesis_ts, Some(my_id), my_rpc).await;
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
                    let my_id = my_validator_id_connect.clone();
                    let my_rpc = Some(my_rpc_addr_connect.clone());
                    tokio::spawn(async move {
                        network::connect_and_handle_peer(peer, addr, tx, tpi_tx, pm, genesis_ts, Some(my_id), my_rpc).await;
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
    let production_ready_block = Arc::clone(&production_ready);

    let mut current_slot = 0u64;
    let mut slot_deadline = tokio::time::Instant::now();

    loop {
        tokio::select! {
            Some((msg, peer_addr)) = rx.recv() => {
                match msg {
                    NetworkMessage::Handshake { peer_addr: their_addr, known_peers, genesis_timestamp: their_genesis, validator_id, rpc_addr: their_rpc_addr } => {
                        let peer_id = generate_peer_id(&peer_addr);
                        let peer_id_short = if peer_id.len() > 12 { &peer_id[..12] } else { &peer_id };
                        println!("[{}] Handshake from {} ({} peers, genesis: {})",
                            timestamp(), peer_id_short, known_peers.len(), their_genesis);

                        if !their_addr.is_empty() && their_addr != peer_addr {
                            println!("[{}] Address note: transport={}, declared={}",
                                timestamp(), peer_addr, their_addr);
                        }

                        let our_genesis = *genesis_timestamp.lock().await;
                        if their_genesis > 0 && their_genesis != our_genesis {
                            println!("[{}] Genesis mismatch from {}: theirs={}, ours={}",
                                timestamp(), peer_id_short, their_genesis, our_genesis);
                        }

                        {
                            let mut pm = peer_manager.lock().await;

                            let canonical_addr = if !their_addr.is_empty() && their_addr != peer_addr {
                                pm.normalize_peer_address(&peer_addr, &their_addr);
                                their_addr.clone()
                            } else {
                                peer_addr.clone()
                            };

                            if let Some(ref rpc) = their_rpc_addr {
                                let normalized = normalize_rpc_addr(rpc, &peer_addr);
                                pm.bind_rpc_addr(&canonical_addr, normalized);
                            }

                            if let Some(vid) = &validator_id {
                                pm.bind_validator(&canonical_addr, vid.clone());
                                let connected_count = pm.connected_validator_count(&validator_set);
                                let self_count = if validator_set.contains_key(&my_validator_id) { 1 } else { 0 };
                                if connected_count + self_count >= validator_count
                                    && !sync_triggered.load(Ordering::SeqCst)
                                {
                                    sync_triggered.store(true, Ordering::SeqCst);
                                    println!("[{}] Validator quorum reached ({}/{}), starting catch-up sync",
                                        timestamp(), connected_count + self_count, validator_count);
                                    let production_ready_sync = Arc::clone(&production_ready);
                                    let state_sync = Arc::clone(&state_clone);
                                    let peer_manager_sync = Arc::clone(&peer_manager);
                                    tokio::spawn(async move {
                                        perform_startup_sync(state_sync, peer_manager_sync, production_ready_sync).await;
                                    });
                                }
                            }

                            for peer in known_peers {
                                if peer != my_addr {
                                    pm.add_peer(peer);
                                }
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

                            let latest_slot = s.latest_slot;
                            drop(s);

                            let archive_state = Arc::clone(&state_clone);
                            let archive_genesis_hash = genesis_hash.clone();
                            let archive_guard = Arc::clone(&archiving_in_progress);
                            tokio::spawn(async move {
                                maybe_archive_and_prune(archive_state, archive_genesis_hash, latest_slot, archive_guard).await;
                            });

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
                slot_deadline = tokio::time::Instant::now() + Duration::from_secs(10);

                if !production_ready_block.load(Ordering::SeqCst) {
                    continue;
                }

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
                let genesis_hash_spawn = genesis_hash.clone();
                let archive_guard_spawn = Arc::clone(&archiving_in_progress);

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

                            let latest_slot = s.latest_slot;
                            drop(s);

                            let archive_state = Arc::clone(&state_clone_spawn);
                            let archive_genesis_hash = genesis_hash_spawn.clone();
                            let archive_guard = Arc::clone(&archive_guard_spawn);
                            tokio::spawn(async move {
                                maybe_archive_and_prune(archive_state, archive_genesis_hash, latest_slot, archive_guard).await;
                            });

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
            }
        }
    }
}
