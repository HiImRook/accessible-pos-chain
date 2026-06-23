use pos_chain::types::{Block, Transaction};
use pos_chain::archive::{build_archive_segment, write_archive_segment, blocks_per_segment};

fn fake_hash() -> String {
    "a".repeat(64)
}

fn fake_address() -> String {
    "B".repeat(44)
}

fn fake_pubkey() -> String {
    "c".repeat(64)
}

fn fake_signature() -> String {
    "d".repeat(128)
}

fn build_test_block(slot: u64, tx_count: usize) -> Block {
    let transactions: Vec<Transaction> = (0..tx_count)
        .map(|_| Transaction {
            from: fake_address(),
            from_pubkey: fake_pubkey(),
            to: fake_address(),
            amount: 123456789012,
            nonce: 42,
            fee: 1000,
            signature: fake_signature(),
        })
        .collect();

    Block {
        slot,
        parent_hash: fake_hash(),
        hash: fake_hash(),
        producer: fake_address(),
        timestamp: slot * 10000,
        transactions,
    }
}

fn main() {
    let seg = blocks_per_segment();

    for tx_per_block in [0usize, 5, 10, 25, 50, 100] {
        let blocks: Vec<Block> = (1..=seg)
            .map(|slot| build_test_block(slot, tx_per_block))
            .collect();

        let segment = build_archive_segment(blocks, &fake_hash(), &fake_hash()).unwrap();
        let path = format!("./size_estimate_{}_tx.json", tx_per_block);
        write_archive_segment(&segment, &path).unwrap();

        let metadata = std::fs::metadata(&path).unwrap();
        let size_mb = metadata.len() as f64 / (1024.0 * 1024.0);

        println!("{} tx/block -> segment size: {:.2} MB ({} bytes)", tx_per_block, size_mb, metadata.len());

        std::fs::remove_file(&path).unwrap();
    }
}
