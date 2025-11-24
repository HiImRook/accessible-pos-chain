use crate::types::Block;
use serde::{Serialize, Deserialize};
use std::collections::HashMap;

/// Storage layer for persisting blockchain data using Sled embedded database
pub struct Storage {
    db: sled::Db,
}

#[derive(Serialize, Deserialize, Debug)]
struct AccountState {
    address: String,
    balance: u64,
}

impl Storage {
    /// Create or open a storage database at the given path
    pub fn new(path: &str) -> Result<Self, sled::Error> {
        let db = sled::open(path)?;
        Ok(Storage { db })
    }

    /// Store a block by its slot number
    pub fn store_block(&self, block: &Block) -> Result<(), Box<dyn std::error::Error>> {
        let tree = self.db.open_tree("blocks")?;
        let key = block.slot.to_be_bytes();
        let value = serde_json::to_vec(block)?;
        tree.insert(key, value)?;
        Ok(())
    }

    /// Retrieve a block by slot number
    pub fn get_block(&self, slot: u64) -> Result<Option<Block>, Box<dyn std::error::Error>> {
        let tree = self.db.open_tree("blocks")?;
        let key = slot.to_be_bytes();
        
        if let Some(data) = tree.get(key)? {
            let block: Block = serde_json::from_slice(&data)?;
            Ok(Some(block))
        } else {
            Ok(None)
        }
    }

    /// Store the latest slot number
    pub fn store_latest_slot(&self, slot: u64) -> Result<(), Box<dyn std::error::Error>> {
        let tree = self.db.open_tree("metadata")?;
        let value = slot.to_be_bytes();
        tree.insert("latest_slot", value.as_ref())?;
        Ok(())
    }

    /// Retrieve the latest slot number
    pub fn get_latest_slot(&self) -> Result<u64, Box<dyn std::error::Error>> {
        let tree = self.db.open_tree("metadata")?;
        
        if let Some(data) = tree.get("latest_slot")? {
            let bytes: [u8; 8] = data.as_ref().try_into()?;
            Ok(u64::from_be_bytes(bytes))
        } else {
            Ok(0)
        }
    }

    /// Store account balance
    pub fn store_account(&self, address: &str, balance: u64) -> Result<(), Box<dyn std::error::Error>> {
        let tree = self.db.open_tree("accounts")?;
        let state = AccountState {
            address: address.to_string(),
            balance,
        };
        let value = serde_json::to_vec(&state)?;
        tree.insert(address.as_bytes(), value)?;
        Ok(())
    }

    /// Retrieve account balance
    pub fn get_account_balance(&self, address: &str) -> Result<u64, Box<dyn std::error::Error>> {
        let tree = self.db.open_tree("accounts")?;
        
        if let Some(data) = tree.get(address.as_bytes())? {
            let state: AccountState = serde_json::from_slice(&data)?;
            Ok(state.balance)
        } else {
            Ok(0)
        }
    }

    /// Load all accounts into a HashMap
    pub fn load_all_accounts(&self) -> Result<HashMap<String, u64>, Box<dyn std::error::Error>> {
        let tree = self.db.open_tree("accounts")?;
        let mut accounts = HashMap::new();
        
        for item in tree.iter() {
            let (_key, value) = item?;
            let state: AccountState = serde_json::from_slice(&value)?;
            accounts.insert(state.address, state.balance);
        }
        
        Ok(accounts)
    }

    /// Load all blocks into a HashMap
    pub fn load_all_blocks(&self) -> Result<HashMap<u64, Block>, Box<dyn std::error::Error>> {
        let tree = self.db.open_tree("blocks")?;
        let mut blocks = HashMap::new();
        
        for item in tree.iter() {
            let (key, value) = item?;
            let slot_bytes: [u8; 8] = key.as_ref().try_into()?;
            let slot = u64::from_be_bytes(slot_bytes);
            let block: Block = serde_json::from_slice(&value)?;
            blocks.insert(slot, block);
        }
        
        Ok(blocks)
    }

    /// Flush all pending writes to disk
    pub fn flush(&self) -> Result<(), sled::Error> {
        self.db.flush()?;
        Ok(())
    }

    /// Clear all data (for testing purposes)
    #[allow(dead_code)]
    pub fn clear(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.db.clear()?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_storage_block_operations() {
        let storage = Storage::new("/tmp/test_storage_blocks").unwrap();
        storage.clear().unwrap();

        let block = Block {
            slot: 1,
            parent_hash: "genesis".to_string(),
            hash: "block_1".to_string(),
            producer: "validator_1".to_string(),
            timestamp: 10,
            transactions: vec![],
        };

        storage.store_block(&block).unwrap();
        let retrieved = storage.get_block(1).unwrap();
        assert!(retrieved.is_some());
        assert_eq!(retrieved.unwrap().slot, 1);
    }

    #[test]
    fn test_storage_account_operations() {
        let storage = Storage::new("/tmp/test_storage_accounts").unwrap();
        storage.clear().unwrap();

        storage.store_account("alice", 1000).unwrap();
        let balance = storage.get_account_balance("alice").unwrap();
        assert_eq!(balance, 1000);

        let balance_unknown = storage.get_account_balance("bob").unwrap();
        assert_eq!(balance_unknown, 0);
    }

    #[test]
    fn test_storage_latest_slot() {
        let storage = Storage::new("/tmp/test_storage_slot").unwrap();
        storage.clear().unwrap();

        storage.store_latest_slot(42).unwrap();
        let slot = storage.get_latest_slot().unwrap();
        assert_eq!(slot, 42);
    }
}
