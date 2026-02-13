use pos_chain::types::{Mempool, Transaction};

fn create_test_transaction(from: &str, to: &str, amount: u64, nonce: u64, signature: &str) -> Transaction {
    Transaction {
        from: from.to_string(),
        from_pubkey: "test_pubkey".to_string(),
        to: to.to_string(),
        amount,
        nonce,
        fee: 1000,
        signature: signature.to_string(),
    }
}

#[test]
fn test_mempool_duplicate_detection() {
    let mut mempool = Mempool::new();
    let tx = create_test_transaction("alice", "bob", 100, 0, "sig1");
    
    assert!(mempool.add(tx.clone()));
    assert_eq!(mempool.len(), 1);
    
    assert!(!mempool.add(tx.clone()));
    assert_eq!(mempool.len(), 1);
}

#[test]
fn test_mempool_size_limit() {
    let mut mempool = Mempool::new();
    
    for i in 0..10_000 {
        let tx = create_test_transaction(
            &format!("addr{}", i),
            "bob",
            100,
            0,
            &format!("sig{}", i)
        );
        assert!(mempool.add(tx));
    }
    
    assert_eq!(mempool.len(), 10_000);
    
    let overflow_tx = create_test_transaction("overflow", "bob", 100, 0, "overflow_sig");
    assert!(!mempool.add(overflow_tx));
    assert_eq!(mempool.len(), 10_000);
}

#[test]
fn test_mempool_get_pending() {
    let mut mempool = Mempool::new();
    
    for i in 0..50 {
        let tx = create_test_transaction(
            &format!("addr{}", i),
            "bob",
            100,
            i,
            &format!("sig{}", i)
        );
        mempool.add(tx);
    }
    
    let pending = mempool.get_pending(20);
    assert_eq!(pending.len(), 20);
    assert_eq!(mempool.len(), 30);
}

#[test]
fn test_mempool_unique_transactions() {
    let mut mempool = Mempool::new();
    
    let tx1 = create_test_transaction("alice", "bob", 100, 0, "sig1");
    let tx2 = create_test_transaction("alice", "bob", 100, 1, "sig2");
    let tx3 = create_test_transaction("alice", "charlie", 100, 0, "sig3");
    
    assert!(mempool.add(tx1));
    assert!(mempool.add(tx2));
    assert!(mempool.add(tx3));
    assert_eq!(mempool.len(), 3);
}

#[test]
fn test_fee_priority_ordering() {
    let mut mempool = Mempool::new();
    
    let mut low_fee_tx = create_test_transaction("alice", "bob", 100, 0, "sig1");
    low_fee_tx.fee = 100;
    
    let mut high_fee_tx = create_test_transaction("charlie", "dave", 100, 0, "sig2");
    high_fee_tx.fee = 1000;
    
    let mut mid_fee_tx = create_test_transaction("eve", "frank", 100, 0, "sig3");
    mid_fee_tx.fee = 500;
    
    mempool.add(low_fee_tx);
    mempool.add(high_fee_tx);
    mempool.add(mid_fee_tx);
    
    let pending = mempool.get_pending(3);
    
    assert_eq!(pending[0].fee, 1000);
    assert_eq!(pending[1].fee, 500);
    assert_eq!(pending[2].fee, 100);
}

#[test]
fn test_same_fee_maintains_order() {
    let mut mempool = Mempool::new();
    
    for i in 0..5 {
        let tx = create_test_transaction(&format!("addr{}", i), "bob", 100, 0, &format!("sig{}", i));
        mempool.add(tx);
    }
    
    let pending = mempool.get_pending(5);
    
    for tx in &pending {
        assert_eq!(tx.fee, 1000);
    }
}
