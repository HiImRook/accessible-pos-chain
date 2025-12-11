use serde::{Serialize, Deserialize};
use std::collections::HashMap;

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Block {
    pub slot: u64,
    pub parent_hash: String,
    pub hash: String,
    pub producer: String,
    pub timestamp: u64,
    pub transactions: Vec<Transaction>,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Transaction {
    pub from: String,
    pub from_pubkey: String,
    pub to: String,
    pub amount: u64,
    pub signature: String,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub enum NetworkMessage {
    Handshake { peer_addr: String, known_peers: Vec<String> },
    PeerExchange { peers: Vec<String> },
    NewBlock(Block),
    RequestBlocks { from_slot: u64 },
    BlockResponse { blocks: Vec<Block> },
    Ping,
    Pong,
}

pub struct ChainState {
    pub blocks: HashMap<u64, Block>,
    pub accounts: HashMap<String, u64>,
    pub latest_slot: u64,
}

impl ChainState {
    pub fn new() -> Self {
        ChainState {
            blocks: HashMap::new(),
            accounts: HashMap::new(),
            latest_slot: 0,
        }
    }

    pub fn add_block(&mut self, block: Block) -> bool {
        use crate::crypto::verify_transaction;
        
        if block.slot != self.latest_slot + 1 {
            return false;
        }

        for tx in &block.transactions {
            if !verify_transaction(&tx.from_pubkey, &tx.from, &tx.to, tx.amount, &tx.signature) {
                println!("Invalid signature for transaction from {}", tx.from);
                return false;
            }
            
            let from_balance = self.accounts.get(&tx.from).copied().unwrap_or(0);
            if from_balance < tx.amount {
                return false;
            }
            self.accounts.insert(tx.from.clone(), from_balance - tx.amount);
            let to_balance = self.accounts.get(&tx.to).copied().unwrap_or(0);
            self.accounts.insert(tx.to.clone(), to_balance + tx.amount);
        }

        self.blocks.insert(block.slot, block);
        self.latest_slot += 1;
        true
    }

    pub fn get_balance(&self, address: &str) -> u64 {
        self.accounts.get(address).copied().unwrap_or(0)
    }
}

#[derive(Clone, Debug)]
pub struct PeerInfo {
    pub addr: String,
    pub last_seen: u64,
    pub connected: bool,
}

pub struct Mempool {
    pending: Vec<Transaction>,
}

impl Mempool {
    pub fn new() -> Self {
        Mempool {
            pending: Vec::new(),
        }
    }

    pub fn add_transaction(&mut self, tx: Transaction) {
        self.pending.push(tx);
    }

    pub fn get_pending(&mut self, max: usize) -> Vec<Transaction> {
        use rand::seq::SliceRandom;
        let mut rng = rand::thread_rng();
        self.pending.shuffle(&mut rng);
        let count = self.pending.len().min(max);
        self.pending.drain(..count).collect()
    }

    pub fn len(&self) -> usize {
        self.pending.len()
    }
}
