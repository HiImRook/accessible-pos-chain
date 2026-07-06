use crate::types::PeerInfo;
use std::collections::{HashMap, HashSet};
use std::time::{SystemTime, UNIX_EPOCH};

const PEER_TIMEOUT_SECS: u64 = 120;

pub struct PeerManager {
    peers: HashMap<String, PeerInfo>,
    dial_targets: HashMap<String, String>,
    bootstrap_nodes: Vec<String>,
}

impl PeerManager {
    pub fn new(bootstrap_nodes: Vec<String>) -> Self {
        PeerManager {
            peers: HashMap::new(),
            dial_targets: HashMap::new(),
            bootstrap_nodes,
        }
    }

    pub fn add_peer(&mut self, peer_hash: String, dial_addr: String) {
        self.dial_targets.entry(peer_hash.clone()).or_insert(dial_addr);
        if !self.peers.contains_key(&peer_hash) {
            self.peers.insert(peer_hash.clone(), PeerInfo {
                peer_hash: peer_hash,
                last_seen: current_timestamp(),
                connected: false,
                rpc_addr: None,
            });
        }
    }

    pub fn bind_canonical_dial_target(&mut self, peer_hash: &str, dial_addr: String) {
        self.dial_targets.insert(peer_hash.to_string(), dial_addr);
    }

    pub fn mark_connected(&mut self, peer_hash: &str) {
        if let Some(peer) = self.peers.get_mut(peer_hash) {
            peer.connected = true;
            peer.last_seen = current_timestamp();
        }
    }

    pub fn mark_disconnected(&mut self, peer_hash: &str) {
        if let Some(peer) = self.peers.get_mut(peer_hash) {
            peer.connected = false;
        }
    }

    pub fn update_seen(&mut self, peer_hash: &str) {
        if let Some(peer) = self.peers.get_mut(peer_hash) {
            peer.last_seen = current_timestamp();
        }
    }

    pub fn bind_rpc_addr(&mut self, peer_hash: &str, rpc_addr: String) {
        if let Some(peer) = self.peers.get_mut(peer_hash) {
            peer.rpc_addr = Some(rpc_addr);
        }
    }

    pub fn normalize_peer_address(&mut self, transport_hash: &str, canonical_hash: &str) {
        if transport_hash == canonical_hash {
            return;
        }

        let inherited_rpc_addr = self.peers
            .get(transport_hash)
            .and_then(|p| p.rpc_addr.clone());

        let inherited_dial = self.dial_targets.get(transport_hash).cloned();

        if let Some(existing) = self.peers.get_mut(canonical_hash) {
            existing.connected = true;
            existing.last_seen = current_timestamp();
            if existing.rpc_addr.is_none() {
                existing.rpc_addr = inherited_rpc_addr;
            }
        } else {
            self.peers.insert(canonical_hash.to_string(), PeerInfo {
                peer_hash: canonical_hash.to_string(),
                last_seen: current_timestamp(),
                connected: true,
                rpc_addr: inherited_rpc_addr,
            });
        }

        if let Some(dial) = inherited_dial {
            self.dial_targets.insert(canonical_hash.to_string(), dial);
        }

        self.peers.remove(transport_hash);
        self.dial_targets.remove(transport_hash);
    }

    pub fn get_connected_peer_rpc_addrs(&self) -> Vec<String> {
        let mut seen: HashSet<String> = HashSet::new();
        self.peers
            .values()
            .filter(|p| p.connected)
            .filter_map(|p| p.rpc_addr.clone())
            .filter(|addr| seen.insert(addr.clone()))
            .collect()
    }

    pub fn get_connected_peers(&self) -> Vec<String> {
        self.peers
            .values()
            .filter(|p| p.connected)
            .map(|p| p.peer_hash.clone())
            .collect()
    }

    pub fn get_connected_peer_dial_targets(&self) -> Vec<(String, String)> {
        self.peers
            .values()
            .filter(|p| p.connected)
            .filter_map(|p| {
                self.dial_targets
                    .get(&p.peer_hash)
                    .map(|dial| (p.peer_hash.clone(), dial.clone()))
            })
            .collect()
    }

    pub fn get_all_known_peers(&self) -> Vec<String> {
        let mut seen: HashSet<String> = HashSet::new();
        self.dial_targets
            .values()
            .filter(|addr| seen.insert((*addr).clone()))
            .cloned()
            .collect()
    }

    pub fn get_bootstrap_nodes(&self) -> Vec<String> {
        self.bootstrap_nodes.clone()
    }

    pub fn get_peers_to_connect(&self) -> Vec<String> {
        self.peers
            .values()
            .filter(|p| !p.connected)
            .filter_map(|p| self.dial_targets.get(&p.peer_hash).cloned())
            .collect()
    }

    pub fn cleanup_stale_peers(&mut self) {
        let now = current_timestamp();
        let stale: Vec<String> = self.peers
            .iter()
            .filter(|(_, peer)| now.saturating_sub(peer.last_seen) >= PEER_TIMEOUT_SECS)
            .map(|(k, _)| k.clone())
            .collect();

        for key in stale {
            self.peers.remove(&key);
            self.dial_targets.remove(&key);
        }
    }
}

fn current_timestamp() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
}
