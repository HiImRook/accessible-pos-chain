use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::time::{SystemTime, UNIX_EPOCH};
use crate::types::Block;

const ARCHIVE_VERSION: u32 = 1;
const BLOCKS_PER_SEGMENT: u64 = 2_160;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ArchiveMetadata {
    pub archive_version: u32,
    pub genesis_hash: String,
    pub segment_start_slot: u64,
    pub segment_end_slot: u64,
    pub block_count: u64,
    pub first_block_hash: String,
    pub last_block_hash: String,
    pub previous_segment_hash: String,
    pub payload_checksum: String,
    pub written_at: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ArchiveSegment {
    pub metadata: ArchiveMetadata,
    pub blocks: Vec<Block>,
}

pub fn compute_segment_checksum(blocks: &[Block]) -> String {
    let mut hasher = Sha256::new();
    for block in blocks {
        hasher.update(block.slot.to_le_bytes());
        hasher.update(block.hash.as_bytes());
        hasher.update(block.parent_hash.as_bytes());
        hasher.update(block.producer.as_bytes());
        hasher.update(block.timestamp.to_le_bytes());
        for tx in &block.transactions {
            hasher.update(tx.from.as_bytes());
            hasher.update(tx.to.as_bytes());
            hasher.update(tx.amount.to_le_bytes());
            hasher.update(tx.nonce.to_le_bytes());
            hasher.update(tx.fee.to_le_bytes());
            hasher.update(tx.signature.as_bytes());
        }
    }
    format!("{:x}", hasher.finalize())
}

pub fn build_archive_segment(
    blocks: Vec<Block>,
    genesis_hash: &str,
    previous_segment_hash: &str,
) -> Option<ArchiveSegment> {
    if blocks.is_empty() {
        return None;
    }

    let mut sorted_blocks = blocks;
    sorted_blocks.sort_by_key(|b| b.slot);

    let segment_start_slot = sorted_blocks.first().unwrap().slot;
    let segment_end_slot = sorted_blocks.last().unwrap().slot;
    let first_block_hash = sorted_blocks.first().unwrap().hash.clone();
    let last_block_hash = sorted_blocks.last().unwrap().hash.clone();
    let block_count = sorted_blocks.len() as u64;
    let payload_checksum = compute_segment_checksum(&sorted_blocks);

    let metadata = ArchiveMetadata {
        archive_version: ARCHIVE_VERSION,
        genesis_hash: genesis_hash.to_string(),
        segment_start_slot,
        segment_end_slot,
        block_count,
        first_block_hash,
        last_block_hash,
        previous_segment_hash: previous_segment_hash.to_string(),
        payload_checksum,
        written_at: SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs(),
    };

    Some(ArchiveSegment {
        metadata,
        blocks: sorted_blocks,
    })
}

pub fn write_archive_segment(
    segment: &ArchiveSegment,
    path: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let temp_path = format!("{}.tmp", path);
    let json = serde_json::to_string_pretty(segment)?;
    std::fs::write(&temp_path, &json)?;
    std::fs::rename(&temp_path, path)?;
    Ok(())
}

pub fn read_archive_segment(path: &str) -> Result<ArchiveSegment, Box<dyn std::error::Error>> {
    let json = std::fs::read_to_string(path)?;
    let segment: ArchiveSegment = serde_json::from_str(&json)?;
    Ok(segment)
}

pub fn verify_archive_segment(segment: &ArchiveSegment) -> bool {
    if segment.metadata.archive_version != ARCHIVE_VERSION {
        println!(
            "[ARCHIVE] Version mismatch: expected {}, got {}",
            ARCHIVE_VERSION,
            segment.metadata.archive_version
        );
        return false;
    }

    let expected_checksum = compute_segment_checksum(&segment.blocks);
    if expected_checksum != segment.metadata.payload_checksum {
        println!("[ARCHIVE] Checksum mismatch: segment is corrupt");
        return false;
    }

    if segment.blocks.len() as u64 != segment.metadata.block_count {
        println!("[ARCHIVE] Block count mismatch");
        return false;
    }

    true
}

pub fn load_verified_archive_segment(
    path: &str,
) -> Result<ArchiveSegment, Box<dyn std::error::Error>> {
    let segment = read_archive_segment(path)?;
    if !verify_archive_segment(&segment) {
        return Err("archive segment verification failed".into());
    }
    Ok(segment)
}

pub fn segment_archive_path(start_slot: u64, end_slot: u64) -> String {
    format!("./archive_{}_{}.json", start_slot, end_slot)
}

pub fn blocks_per_segment() -> u64 {
    BLOCKS_PER_SEGMENT
}
