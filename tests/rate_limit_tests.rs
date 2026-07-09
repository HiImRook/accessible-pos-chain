use pos_chain::network::allow_inbound_connection;
use pos_chain::peer_manager::PeerManager;
use std::collections::HashMap;

#[test]
fn test_connection_limiter_allows_up_to_threshold() {
    let mut state: HashMap<String, Vec<u64>> = HashMap::new();
    for _ in 0..5 {
        assert!(allow_inbound_connection(&mut state, "1.2.3.4"));
    }
}

#[test]
fn test_connection_limiter_rejects_above_threshold() {
    let mut state: HashMap<String, Vec<u64>> = HashMap::new();
    for _ in 0..5 {
        assert!(allow_inbound_connection(&mut state, "1.2.3.4"));
    }
    assert!(!allow_inbound_connection(&mut state, "1.2.3.4"));
}

#[test]
fn test_connection_limiter_different_ips_are_independent() {
    let mut state: HashMap<String, Vec<u64>> = HashMap::new();
    for _ in 0..5 {
        assert!(allow_inbound_connection(&mut state, "1.2.3.4"));
    }
    assert!(!allow_inbound_connection(&mut state, "1.2.3.4"));
    assert!(allow_inbound_connection(&mut state, "5.6.7.8"));
}

#[test]
fn test_message_limiter_allows_up_to_threshold() {
    let mut pm = PeerManager::new(vec![]);
    pm.add_peer("peer-hash".to_string(), "1.2.3.4:8000".to_string());
    for _ in 0..100 {
        assert!(pm.record_inbound_message("peer-hash"));
    }
}

#[test]
fn test_message_limiter_rejects_above_threshold() {
    let mut pm = PeerManager::new(vec![]);
    pm.add_peer("peer-hash".to_string(), "1.2.3.4:8000".to_string());
    for _ in 0..100 {
        assert!(pm.record_inbound_message("peer-hash"));
    }
    assert!(!pm.record_inbound_message("peer-hash"));
}

#[test]
fn test_message_limiter_independent_per_peer() {
    let mut pm = PeerManager::new(vec![]);
    pm.add_peer("peer-a".to_string(), "1.2.3.4:8000".to_string());
    pm.add_peer("peer-b".to_string(), "5.6.7.8:8000".to_string());
    for _ in 0..100 {
        assert!(pm.record_inbound_message("peer-a"));
    }
    assert!(!pm.record_inbound_message("peer-a"));
    assert!(pm.record_inbound_message("peer-b"));
}

#[test]
fn test_normalization_preserves_message_rate_history() {
    let mut pm = PeerManager::new(vec![]);
    pm.add_peer("transport-hash".to_string(), "1.2.3.4:8000".to_string());
    for _ in 0..50 {
        assert!(pm.record_inbound_message("transport-hash"));
    }
    pm.normalize_peer_address("transport-hash", "canonical-hash");
    for _ in 0..50 {
        assert!(pm.record_inbound_message("canonical-hash"));
    }
    assert!(!pm.record_inbound_message("canonical-hash"));
}
