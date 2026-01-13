use crate::types::PeerInfo;
use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};

const PEER_TIMEOUT_SECS: u64 = 120;

pub struct PeerManager {
    peers: HashMap<String, PeerInfo>,
    bootstrap_nodes: Vec<String>,
}

impl PeerManager {
    pub fn new(bootstrap_nodes: Vec<String>) -> Self {
        PeerManager {
            peers: HashMap::new(),
            bootstrap_nodes,
        }
    }

    pub fn add_peer(&mut self, address: String) {
        if !self.peers.contains_key(&address) {
            self.peers.insert(address.clone(), PeerInfo {
                address,
                last_seen: current_timestamp(),
                connected: false,
            });
        }
    }

    pub fn mark_connected(&mut self, address: &str) {
        if let Some(peer) = self.peers.get_mut(address) {
            peer.connected = true;
            peer.last_seen = current_timestamp();
        }
    }

    pub fn mark_disconnected(&mut self, address: &str) {
        if let Some(peer) = self.peers.get_mut(address) {
            peer.connected = false;
        }
    }

    pub fn update_seen(&mut self, address: &str) {
        if let Some(peer) = self.peers.get_mut(address) {
            peer.last_seen = current_timestamp();
        }
    }

    pub fn get_connected_peers(&self) -> Vec<String> {
        self.peers
            .values()
            .filter(|p| p.connected)
            .map(|p| p.address.clone())
            .collect()
    }

    pub fn get_all_known_peers(&self) -> Vec<String> {
        self.peers.keys().cloned().collect()
    }

    pub fn get_bootstrap_nodes(&self) -> Vec<String> {
        self.bootstrap_nodes.clone()
    }

    pub fn get_peers_to_connect(&self) -> Vec<String> {
        self.peers
            .values()
            .filter(|p| !p.connected)
            .map(|p| p.address.clone())
            .collect()
    }

    pub fn cleanup_stale_peers(&mut self) {
        let now = current_timestamp();
        self.peers.retain(|_, peer| {
            now.saturating_sub(peer.last_seen) < PEER_TIMEOUT_SECS
        });
    }
}

fn current_timestamp() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
}
