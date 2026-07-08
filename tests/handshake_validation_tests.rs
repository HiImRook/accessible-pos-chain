use pos_chain::address::is_valid_peer_addr;
use pos_chain::crypto::peer_addr_hash;
use pos_chain::peer_manager::PeerManager;

#[test]
fn test_valid_ipv4_peer_accepted() {
    assert!(is_valid_peer_addr("1.2.3.4:8000"));
}

#[test]
fn test_valid_hostname_peer_accepted() {
    assert!(is_valid_peer_addr("example.com:8000"));
}

#[test]
fn test_valid_bracketed_ipv6_peer_accepted() {
    assert!(is_valid_peer_addr("[2001:db8::1]:8000"));
}

#[test]
fn test_invalid_no_port_rejected() {
    assert!(!is_valid_peer_addr("1.2.3.4"));
}

#[test]
fn test_invalid_ambiguous_ipv6_rejected() {
    assert!(!is_valid_peer_addr("2001:db8::1"));
}

#[test]
fn test_invalid_empty_string_rejected() {
    assert!(!is_valid_peer_addr(""));
}

#[test]
fn test_invalid_peer_hash_string_rejected() {
    assert!(!is_valid_peer_addr("peer-a1b2c3d4"));
}

#[test]
fn test_invalid_colon_only_rejected() {
    assert!(!is_valid_peer_addr(":"));
}

#[test]
fn test_wildcard_ipv4_is_parseable() {
    assert!(is_valid_peer_addr("0.0.0.0:8000"));
}

#[test]
fn test_wildcard_ipv6_bracketed_is_parseable() {
    assert!(is_valid_peer_addr("[::]:8000"));
}

#[test]
fn test_localhost_is_parseable() {
    assert!(is_valid_peer_addr("localhost:8000"));
}

#[test]
fn test_hostname_without_port_rejected() {
    assert!(!is_valid_peer_addr("rpc.example.com"));
}

#[test]
fn test_hostname_with_port_accepted() {
    assert!(is_valid_peer_addr("rpc.example.com:3000"));
}

#[test]
fn test_invalid_empty_host_rejected() {
    assert!(!is_valid_peer_addr(":8000"));
}

#[test]
fn test_invalid_empty_port_rejected() {
    assert!(!is_valid_peer_addr("1.2.3.4:"));
}

#[test]
fn test_invalid_empty_bracketed_host_rejected() {
    assert!(!is_valid_peer_addr("[]:8000"));
}

#[test]
fn test_invalid_bracketed_ipv6_no_port_rejected() {
    assert!(!is_valid_peer_addr("[2001:db8::1]"));
}

#[test]
fn test_invalid_unbracketed_ipv6_with_port_rejected() {
    assert!(!is_valid_peer_addr("2001:db8::1:8000"));
}

#[test]
fn test_invalid_their_addr_ignores_all_handshake_data() {
    let mut pm = PeerManager::new(vec![]);
    let genesis_hash = "test-genesis";
    let known_peers = vec!["5.6.7.8:8000".to_string()];
    let result = pm.apply_handshake_metadata(
        "provisional-hash",
        "notanaddress",
        &known_peers,
        Some("1.2.3.4:3000"),
        "myaddr:8000",
        genesis_hash,
    );
    assert!(!result);
    assert!(pm.get_peers_to_connect().is_empty());
    assert!(pm.get_connected_peer_rpc_addrs().is_empty());
    assert!(pm.get_all_known_peers().is_empty());
}

#[test]
fn test_invalid_gossiped_peer_is_skipped() {
    let mut pm = PeerManager::new(vec![]);
    let genesis_hash = "test-genesis";
    let their_addr = "1.2.3.4:8000";
    let declared_hash = peer_addr_hash(their_addr, genesis_hash);
    let known_peers = vec![
        "5.6.7.8:9000".to_string(),
        "notanaddress".to_string(),
        "9.9.9.9:8000".to_string(),
    ];
    pm.apply_handshake_metadata(
        &declared_hash,
        their_addr,
        &known_peers,
        None,
        "myaddr:8000",
        genesis_hash,
    );
    let known = pm.get_all_known_peers();
    assert!(known.contains(&"5.6.7.8:9000".to_string()));
    assert!(known.contains(&"9.9.9.9:8000".to_string()));
    assert!(!known.contains(&"notanaddress".to_string()));
}

#[test]
fn test_invalid_rpc_addr_is_not_bound() {
    let mut pm = PeerManager::new(vec![]);
    let genesis_hash = "test-genesis";
    let their_addr = "1.2.3.4:8000";
    let declared_hash = peer_addr_hash(their_addr, genesis_hash);
    pm.add_peer(declared_hash.clone(), their_addr.to_string());
    pm.mark_connected(&declared_hash);
    pm.apply_handshake_metadata(
        &declared_hash,
        their_addr,
        &[],
        Some("notanrpc"),
        "myaddr:8000",
        genesis_hash,
    );
    assert!(pm.get_connected_peer_rpc_addrs().is_empty());
}

#[test]
fn test_wildcard_rpc_is_normalized_and_bound() {
    let mut pm = PeerManager::new(vec![]);
    let genesis_hash = "test-genesis";
    let their_addr = "1.2.3.4:8000";
    let declared_hash = peer_addr_hash(their_addr, genesis_hash);
    pm.add_peer(declared_hash.clone(), their_addr.to_string());
    pm.mark_connected(&declared_hash);
    pm.apply_handshake_metadata(
        &declared_hash,
        their_addr,
        &[],
        Some("0.0.0.0:3000"),
        "myaddr:8000",
        genesis_hash,
    );
    let rpc_addrs = pm.get_connected_peer_rpc_addrs();
    assert!(rpc_addrs.contains(&"1.2.3.4:3000".to_string()));
}

#[test]
fn test_my_addr_excluded_from_gossip() {
    let mut pm = PeerManager::new(vec![]);
    let genesis_hash = "test-genesis";
    let their_addr = "1.2.3.4:8000";
    let declared_hash = peer_addr_hash(their_addr, genesis_hash);
    let known_peers = vec![
        "5.6.7.8:8000".to_string(),
        "myaddr:8000".to_string(),
    ];
    pm.apply_handshake_metadata(
        &declared_hash,
        their_addr,
        &known_peers,
        None,
        "myaddr:8000",
        genesis_hash,
    );
    let known = pm.get_all_known_peers();
    assert!(known.contains(&"5.6.7.8:8000".to_string()));
    assert!(!known.contains(&"myaddr:8000".to_string()));
}
