use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use sha2::{Sha256, Digest};
use crate::tokenomics::{calculate_epoch_rewards, TOTAL_SUPPLY};

const MAX_MEMPOOL_SIZE: usize = 10_000;

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

            *self.accounts.entry(block.producer.clone()).or_insert(0) += tx.fee;

            self.nonces.insert(tx.from.clone(), expected_nonce + 1);
        }

        if !self.mint_block_reward(&block) {
            println!("Warning: Block reward minting failed (supply cap reached)");
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

    pub fn mint_block_reward(&mut self, block: &Block) -> bool {
        const BLOCKS_PER_EPOCH: u64 = 3_150_000 * 7;
        let epoch = (block.slot / BLOCKS_PER_EPOCH) as usize;
        let rewards = calculate_epoch_rewards(epoch);

        if self.total_supply + rewards.block_reward > TOTAL_SUPPLY {
            println!("Cannot mint: would exceed supply cap");
            return false;
        }

        *self.accounts.entry(block.producer.clone()).or_insert(0) += rewards.block_reward;
        self.total_supply += rewards.block_reward;

        true
    }
}

pub struct Mempool {
    transactions: Vec<Transaction>,
    seen_hashes: HashSet<String>,
}

impl Mempool {
    pub fn new() -> Self {
        Mempool {
            transactions: Vec::new(),
            seen_hashes: HashSet::new(),
        }
    }

    pub fn add(&mut self, tx: Transaction) -> bool {
        if self.transactions.len() >= MAX_MEMPOOL_SIZE {
            return false;
        }
        let tx_hash = compute_tx_hash(&tx);
        if self.seen_hashes.contains(&tx_hash) {
            return false;
        }
        self.seen_hashes.insert(tx_hash);
        self.transactions.push(tx);
        true
    }

    pub fn add_transaction(&mut self, tx: Transaction) {
        self.add(tx);
    }

    pub fn get_pending(&mut self, max: usize) -> Vec<Transaction> {
        let count = max.min(self.transactions.len());
        let mut txs = self.transactions.drain(..count).collect::<Vec<_>>();

        for tx in &txs {
            let tx_hash = compute_tx_hash(tx);
            self.seen_hashes.remove(&tx_hash);
        }

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

fn compute_tx_hash(tx: &Transaction) -> String {
    let mut hasher = Sha256::new();
    hasher.update(tx.from.as_bytes());
    hasher.update(tx.to.as_bytes());
    hasher.update(tx.amount.to_le_bytes());
    hasher.update(tx.nonce.to_le_bytes());
    hasher.update(tx.fee.to_le_bytes());
    hasher.update(tx.signature.as_bytes());
    format!("{:x}", hasher.finalize())
}

pub fn generate_peer_id(addr: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(addr.as_bytes());
    let result = hasher.finalize();
    format!("peer-{:x}", &result[..4].iter().fold(0u32, |acc, &b| (acc << 8) | b as u32))
}
