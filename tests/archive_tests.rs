use pos_chain::archive::{
    build_archive_segment, compute_segment_checksum, verify_archive_segment,
    write_archive_segment, read_archive_segment, load_verified_archive_segment,
    blocks_per_segment,
};
use pos_chain::types::Block;

fn sample_block(slot: u64) -> Block {
    Block {
        slot,
        parent_hash: format!("parent_{}", slot),
        hash: format!("hash_{}", slot),
        producer: "validator_test".to_string(),
        timestamp: slot * 10,
        transactions: vec![],
    }
}

fn temp_test_path(name: &str) -> String {
    let nanos = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    format!("./{}_{}.json", name, nanos)
}

#[test]
fn test_blocks_per_segment_is_2160() {
    assert_eq!(blocks_per_segment(), 2_160);
}

#[test]
fn test_build_archive_segment_rejects_empty() {
    let result = build_archive_segment(vec![], "genesis_test", "");
    assert!(result.is_none());
}

#[test]
fn test_build_archive_segment_sorts_blocks() {
    let blocks = vec![sample_block(3), sample_block(1), sample_block(2)];
    let segment = build_archive_segment(blocks, "genesis_test", "").unwrap();
    assert_eq!(segment.metadata.segment_start_slot, 1);
    assert_eq!(segment.metadata.segment_end_slot, 3);
    assert_eq!(segment.blocks[0].slot, 1);
    assert_eq!(segment.blocks[2].slot, 3);
}

#[test]
fn test_verify_archive_segment_detects_checksum_mismatch() {
    let blocks = vec![sample_block(1), sample_block(2)];
    let mut segment = build_archive_segment(blocks, "genesis_test", "").unwrap();
    segment.metadata.payload_checksum = "tampered".to_string();
    assert!(!verify_archive_segment(&segment));
}

#[test]
fn test_verify_archive_segment_detects_block_count_mismatch() {
    let blocks = vec![sample_block(1), sample_block(2)];
    let mut segment = build_archive_segment(blocks, "genesis_test", "").unwrap();
    segment.metadata.block_count = 99;
    assert!(!verify_archive_segment(&segment));
}

#[test]
fn test_verify_archive_segment_detects_version_mismatch() {
    let blocks = vec![sample_block(1)];
    let mut segment = build_archive_segment(blocks, "genesis_test", "").unwrap();
    segment.metadata.archive_version = 99;
    assert!(!verify_archive_segment(&segment));
}

#[test]
fn test_verify_archive_segment_accepts_valid_segment() {
    let blocks = vec![sample_block(1), sample_block(2), sample_block(3)];
    let segment = build_archive_segment(blocks, "genesis_test", "").unwrap();
    assert!(verify_archive_segment(&segment));
}

#[test]
fn test_write_and_read_round_trip() {
    let blocks = vec![sample_block(1), sample_block(2)];
    let segment = build_archive_segment(blocks, "genesis_test", "").unwrap();
    let path = temp_test_path("test_archive_roundtrip");

    write_archive_segment(&segment, &path).unwrap();
    let loaded = read_archive_segment(&path).unwrap();

    assert_eq!(loaded.metadata.payload_checksum, segment.metadata.payload_checksum);
    assert_eq!(loaded.blocks.len(), 2);
    assert!(verify_archive_segment(&loaded));

    std::fs::remove_file(&path).unwrap();
}

#[test]
fn test_load_verified_archive_segment_round_trip() {
    let blocks = vec![sample_block(10), sample_block(11)];
    let segment = build_archive_segment(blocks, "genesis_test", "").unwrap();
    let path = temp_test_path("test_archive_verified_roundtrip");

    write_archive_segment(&segment, &path).unwrap();
    let loaded = load_verified_archive_segment(&path).unwrap();

    assert_eq!(loaded.metadata.segment_start_slot, 10);
    assert_eq!(loaded.metadata.segment_end_slot, 11);

    std::fs::remove_file(&path).unwrap();
}

#[test]
fn test_compute_segment_checksum_is_deterministic() {
    let blocks_a = vec![sample_block(1), sample_block(2)];
    let blocks_b = vec![sample_block(1), sample_block(2)];
    assert_eq!(compute_segment_checksum(&blocks_a), compute_segment_checksum(&blocks_b));
}

#[test]
fn test_compute_segment_checksum_differs_on_change() {
    let blocks_a = vec![sample_block(1), sample_block(2)];
    let blocks_b = vec![sample_block(1), sample_block(3)];
    assert_ne!(compute_segment_checksum(&blocks_a), compute_segment_checksum(&blocks_b));
}
