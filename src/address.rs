use std::net::IpAddr;

pub fn canonicalize_peer_addr(advertised: &str, transport_ip: &str) -> String {
    let (host, port) = match split_host_port(advertised) {
        Some(pair) => pair,
        None => return advertised.to_string(),
    };

    let canonical_host = if is_unspecified_host(&host) {
        transport_ip.to_string()
    } else {
        normalize_host(&host)
    };

    format_host_port(&canonical_host, &port)
}

pub fn canonicalize_rpc_addr(advertised_rpc: &str, canonical_peer_addr: &str) -> String {
    let (rpc_host, rpc_port) = match split_host_port(advertised_rpc) {
        Some(pair) => pair,
        None => return advertised_rpc.to_string(),
    };

    if is_unspecified_host(&rpc_host) {
        let peer_host = split_host_port(canonical_peer_addr)
            .map(|(h, _)| h)
            .unwrap_or_else(|| canonical_peer_addr.to_string());
        format_host_port(&peer_host, &rpc_port)
    } else {
        format_host_port(&normalize_host(&rpc_host), &rpc_port)
    }
}

pub fn is_valid_peer_addr(addr: &str) -> bool {
    split_host_port(addr).is_some()
}

fn split_host_port(addr: &str) -> Option<(String, String)> {
    if addr.starts_with('[') {
        let bracket_end = addr.find(']')?;
        let host = addr[1..bracket_end].to_string();
        let rest = &addr[bracket_end + 1..];
        let port = rest.strip_prefix(':')?.to_string();
        if port.is_empty() {
            return None;
        }
        Some((host, port))
    } else {
        let colon_count = addr.chars().filter(|&c| c == ':').count();
        if colon_count == 1 {
            let mut parts = addr.splitn(2, ':');
            let host = parts.next()?.to_string();
            let port = parts.next()?.to_string();
            if host.is_empty() || port.is_empty() {
                return None;
            }
            Some((host, port))
        } else {
            None
        }
    }
}

fn is_unspecified_host(host: &str) -> bool {
    if let Ok(ip) = host.parse::<IpAddr>() {
        return ip.is_unspecified();
    }
    host.eq_ignore_ascii_case("localhost")
}

fn normalize_host(host: &str) -> String {
    if let Ok(ip) = host.parse::<IpAddr>() {
        return ip.to_string();
    }
    host.to_lowercase()
}

fn format_host_port(host: &str, port: &str) -> String {
    if let Ok(ip) = host.parse::<IpAddr>() {
        if let IpAddr::V6(_) = ip {
            return format!("[{}]:{}", host, port);
        }
    }
    format!("{}:{}", host, port)
}
