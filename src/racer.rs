use crate::types::*;
use sha2::{Sha256, Digest};

pub fn select_racer(
    slot: u64,
    validators: &[(String, u64)],
) -> String {
    if validators.is_empty() {
        return String::new();
    }
    
    let mut sorted: Vec<_> = validators.to_vec();
    sorted.sort_by(|a, b| a.1.cmp(&b.1));
    
    let pool_size = 10.min(sorted.len());
    let top_fastest: Vec<String> = sorted
        .into_iter()
        .take(pool_size)
        .map(|(id, _)| id)
        .collect();
    
    if top_fastest.is_empty() {
        return String::new();
    }
    
    let mut hasher = Sha256::new();
    hasher.update(slot.to_le_bytes());
    hasher.update(b"racer");
    let seed = hasher.finalize();
    
    let index = u64::from_le_bytes(seed[0..8].try_into().unwrap()) as usize % top_fastest.len();
    top_fastest[index].clone()
}

pub fn calculate_validator_speed(
    validator_id: &str,
    recent_blocks: &[Block],
) -> u64 {
    let validator_blocks: Vec<_> = recent_blocks
        .iter()
        .filter(|b| b.producer == validator_id)
        .collect();
    
    if validator_blocks.is_empty() {
        return u64::MAX;
    }
    
    let total_time: u64 = validator_blocks
        .windows(2)
        .map(|pair| {
            let time_diff = pair[1].timestamp.saturating_sub(pair[0].timestamp);
            time_diff.min(15000)
        })
        .sum();
    
    total_time / validator_blocks.len() as u64
}
