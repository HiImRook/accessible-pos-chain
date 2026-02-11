use crate::types::*;
use crate::tpi::*;
use crate::racer::*;
use crate::peer_manager::PeerManager;
use std::sync::Arc;
use tokio::time::{timeout, Duration, sleep};
use tokio::sync::{mpsc, Mutex, RwLock};
use std::collections::HashMap;

const MAX_BLOCK_WAIT_ATTEMPTS: usize = 80;
const BLOCK_POLL_INTERVAL_MS: u64 = 100;

pub async fn produce_block_with_tpi(
    slot: u64,
    my_validator_id: String,
    all_validator_ids: Vec<String>,
    validator_merit_scores: HashMap<String, u64>,
    state: Arc<RwLock<ChainState>>,
    mempool: Arc<Mutex<Mempool>>,
    tpi_rx: Arc<Mutex<mpsc::Receiver<TpiHashMessage>>>,
    tpi_tx: mpsc::Sender<TpiHashMessage>,
    peer_manager: Arc<Mutex<PeerManager>>,
    genesis_ms: u64,
) -> Option<Block> {
    let tpi_start = tokio::time::Instant::now();

    let tpi_group = select_tpi_validators(slot, &all_validator_ids);

    if tpi_group.is_empty() {
        return None;
    }

    println!("[TPI] Slot {}: Selected validators: {:?}", slot, tpi_group);

    let am_i_in_tpi = tpi_group.contains(&my_validator_id);

    if am_i_in_tpi {
        let block = create_block(slot, &my_validator_id, state.clone(), mempool.clone(), genesis_ms).await;
        let my_hash = compute_block_hash(&block);

        println!("[TPI] Slot {}: {} computed hash {} at T+0ms",
            slot, &my_validator_id[..12.min(my_validator_id.len())], &my_hash[..8]);

        let my_tpi_msg = TpiHashMessage {
            slot,
            validator_id: my_validator_id.clone(),
            block_hash: my_hash.clone(),
            signature: vec![],
        };

        broadcast_tpi_hash(my_tpi_msg.clone(), tpi_tx.clone(), peer_manager.clone()).await;

        let mut received_hashes = vec![my_tpi_msg];

        loop {
            let elapsed = tpi_start.elapsed();
            if elapsed >= Duration::from_millis(6000) {
                break;
            }

            let remaining = Duration::from_millis(6000).saturating_sub(elapsed);

            match tokio::time::timeout(remaining, async {
                let mut rx = tpi_rx.lock().await;
                rx.recv().await
            }).await {
                Ok(Some(msg)) if msg.slot == slot => {
                    println!("[TPI] Slot {}: Received hash from {} at T+{}ms",
                        slot, &msg.validator_id[..12.min(msg.validator_id.len())], elapsed.as_millis());
                    received_hashes.push(msg);

                    if received_hashes.len() >= 3 {
                        println!("[TPI] Slot {}: All 3 hashes received early", slot);
                        break;
                    }
                }
                _ => break,
            }
        }

        let consensus = check_tpi_consensus(received_hashes);

        match consensus {
            TpiConsensus::Perfect(hash) => {
                println!("[TPI] Slot {}: Perfect consensus (3/3) on hash {}", slot, &hash[..8]);
                let tpi_with_merit: Vec<(String, u64)> = tpi_group
                    .iter()
                    .map(|id| {
                        let merit = validator_merit_scores.get(id).copied().unwrap_or(0);
                        (id.clone(), merit)
                    })
                    .collect();

                let broadcaster = select_broadcaster_by_merit(&tpi_with_merit);

                if broadcaster == my_validator_id {
                    println!("[TPI] Slot {}: Broadcasting block (highest merit)", slot);
                    return Some(block);
                }
            }
            TpiConsensus::TwoOfThree(hash, _) => {
                println!("[TPI] Slot {}: Two-of-three consensus on hash {}", slot, &hash[..8]);
                let tpi_with_merit: Vec<(String, u64)> = tpi_group
                    .iter()
                    .map(|id| {
                        let merit = validator_merit_scores.get(id).copied().unwrap_or(0);
                        (id.clone(), merit)
                    })
                    .collect();

                let broadcaster = select_broadcaster_by_merit(&tpi_with_merit);

                if broadcaster == my_validator_id {
                    println!("[TPI] Slot {}: Broadcasting block (highest merit)", slot);
                    return Some(block);
                }
            }
            TpiConsensus::TwoOfTwo(hash, _) => {
                println!("[TPI] Slot {}: Two-of-two consensus on hash {}", slot, &hash[..8]);
                let tpi_with_merit: Vec<(String, u64)> = tpi_group
                    .iter()
                    .map(|id| {
                        let merit = validator_merit_scores.get(id).copied().unwrap_or(0);
                        (id.clone(), merit)
                    })
                    .collect();

                let broadcaster = select_broadcaster_by_merit(&tpi_with_merit);

                if broadcaster == my_validator_id {
                    println!("[TPI] Slot {}: Broadcasting block (highest merit)", slot);
                    return Some(block);
                }
            }
            _ => {
                println!("[TPI] Slot {}: Consensus failed - no matching hashes", slot);
            }
        }
    }

    println!("[TPI] Slot {}: TPI failed, waiting for block or racer", slot);
    match timeout(Duration::from_millis(8000), wait_for_block(slot, state.clone())).await {
        Ok(Some(block)) => {
            println!("[TPI] Slot {}: Received block from network", slot);
            return Some(block);
        }
        _ => {}
    }

    println!("[RACER] Slot {}: Checking racer eligibility", slot);
    let validators_with_speed: Vec<(String, u64)> = all_validator_ids
        .iter()
        .map(|id| {
            let speed = validator_merit_scores.get(id).copied().unwrap_or(u64::MAX);
            (id.clone(), speed)
        })
        .collect();

    let racer = select_racer(slot, &validators_with_speed);

    if racer == my_validator_id {
        let block = create_block(slot, &my_validator_id, state, mempool, genesis_ms).await;
        println!("[RACER] Slot {}: Racer activated, producing block", slot);
        return Some(block);
    }

    None
}

