use std::collections::VecDeque;
use std::sync::{Arc, Mutex};
use std::time::{SystemTime, UNIX_EPOCH};

const MAX_BLOCK_HISTORY: usize = 100;
const MAX_TX_HISTORY: usize = 50;
const MAX_LOG_ENTRIES: usize = 500;

#[derive(Clone, serde::Serialize)]
pub struct BlockMetric {
    pub slot: u64,
    pub hash: String,
    pub producer: String,
    pub tx_count: usize,
    pub time_ms: u64,
    pub timestamp: u64,
}

#[derive(Clone, serde::Serialize)]
pub struct TxMetric {
    pub from: String,
    pub to: String,
    pub amount: u64,
    pub hash: String,
    pub timestamp: u64,
}

#[derive(Clone, serde::Serialize)]
pub struct PeerMetric {
    pub peer_id: String,
    pub address: String,
    pub latency_ms: u64,
    pub connected_at: u64,
}

#[derive(Clone, serde::Serialize)]
pub struct LogEntry {
    pub timestamp: u64,
    pub level: String,
    pub message: String,
}

pub struct Metrics {
    blocks: VecDeque<BlockMetric>,
    transactions: VecDeque<TxMetric>,
    peers: Vec<PeerMetric>,
    logs: VecDeque<LogEntry>,
    
    start_time: u64,
    blocks_produced: u64,
    current_slot: u64,
    mempool_size: usize,
    
    total_block_time_ms: u64,
    block_count: u64,
    
    memory_mb: u64,
    cpu_percent: f64,
}

impl Metrics {
    pub fn new() -> Arc<Mutex<Self>> {
        Arc::new(Mutex::new(Metrics {
            blocks: VecDeque::with_capacity(MAX_BLOCK_HISTORY),
            transactions: VecDeque::with_capacity(MAX_TX_HISTORY),
            peers: Vec::new(),
            logs: VecDeque::with_capacity(MAX_LOG_ENTRIES),
            
            start_time: current_timestamp(),
            blocks_produced: 0,
            current_slot: 0,
            mempool_size: 0,
            
            total_block_time_ms: 0,
            block_count: 0,
            
            memory_mb: 0,
            cpu_percent: 0.0,
        }))
    }
    
    pub fn record_block(&mut self, block: BlockMetric) {
        self.blocks_produced += 1;
        self.current_slot = block.slot;
        self.total_block_time_ms += block.time_ms;
        self.block_count += 1;
        
        self.blocks.push_back(block);
        if self.blocks.len() > MAX_BLOCK_HISTORY {
            self.blocks.pop_front();
        }
    }
    
    pub fn record_transaction(&mut self, tx: TxMetric) {
        self.transactions.push_back(tx);
        if self.transactions.len() > MAX_TX_HISTORY {
            self.transactions.pop_front();
        }
    }
    
    pub fn add_peer(&mut self, peer: PeerMetric) {
        if !self.peers.iter().any(|p| p.peer_id == peer.peer_id) {
            self.peers.push(peer);
        }
    }
    
    pub fn remove_peer(&mut self, peer_id: &str) {
        self.peers.retain(|p| p.peer_id != peer_id);
    }
    
    pub fn update_peer_latency(&mut self, peer_id: &str, latency_ms: u64) {
        if let Some(peer) = self.peers.iter_mut().find(|p| p.peer_id == peer_id) {
            peer.latency_ms = latency_ms;
        }
    }
    
    pub fn add_log(&mut self, level: String, message: String) {
        self.logs.push_back(LogEntry {
            timestamp: current_timestamp(),
            level,
            message,
        });
        if self.logs.len() > MAX_LOG_ENTRIES {
            self.logs.pop_front();
        }
    }
    
    pub fn set_mempool_size(&mut self, size: usize) {
        self.mempool_size = size;
    }
    
    pub fn update_system_stats(&mut self, memory_mb: u64, cpu_percent: f64) {
        self.memory_mb = memory_mb;
        self.cpu_percent = cpu_percent;
    }
    
    pub fn get_status(&self) -> StatusResponse {
        let uptime = current_timestamp() - self.start_time;
        let avg_block_time = if self.block_count > 0 {
            self.total_block_time_ms / self.block_count
        } else {
            10000
        };
        
        let current_tps = if !self.blocks.is_empty() {
            let recent_block = &self.blocks[self.blocks.len() - 1];
            (recent_block.tx_count as f64 / (recent_block.time_ms as f64 / 1000.0)) as u64
        } else {
            0
        };
        
        let avg_tps = if self.block_count > 0 && self.total_block_time_ms > 0 {
            let total_txs: usize = self.blocks.iter().map(|b| b.tx_count).sum();
            ((total_txs as f64 / (self.total_block_time_ms as f64 / 1000.0)) * 1000.0) as u64
        } else {
            0
        };
        
        StatusResponse {
            current_slot: self.current_slot,
            blocks_produced: self.blocks_produced,
            mempool_size: self.mempool_size,
            connected_peers: self.peers.len(),
            uptime_seconds: uptime,
            avg_block_time,
            current_tps,
            avg_tps,
            memory_mb: self.memory_mb,
            cpu_percent: self.cpu_percent,
        }
    }
    
    pub fn get_blocks(&self) -> Vec<BlockMetric> {
        self.blocks.iter().cloned().collect()
    }
    
    pub fn get_peers(&self) -> Vec<PeerMetric> {
        self.peers.clone()
    }
    
    pub fn get_transactions(&self) -> Vec<TxMetric> {
        self.transactions.iter().cloned().collect()
    }
    
    pub fn get_logs(&self) -> Vec<LogEntry> {
        self.logs.iter().cloned().collect()
    }
}

#[derive(serde::Serialize)]
pub struct StatusResponse {
    pub current_slot: u64,
    pub blocks_produced: u64,
    pub mempool_size: usize,
    pub connected_peers: usize,
    pub uptime_seconds: u64,
    pub avg_block_time: u64,
    pub current_tps: u64,
    pub avg_tps: u64,
    pub memory_mb: u64,
    pub cpu_percent: f64,
}

fn current_timestamp() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
}
