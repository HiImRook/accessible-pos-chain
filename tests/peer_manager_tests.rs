use pos_chain::peer_manager::PeerManager;

#[test]
fn test_normalize_moves_connected_state_to_canonical() {
    let mut pm = PeerManager::new(vec![]);
    pm.add_peer("transport-hash".to_string(), "1.2.3.4:8000".to_string());
    pm.mark_connected("transport-hash");
    pm.normalize_peer_address("transport-hash", "canonical-hash");
    let connected = pm.get_connected_peers();
    assert!(connected.contains(&"canonical-hash".to_string()));
    assert!(!connected.contains(&"transport-hash".to_string()));
}

#[test]
fn test_normalize_inherits_rpc_when_canonical_lacks_one() {
    let mut pm = PeerManager::new(vec![]);
    pm.add_peer("transport-hash".to_string(), "1.2.3.4:8000".to_string());
    pm.bind_rpc_addr("transport-hash", "1.2.3.4:3000".to_string());
    pm.normalize_peer_address("transport-hash", "canonical-hash");
    let rpc_addrs = pm.get_connected_peer_rpc_addrs();
    assert!(rpc_addrs.contains(&"1.2.3.4:3000".to_string()));
}

#[test]
fn test_normalize_does_not_overwrite_existing_canonical_rpc() {
    let mut pm = PeerManager::new(vec![]);
    pm.add_peer("transport-hash".to_string(), "1.2.3.4:8000".to_string());
    pm.bind_rpc_addr("transport-hash", "1.2.3.4:3000".to_string());
    pm.add_peer("canonical-hash".to_string(), "1.2.3.4:8000".to_string());
    pm.mark_connected("canonical-hash");
    pm.bind_rpc_addr("canonical-hash", "1.2.3.4:9000".to_string());
    pm.normalize_peer_address("transport-hash", "canonical-hash");
    let rpc_addrs = pm.get_connected_peer_rpc_addrs();
    assert!(rpc_addrs.contains(&"1.2.3.4:9000".to_string()));
    assert!(!rpc_addrs.contains(&"1.2.3.4:3000".to_string()));
}

#[test]
fn test_normalize_moves_dial_target_to_canonical_hash() {
    let mut pm = PeerManager::new(vec![]);
    pm.add_peer("transport-hash".to_string(), "1.2.3.4:8000".to_string());
    pm.mark_connected("transport-hash");
    pm.normalize_peer_address("transport-hash", "canonical-hash");
    let targets = pm.get_connected_peer_dial_targets();
    let hashes: Vec<&str> = targets.iter().map(|(h, _)| h.as_str()).collect();
    let addrs: Vec<&str> = targets.iter().map(|(_, a)| a.as_str()).collect();
    assert!(hashes.contains(&"canonical-hash"));
    assert!(addrs.contains(&"1.2.3.4:8000"));
}

#[test]
fn test_normalize_removes_transport_hash_from_dial_target_lookup() {
    let mut pm = PeerManager::new(vec![]);
    pm.add_peer("transport-hash".to_string(), "1.2.3.4:8000".to_string());
    pm.mark_connected("transport-hash");
    pm.normalize_peer_address("transport-hash", "canonical-hash");
    let targets = pm.get_connected_peer_dial_targets();
    assert!(targets.iter().any(|(h, a)| h == "canonical-hash" && a == "1.2.3.4:8000"));
    assert!(!targets.iter().any(|(h, _)| h == "transport-hash"));
}

#[test]
fn test_bind_canonical_dial_target_overwrites_stale_target() {
    let mut pm = PeerManager::new(vec![]);
    pm.add_peer("peer-hash".to_string(), "0.0.0.0:8000".to_string());
    pm.mark_connected("peer-hash");
    pm.bind_canonical_dial_target("peer-hash", "1.2.3.4:8000".to_string());
    let targets = pm.get_connected_peer_dial_targets();
    let addrs: Vec<&str> = targets.iter().map(|(_, a)| a.as_str()).collect();
    assert!(addrs.contains(&"1.2.3.4:8000"));
    assert!(!addrs.contains(&"0.0.0.0:8000"));
}

#[test]
fn test_add_peer_does_not_overwrite_first_seen_identity() {
    let mut pm = PeerManager::new(vec![]);
    pm.add_peer("peer-hash".to_string(), "1.2.3.4:8000".to_string());
    pm.add_peer("peer-hash".to_string(), "5.6.7.8:8000".to_string());
    let known = pm.get_all_known_peers();
    assert!(known.contains(&"1.2.3.4:8000".to_string()));
    assert!(!known.contains(&"5.6.7.8:8000".to_string()));
}

#[test]
fn test_normalize_equal_hashes_is_noop() {
    let mut pm = PeerManager::new(vec![]);
    pm.add_peer("same-hash".to_string(), "1.2.3.4:8000".to_string());
    pm.mark_connected("same-hash");
    pm.normalize_peer_address("same-hash", "same-hash");
    let connected = pm.get_connected_peers();
    assert!(connected.contains(&"same-hash".to_string()));
}
