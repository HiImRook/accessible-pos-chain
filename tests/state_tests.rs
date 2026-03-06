use pos_chain::types::{Block, ChainState, Transaction};
use pos_chain::crypto::{generate_keypair, sign_transaction, keypair_to_address};

fn build_block(slot: u64, producer: &str, transactions: Vec<Transaction>) -> Block {
    Block {
        slot,
        parent_hash: "genesis".to_string(),
        hash: format!("hash_{}", slot),
        producer: producer.to_string(),
        timestamp: 0,
        transactions,
    }
}

fn build_signed_tx(keypair: &pos_chain::crypto::KeyPair, to: &str, amount: u64, nonce: u64, fee: u64) -> Transaction {
    let from = keypair_to_address(keypair);
    let signature = sign_transaction(keypair, &from, to, amount, nonce, fee);
    Transaction {
        from: from.clone(),
        from_pubkey: hex::encode(keypair.verifying_key.to_bytes()),
        to: to.to_string(),
        amount,
        nonce,
        fee,
        signature,
    }
}

const EPOCH_0_BLOCK_REWARD: u64 = 80_816_326;

#[test]
fn test_duplicate_block_rejected() {
    let mut state = ChainState::new();
    let block = build_block(0, "validator1", vec![]);
    assert!(state.add_block(block.clone()));
    assert!(!state.add_block(block));
}

#[test]
fn test_insufficient_balance_rejected() {
    let mut state = ChainState::new();
    let keypair = generate_keypair();
    let tx = build_signed_tx(&keypair, "bob", 100, 0, 10);
    let block = build_block(0, "validator1", vec![tx]);
    assert!(!state.add_block(block));
}

#[test]
fn test_invalid_nonce_rejected() {
    let mut state = ChainState::new();
    let keypair = generate_keypair();
    let alice = keypair_to_address(&keypair);
    state.accounts.insert(alice.clone(), 1000);
    let tx = build_signed_tx(&keypair, "bob", 100, 5, 10);
    let block = build_block(0, "validator1", vec![tx]);
    assert!(!state.add_block(block));
}

#[test]
fn test_balance_updates_correctly() {
    let mut state = ChainState::new();
    let keypair = generate_keypair();
    let alice = keypair_to_address(&keypair);
    state.accounts.insert(alice.clone(), 1000);
    let tx = build_signed_tx(&keypair, "bob", 100, 0, 10);
    let block = build_block(0, "validator1", vec![tx]);
    assert!(state.add_block(block));
    assert_eq!(state.get_balance(&alice), 890);
    assert_eq!(state.get_balance("bob"), 100);
    assert_eq!(state.get_balance("validator1"), 10 + EPOCH_0_BLOCK_REWARD);
}

#[test]
fn test_nonce_increments_after_transaction() {
    let mut state = ChainState::new();
    let keypair = generate_keypair();
    let alice = keypair_to_address(&keypair);
    state.accounts.insert(alice.clone(), 10_000);
    let tx1 = build_signed_tx(&keypair, "bob", 100, 0, 10);
    let block1 = build_block(0, "validator1", vec![tx1]);
    assert!(state.add_block(block1));
    let tx2 = build_signed_tx(&keypair, "bob", 100, 1, 10);
    let block2 = build_block(1, "validator1", vec![tx2]);
    assert!(state.add_block(block2));
    assert_eq!(state.nonces.get(&alice), Some(&2));
}