async fn broadcast_tpi_hash(
    msg: TpiHashMessage,
    _tpi_tx: mpsc::Sender<TpiHashMessage>,
    peer_manager: Arc<Mutex<PeerManager>>,
) {
    let peers = {
        let pm = peer_manager.lock().await;
        pm.get_connected_peers()
    };

    let network_msg = NetworkMessage::TpiHash {
        slot: msg.slot,
        validator_id: msg.validator_id,
        block_hash: msg.block_hash,
        signature: String::from_utf8_lossy(&msg.signature).to_string(),
    };

    for peer in peers {
        if let Ok(mut stream) = tokio::net::TcpStream::connect(&peer).await {
            use tokio::io::AsyncWriteExt;
            if let Ok(data) = serde_json::to_vec(&network_msg) {
                let len = (data.len() as u32).to_be_bytes();
                let _ = stream.write_all(&len).await;
                let _ = stream.write_all(&data).await;
            }
        }
    }
}

async fn create_block(
    slot: u64,
    producer: &str,
    state: Arc<RwLock<ChainState>>,
    mempool: Arc<Mutex<Mempool>>,
    genesis_ms: u64,
) -> Block {
    let transactions = {
        let mut mp = mempool.lock().await;
        mp.get_pending(100)
    };

    let parent_hash = if slot > 0 {
        let s = state.read().await;
        s.blocks.get(&(slot - 1))
            .map(|b| b.hash.clone())
            .unwrap_or_else(|| "genesis".to_string())
    } else {
        "genesis".to_string()
    };

    let timestamp = genesis_ms + (slot * 10_000);

    let mut block = Block {
        slot,
        parent_hash,
        hash: String::new(),
        producer: producer.to_string(),
        timestamp,
        transactions,
    };

    block.hash = compute_block_hash(&block);
    block
}

async fn wait_for_block(slot: u64, state: Arc<RwLock<ChainState>>) -> Option<Block> {
    for _ in 0..MAX_BLOCK_WAIT_ATTEMPTS {
        sleep(Duration::from_millis(BLOCK_POLL_INTERVAL_MS)).await;

        let s = state.read().await;
        if let Some(block) = s.blocks.get(&slot) {
            return Some(block.clone());
        }
    }
    None
}
