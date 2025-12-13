use crate::types::PeerInfo;
use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};

const PEER_TIMEOUT: u64 = 300;
const MAX_PEERS: usize = 50;

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

    pub fn add_peer(&mut self, addr: String) {
        use crate::types::generate_peer_id;
        
        if self.peers.len() >= MAX_PEERS {
            return;
        }

        if !self.peers.contains_key(&addr) {
            let peer_id = generate_peer_id(&addr);
            self.peers.insert(addr.clone(), PeerInfo {
                addr,
                peer_id,
                last_seen: current_timestamp(),
                connected: false,
            });
        }
    }

    pub fn mark_connected(&mut self, addr: &str) {
        if let Some(peer) = self.peers.get_mut(addr) {
            peer.connected = true;
            peer.last_seen = current_timestamp();
        }
    }

    pub fn mark_disconnected(&mut self, addr: &str) {
        if let Some(peer) = self.peers.get_mut(addr) {
            peer.connected = false;
        }
    }

    pub fn update_seen(&mut self, addr: &str) {
        if let Some(peer) = self.peers.get_mut(addr) {
            peer.last_seen = current_timestamp();
        }
    }

    pub fn get_peers_to_connect(&self) -> Vec<String> {
        use rand::seq::SliceRandom;
        let now = current_timestamp();
        let mut peers: Vec<String> = self.peers
            .values()
            .filter(|p| !p.connected && now - p.last_seen < PEER_TIMEOUT)
            .map(|p| p.addr.clone())
            .collect();
        let mut rng = rand::thread_rng();
        peers.shuffle(&mut rng);
        peers.into_iter().take(10).collect()
    }

    pub fn get_connected_peers(&self) -> Vec<String> {
        self.peers
            .values()
            .filter(|p| p.connected)
            .map(|p| p.addr.clone())
            .collect()
    }

    pub fn get_all_known_peers(&self) -> Vec<String> {
        self.peers.keys().cloned().collect()
    }

    pub fn cleanup_stale_peers(&mut self) {
        let now = current_timestamp();
        self.peers.retain(|_, peer| {
            peer.connected || now - peer.last_seen < PEER_TIMEOUT
        });
    }

    pub fn get_bootstrap_nodes(&self) -> Vec<String> {
        self.bootstrap_nodes.clone()
    }
}

fn current_timestamp() -> u64 {
    SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs()
}
