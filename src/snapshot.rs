use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};
use crate::types::ChainState;

const SNAPSHOT_VERSION: u32 = 1;
const RECENT_BLOCK_TIP_COUNT: usize = 10;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct RecentBlockRef {
    pub slot: u64,
    pub hash: String,
    pub parent_hash: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SnapshotPayload {
    pub accounts: HashMap<String, u64>,
    pub nonces: HashMap<String, u64>,
    pub total_supply: u64,
    pub latest_slot: u64,
    pub delegations: HashMap<String, String>,
    pub recent_block_tips: Vec<RecentBlockRef>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SnapshotMetadata {
    pub snapshot_version: u32,
    pub written_at: u64,
    pub latest_slot: u64,
    pub latest_block_hash: String,
    pub genesis_hash: String,
    pub payload_checksum: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Snapshot {
    pub metadata: SnapshotMetadata,
    pub payload: SnapshotPayload,
}

pub fn snapshot_exists(path: &str) -> bool {
    std::path::Path::new(path).exists()
}

pub fn compute_genesis_hash(
    genesis_timestamp: u64,
    genesis_accounts: &HashMap<String, u64>,
    validators: &HashMap<String, u64>,
) -> String {
    let mut hasher = Sha256::new();
    hasher.update(genesis_timestamp.to_le_bytes());

    let mut sorted_accounts: Vec<(&String, &u64)> = genesis_accounts.iter().collect();
    sorted_accounts.sort_by_key(|(address, _)| address.as_str());
    for (address, balance) in sorted_accounts {
        hasher.update(address.as_bytes());
        hasher.update(balance.to_le_bytes());
    }

    let mut sorted_validators: Vec<(&String, &u64)> = validators.iter().collect();
    sorted_validators.sort_by_key(|(address, _)| address.as_str());
    for (address, stake) in sorted_validators {
        hasher.update(address.as_bytes());
        hasher.update(stake.to_le_bytes());
    }

    format!("{:x}", hasher.finalize())
}

pub fn compute_payload_checksum(payload: &SnapshotPayload) -> String {
    let mut hasher = Sha256::new();

    let mut sorted_accounts: Vec<(&String, &u64)> = payload.accounts.iter().collect();
    sorted_accounts.sort_by_key(|(address, _)| address.as_str());
    for (address, balance) in sorted_accounts {
        hasher.update(address.as_bytes());
        hasher.update(balance.to_le_bytes());
    }

    let mut sorted_nonces: Vec<(&String, &u64)> = payload.nonces.iter().collect();
    sorted_nonces.sort_by_key(|(address, _)| address.as_str());
    for (address, nonce) in sorted_nonces {
        hasher.update(address.as_bytes());
        hasher.update(nonce.to_le_bytes());
    }

    hasher.update(payload.total_supply.to_le_bytes());
    hasher.update(payload.latest_slot.to_le_bytes());

    let mut sorted_delegations: Vec<(&String, &String)> = payload.delegations.iter().collect();
    sorted_delegations.sort_by_key(|(delegator, _)| delegator.as_str());
    for (delegator, validator) in sorted_delegations {
        hasher.update(delegator.as_bytes());
        hasher.update(validator.as_bytes());
    }

    let mut sorted_tips = payload.recent_block_tips.clone();
    sorted_tips.sort_by_key(|tip| tip.slot);
    for tip in sorted_tips {
        hasher.update(tip.slot.to_le_bytes());
        hasher.update(tip.hash.as_bytes());
        hasher.update(tip.parent_hash.as_bytes());
    }

    format!("{:x}", hasher.finalize())
}

fn collect_recent_block_tips(state: &ChainState) -> Vec<RecentBlockRef> {
    let mut slots: Vec<u64> = state.blocks.keys().cloned().collect();
    slots.sort();

    slots.into_iter()
        .rev()
        .take(RECENT_BLOCK_TIP_COUNT)
        .collect::<Vec<_>>()
        .into_iter()
        .rev()
        .filter_map(|slot| {
            state.blocks.get(&slot).map(|block| RecentBlockRef {
                slot: block.slot,
                hash: block.hash.clone(),
                parent_hash: block.parent_hash.clone(),
            })
        })
        .collect()
}

fn find_latest_block_hash(state: &ChainState) -> String {
    if let Some(block) = state.blocks.get(&state.latest_slot) {
        return block.hash.clone();
    }

    let mut slots: Vec<u64> = state.blocks.keys().cloned().collect();
    slots.sort();
    slots.into_iter()
        .last()
        .and_then(|slot| state.blocks.get(&slot))
        .map(|block| block.hash.clone())
        .unwrap_or_default()
}

pub fn build_snapshot(state: &ChainState, genesis_hash: &str) -> Snapshot {
    let payload = SnapshotPayload {
        accounts: state.accounts.clone(),
        nonces: state.nonces.clone(),
        total_supply: state.total_supply,
        latest_slot: state.latest_slot,
        delegations: state.delegations.clone(),
        recent_block_tips: collect_recent_block_tips(state),
    };

    let metadata = SnapshotMetadata {
        snapshot_version: SNAPSHOT_VERSION,
        written_at: SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs(),
        latest_slot: payload.latest_slot,
        latest_block_hash: find_latest_block_hash(state),
        genesis_hash: genesis_hash.to_string(),
        payload_checksum: compute_payload_checksum(&payload),
    };

    Snapshot { metadata, payload }
}

pub fn write_snapshot(snapshot: &Snapshot, path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let temp_path = format!("{}.tmp", path);
    let json = serde_json::to_string_pretty(snapshot)?;
    std::fs::write(&temp_path, &json)?;
    std::fs::rename(&temp_path, path)?;
    Ok(())
}

pub fn read_snapshot(path: &str) -> Result<Snapshot, Box<dyn std::error::Error>> {
    let json = std::fs::read_to_string(path)?;
    let snapshot: Snapshot = serde_json::from_str(&json)?;
    Ok(snapshot)
}

pub fn verify_snapshot(snapshot: &Snapshot) -> bool {
    if snapshot.metadata.snapshot_version != SNAPSHOT_VERSION {
        println!(
            "[SNAPSHOT] Version mismatch: expected {}, got {}",
            SNAPSHOT_VERSION,
            snapshot.metadata.snapshot_version
        );
        return false;
    }

    let expected_checksum = compute_payload_checksum(&snapshot.payload);
    if expected_checksum != snapshot.metadata.payload_checksum {
        println!("[SNAPSHOT] Checksum mismatch: snapshot is corrupt");
        return false;
    }

    true
}

pub fn load_verified_snapshot(path: &str) -> Result<Snapshot, Box<dyn std::error::Error>> {
    let snapshot = read_snapshot(path)?;
    if !verify_snapshot(&snapshot) {
        return Err("snapshot verification failed".into());
    }
    Ok(snapshot)
}

pub fn restore_state(state: &mut ChainState, snapshot: &Snapshot) {
    state.accounts = snapshot.payload.accounts.clone();
    state.nonces = snapshot.payload.nonces.clone();
    state.total_supply = snapshot.payload.total_supply;
    state.latest_slot = snapshot.payload.latest_slot;
    state.delegations = snapshot.payload.delegations.clone();
}

pub fn restored_tip_matches(snapshot: &Snapshot, latest_slot: u64, latest_block_hash: &str) -> bool {
    snapshot.metadata.latest_slot == latest_slot
        && snapshot.metadata.latest_block_hash == latest_block_hash
}
