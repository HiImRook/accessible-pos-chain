use crate::types::*;
use crate::tpi::*;
use crate::racer::*;
use std::sync::{Arc, Mutex};
use tokio::time::{timeout, Duration, sleep};
use std::collections::HashMap;

pub async fn produce_block_with_tpi(
    slot: u64,
    my_validator_id: String,
    all_validator_ids: Vec<String>,
    validator_merit_scores: HashMap<String, u64>,
    state: Arc<Mutex<ChainState>>,
    mempool: Arc<Mutex<Mempool>>,
) -> Option<Block> {
    
    let tpi_group = select_tpi_validators(slot, &all_validator_ids);
    
    if tpi_group.is_empty() {
        return None;
    }
    
    let am_i_in_tpi = tpi_group.contains(&my_validator_id);
    
    if am_i_in_tpi {
        if let Some(block) = tpi_produce_block(
            slot,
            &my_validator_id,
            &tpi_group,
            &validator_merit_scores,
            state.clone(),
            mempool.clone(),
        ).await {
            return Some(block);
        }
    }
    
    match timeout(Duration::from_secs(8), wait_for_block(slot, state.clone())).await {
        Ok(Some(block)) => return Some(block),
        _ => {}
    }
    
    let validators_with_speed: Vec<(String, u64)> = all_validator_ids
        .iter()
        .map(|id| {
            let speed = validator_merit_scores.get(id).copied().unwrap_or(u64::MAX);
            (id.clone(), speed)
        })
        .collect();
    
    let racer = select_racer(slot, &validators_with_speed);
    
    if racer == my_validator_id {
        let block = create_block(slot, &my_validator_id, state, mempool);
        println!("[RACER] Slot {}: Speed save by {}", slot, my_validator_id);
        return Some(block);
    }
    
    None
}

async fn tpi_produce_block(
    slot: u64,
    my_id: &str,
    tpi_group: &[String],
    merit_scores: &HashMap<String, u64>,
    _state: Arc<Mutex<ChainState>>,
    mempool: Arc<Mutex<Mempool>>,
) -> Option<Block> {
    
    let block = create_block(slot, my_id, _state.clone(), mempool.clone());
    let my_hash = compute_block_hash(&block);
    
    println!("[TPI] Slot {}: {} computed hash {}", slot, my_id, &my_hash[..8]);
    
    sleep(Duration::from_secs(1)).await;
    
    let tpi_with_merit: Vec<(String, u64)> = tpi_group
        .iter()
        .map(|id| {
            let merit = merit_scores.get(id).copied().unwrap_or(0);
            (id.clone(), merit)
        })
        .collect();
    
    let broadcaster = select_broadcaster_by_merit(&tpi_with_merit);
    
    if broadcaster == my_id {
        println!("[TPI] Slot {}: {} broadcasting (highest merit)", slot, my_id);
        return Some(block);
    }
    
    None
}

fn create_block(
    slot: u64,
    producer: &str,
    _state: Arc<Mutex<ChainState>>,
    mempool: Arc<Mutex<Mempool>>,
) -> Block {
    let transactions = {
        let mut mp = mempool.lock().unwrap();
        mp.get_pending(100)
    };
    
    Block {
        slot,
        parent_hash: if slot > 0 {
            format!("block_{}", slot - 1)
        } else {
            "genesis".to_string()
        },
        hash: format!("block_{}", slot),
        producer: producer.to_string(),
        timestamp: slot * 10,
        transactions,
    }
}

async fn wait_for_block(slot: u64, state: Arc<Mutex<ChainState>>) -> Option<Block> {
    for _ in 0..50 {
        sleep(Duration::from_millis(100)).await;
        
        let s = state.lock().unwrap();
        if let Some(block) = s.blocks.get(&slot) {
            return Some(block.clone());
        }
    }
    None
}
