use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use sha2::{Sha256, Digest};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Block {
    pub slot: u64,
    pub parent_hash: String,
    pub hash: String,
    pub producer: String,
    pub timestamp: u64,
    pub transactions: Vec<Transaction>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Transaction {
    pub from: String,
    pub from_pubkey: String,
    pub to: String,
    pub amount: u64,
    pub nonce: u64,
    pub fee: u64,
    pub signature: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum NetworkMessage {
    Handshake {
        peer_addr: String,
        known_peers: Vec<String>,
        genesis_timestamp: u64,
    },
    NewBlock(Block),
    Ping,
    TpiHash {
        slot: u64,
        validator_id: String,
        block_hash: String,
        signature: String,
    },
}

#[derive(Clone, Debug)]
pub struct PeerInfo {
    pub address: String,
    pub last_seen: u64,
    pub connected: bool,
}

pub struct ChainState {
    pub accounts: HashMap<String, u64>,
    pub total_supply: u64,
    pub nonces: HashMap<String, u64>,
    pub delegations: HashMap<String, String>,
    pub blocks: HashMap<u64, Block>,
    pub latest_slot: u64,
}

impl ChainState {
    pub fn new() -> Self {
        ChainState {
            accounts: HashMap::new(),
            total_supply: 0,
            nonces: HashMap::new(),
            delegations: HashMap::new(),
            blocks: HashMap::new(),
            latest_slot: 0,
        }
    }

    pub fn add_block(&mut self, block: Block) -> bool {
        if self.blocks.contains_key(&block.slot) {
            return false;
        }

        for tx in &block.transactions {
            use crate::crypto::verify_transaction;
            if !verify_transaction(
                &tx.from_pubkey,
                &tx.from,
                &tx.to,
                tx.amount,
                tx.nonce,
                tx.fee,
                &tx.signature
            ) {
                println!("Invalid signature for tx from {}", tx.from);
                return false;
            }

            let expected_nonce = self.nonces.get(&tx.from).copied().unwrap_or(0);
            if tx.nonce != expected_nonce {
                println!("Invalid nonce for {}: expected {}, got {}", tx.from, expected_nonce, tx.nonce);
                return false;
            }

            let from_balance = self.accounts.get(&tx.from).copied().unwrap_or(0);
            if from_balance < tx.amount + tx.fee {
                println!("Insufficient balance for {}: {} < {}", tx.from, from_balance, tx.amount + tx.fee);
                return false;
            }

            self.accounts.insert(tx.from.clone(), from_balance - tx.amount - tx.fee);
            *self.accounts.entry(tx.to.clone()).or_insert(0) += tx.amount;

            if let Some(validator) = self.delegations.get(&tx.from) {
                *self.accounts.entry(validator.clone()).or_insert(0) += tx.fee;
            } else {
                *self.accounts.entry(block.producer.clone()).or_insert(0) += tx.fee;
            }

            self.nonces.insert(tx.from.clone(), expected_nonce + 1);
        }

        if block.slot > self.latest_slot {
            self.latest_slot = block.slot;
        }
        self.blocks.insert(block.slot, block);
        true
    }

    pub fn get_balance(&self, address: &str) -> u64 {
        self.accounts.get(address).copied().unwrap_or(0)
    }

    pub fn current_epoch(&self) -> usize {
        const BLOCKS_PER_EPOCH: u64 = 3_150_000 * 7;
        (self.latest_slot / BLOCKS_PER_EPOCH) as usize
    }
}

pub struct Mempool {
    transactions: Vec<Transaction>,
}

impl Mempool {
    pub fn new() -> Self {
        Mempool {
            transactions: Vec::new(),
        }
    }

    pub fn add(&mut self, tx: Transaction) {
        self.transactions.push(tx);
    }

    pub fn add_transaction(&mut self, tx: Transaction) {
        self.add(tx);
    }

    pub fn get_pending(&mut self, max: usize) -> Vec<Transaction> {
        let count = max.min(self.transactions.len());
        let mut txs = self.transactions.drain(..count).collect::<Vec<_>>();
        txs.sort_by_cached_key(|tx| {
            let mut hasher = Sha256::new();
            hasher.update(tx.from.as_bytes());
            hasher.update(tx.to.as_bytes());
            hasher.update(tx.amount.to_le_bytes());
            hasher.finalize().to_vec()
        });
        txs
    }

    pub fn len(&self) -> usize {
        self.transactions.len()
    }
}

pub fn generate_peer_id(addr: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(addr.as_bytes());
    let result = hasher.finalize();
    format!("peer-{:x}", &result[..4].iter().fold(0u32, |acc, &b| (acc << 8) | b as u32))
}
