use crate::types::*;
use sha2::{Sha256, Digest};
use std::collections::HashMap;

pub const TPI_HASH_TIMEOUT_MS: u64 = 1000;
pub const TPI_GROUP_SIZE: usize = 3;

#[derive(Debug, Clone)]
pub struct TpiHashMessage {
    pub slot: u64,
    pub validator_id: String,
    pub block_hash: String,
    pub signature: Vec<u8>,
}

#[derive(Debug)]
pub enum TpiConsensus {
    Perfect(String),
    TwoOfThree(String, String),
    TwoOfTwo(String, String),
    NoConsensus,
    InsufficientData,
}

pub fn select_tpi_validators(slot: u64, validators: &[String]) -> Vec<String> {
    if validators.is_empty() {
        return Vec::new();
    }
    
    let mut hasher = Sha256::new();
    hasher.update(slot.to_le_bytes());
    let seed = hasher.finalize();
    
    let mut indices: Vec<usize> = (0..validators.len()).collect();
    indices.sort_by_key(|&i| {
        let mut h = Sha256::new();
        h.update(&seed);
        h.update(&validators[i].as_bytes());
        h.finalize()
    });
    
    let selection_size = TPI_GROUP_SIZE.min(validators.len());
    
    indices.into_iter()
        .take(selection_size)
        .map(|i| validators[i].clone())
        .collect()
}

pub fn compute_block_hash(block: &Block) -> String {
    let mut hasher = Sha256::new();
    hasher.update(block.slot.to_le_bytes());
    hasher.update(block.parent_hash.as_bytes());
    hasher.update(block.producer.as_bytes());
    hasher.update(block.timestamp.to_le_bytes());
    
    for tx in &block.transactions {
        hasher.update(tx.from.as_bytes());
        hasher.update(tx.from_pubkey.as_bytes());
        hasher.update(tx.to.as_bytes());
        hasher.update(tx.amount.to_le_bytes());
        hasher.update(tx.signature.as_bytes());
    }
    
    format!("{:x}", hasher.finalize())
}

pub fn check_tpi_consensus(responses: Vec<TpiHashMessage>) -> TpiConsensus {
    if responses.len() < 2 {
        return TpiConsensus::InsufficientData;
    }
    
    let mut hash_counts: HashMap<String, Vec<String>> = HashMap::new();
    for response in &responses {
        hash_counts
            .entry(response.block_hash.clone())
            .or_insert_with(Vec::new)
            .push(response.validator_id.clone());
    }
    
    if responses.len() == 3 {
        if hash_counts.len() == 1 {
            let hash = responses[0].block_hash.clone();
            return TpiConsensus::Perfect(hash);
        }
        
        for (hash, validators) in hash_counts {
            if validators.len() >= 2 {
                let outlier = responses
                    .iter()
                    .find(|r| r.block_hash != hash)
                    .map(|r| r.validator_id.clone())
                    .unwrap_or_default();
                return TpiConsensus::TwoOfThree(hash, outlier);
            }
        }
        
        return TpiConsensus::NoConsensus;
    }
    
    if responses.len() == 2 {
        if responses[0].block_hash == responses[1].block_hash {
            let missing = "missing_validator".to_string();
            return TpiConsensus::TwoOfTwo(responses[0].block_hash.clone(), missing);
        }
        return TpiConsensus::NoConsensus;
    }
    
    TpiConsensus::InsufficientData
}

pub fn select_broadcaster_by_merit(
    validators: &[(String, u64)],
) -> String {
    if validators.is_empty() {
        return String::new();
    }
    
    let mut sorted = validators.to_vec();
    sorted.sort_by(|a, b| b.1.cmp(&a.1));
    
    sorted[0].0.clone()
}
