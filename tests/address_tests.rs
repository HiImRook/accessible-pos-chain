use pos_chain::address::{canonicalize_peer_addr, canonicalize_rpc_addr};

#[test]
fn test_wildcard_ipv4_replaced_with_transport_ip() {
    assert_eq!(canonicalize_peer_addr("0.0.0.0:8000", "1.2.3.4"), "1.2.3.4:8000");
}

#[test]
fn test_localhost_replaced_with_transport_ip() {
    assert_eq!(canonicalize_peer_addr("localhost:8000", "1.2.3.4"), "1.2.3.4:8000");
}

#[test]
fn test_localhost_case_insensitive_replaced() {
    assert_eq!(canonicalize_peer_addr("LOCALHOST:8000", "1.2.3.4"), "1.2.3.4:8000");
}

#[test]
fn test_loopback_preserved() {
    assert_eq!(canonicalize_peer_addr("127.0.0.1:8000", "1.2.3.4"), "127.0.0.1:8000");
}

#[test]
fn test_hostname_lowercased() {
    assert_eq!(canonicalize_peer_addr("EXAMPLE.COM:8000", "1.2.3.4"), "example.com:8000");
}

#[test]
fn test_normal_ipv4_preserved() {
    assert_eq!(canonicalize_peer_addr("1.2.3.4:8000", "9.9.9.9"), "1.2.3.4:8000");
}

#[test]
fn test_wildcard_ipv6_replaced_with_transport_ip() {
    assert_eq!(canonicalize_peer_addr("[::]:8000", "2001:db8::1"), "[2001:db8::1]:8000");
}

#[test]
fn test_bracketed_ipv6_preserved_and_normalized() {
    assert_eq!(canonicalize_peer_addr("[2001:DB8::1]:8000", "9.9.9.9"), "[2001:db8::1]:8000");
}

#[test]
fn test_malformed_no_port_returned_unchanged() {
    assert_eq!(canonicalize_peer_addr("notanaddress", "1.2.3.4"), "notanaddress");
}

#[test]
fn test_malformed_ambiguous_ipv6_returned_unchanged() {
    assert_eq!(canonicalize_peer_addr("2001:db8::1", "1.2.3.4"), "2001:db8::1");
}

#[test]
fn test_rpc_wildcard_substitutes_peer_host() {
    assert_eq!(canonicalize_rpc_addr("0.0.0.0:9000", "1.2.3.4:8000"), "1.2.3.4:9000");
}

#[test]
fn test_rpc_localhost_substitutes_peer_host() {
    assert_eq!(canonicalize_rpc_addr("localhost:9000", "1.2.3.4:8000"), "1.2.3.4:9000");
}

#[test]
fn test_rpc_localhost_case_insensitive_substitutes_peer_host() {
    assert_eq!(canonicalize_rpc_addr("LOCALHOST:9000", "1.2.3.4:8000"), "1.2.3.4:9000");
}

#[test]
fn test_rpc_wildcard_ipv6_substitutes_peer_host() {
    assert_eq!(canonicalize_rpc_addr("[::]:9000", "[2001:db8::1]:8000"), "[2001:db8::1]:9000");
}

#[test]
fn test_rpc_hostname_lowercased() {
    assert_eq!(canonicalize_rpc_addr("RPC.EXAMPLE.COM:9000", "1.2.3.4:8000"), "rpc.example.com:9000");
}

#[test]
fn test_rpc_normal_address_preserved() {
    assert_eq!(canonicalize_rpc_addr("5.6.7.8:9000", "1.2.3.4:8000"), "5.6.7.8:9000");
}

#[test]
fn test_rpc_malformed_no_port_returned_unchanged() {
    assert_eq!(canonicalize_rpc_addr("notanaddress", "1.2.3.4:8000"), "notanaddress");
}

#[test]
fn test_rpc_wildcard_substitutes_hostname_peer_host() {
    assert_eq!(canonicalize_rpc_addr("0.0.0.0:9000", "example.com:8000"), "example.com:9000");
}
