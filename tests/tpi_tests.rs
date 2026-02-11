use pos_chain::tpi::{check_tpi_consensus, TpiHashMessage, TpiConsensus};

fn create_tpi_message(slot: u64, validator_id: &str, block_hash: &str) -> TpiHashMessage {
    TpiHashMessage {
        slot,
        validator_id: validator_id.to_string(),
        block_hash: block_hash.to_string(),
        signature: vec![],
    }
}

#[test]
fn test_tpi_perfect_consensus() {
    let responses = vec![
        create_tpi_message(1, "val1", "hash123"),
        create_tpi_message(1, "val2", "hash123"),
        create_tpi_message(1, "val3", "hash123"),
    ];
    
    match check_tpi_consensus(responses) {
        TpiConsensus::Perfect(hash) => assert_eq!(hash, "hash123"),
        _ => panic!("Expected perfect consensus"),
    }
}

#[test]
fn test_tpi_two_of_three_consensus() {
    let responses = vec![
        create_tpi_message(1, "val1", "hash123"),
        create_tpi_message(1, "val2", "hash123"),
        create_tpi_message(1, "val3", "hash456"),
    ];
    
    match check_tpi_consensus(responses) {
        TpiConsensus::TwoOfThree(hash, outlier) => {
            assert_eq!(hash, "hash123");
            assert_eq!(outlier, "val3");
        }
        _ => panic!("Expected two-of-three consensus"),
    }
}

#[test]
fn test_tpi_two_of_two_consensus() {
    let responses = vec![
        create_tpi_message(1, "val1", "hash123"),
        create_tpi_message(1, "val2", "hash123"),
    ];
    
    match check_tpi_consensus(responses) {
        TpiConsensus::TwoOfTwo(hash, _missing) => {
            assert_eq!(hash, "hash123");
        }
        _ => panic!("Expected two-of-two consensus"),
    }
}

#[test]
fn test_tpi_no_consensus() {
    let responses = vec![
        create_tpi_message(1, "val1", "hash123"),
        create_tpi_message(1, "val2", "hash456"),
        create_tpi_message(1, "val3", "hash789"),
    ];
    
    match check_tpi_consensus(responses) {
        TpiConsensus::NoConsensus => {},
        _ => panic!("Expected no consensus"),
    }
}

#[test]
fn test_tpi_insufficient_data() {
    let responses = vec![
        create_tpi_message(1, "val1", "hash123"),
    ];
    
    match check_tpi_consensus(responses) {
        TpiConsensus::InsufficientData => {},
        _ => panic!("Expected insufficient data"),
    }
}

#[test]
fn test_tpi_two_validators_no_consensus() {
    let responses = vec![
        create_tpi_message(1, "val1", "hash123"),
        create_tpi_message(1, "val2", "hash456"),
    ];
    
    match check_tpi_consensus(responses) {
        TpiConsensus::NoConsensus => {},
        _ => panic!("Expected no consensus for conflicting two validators"),
    }
}
