use pos_chain::arweave::ArweaveClient;
use pos_chain::publication::{PublicationManifest, ArchiveArtifact};
use pos_chain::archive::ArchiveMetadata;
use std::time::{SystemTime, UNIX_EPOCH};

#[tokio::main]
async fn main() {
    let client = match ArweaveClient::from_env() {
        Ok(c) => c,
        Err(e) => {
            eprintln!("[TEST] Failed to load Arweave client: {}", e);
            std::process::exit(1);
        }
    };

    let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();

    let test_path = format!("/tmp/arweave_test_{}.json", now);
    let test_data = serde_json::json!({
        "test": true,
        "chain": "valid-blockchain",
        "segment_start": 1,
        "segment_end": 1,
        "timestamp": now,
    });
    std::fs::write(&test_path, serde_json::to_vec(&test_data).unwrap()).unwrap();

    let manifest = PublicationManifest {
        backend: "arweave".to_string(),
        artifact: ArchiveArtifact {
            file_path: test_path.clone(),
            metadata: ArchiveMetadata {
                archive_version: 1,
                genesis_hash: "test-genesis-hash".to_string(),
                segment_start_slot: 1,
                segment_end_slot: 1,
                block_count: 1,
                first_block_hash: "test-first-hash".to_string(),
                last_block_hash: "test-last-hash".to_string(),
                previous_segment_hash: String::new(),
                payload_checksum: "test-checksum".to_string(),
                written_at: now,
            },
        },
        content_type: "application/json".to_string(),
        created_at: now,
    };

    println!("[TEST] Submitting test segment to Arweave...");
    let receipt = client.upload_manifest(&manifest).await;

    println!("[TEST] Status: {:?}", receipt.status);
    if let Some(tx_id) = receipt.object_id {
        println!("[TEST] SUCCESS — tx_id: {}", tx_id);
        println!("[TEST] View at: https://arweave.net/{}", tx_id);
    }
    if let Some(err) = receipt.error_message {
        println!("[TEST] Error: {}", err);
    }

    let _ = std::fs::remove_file(&test_path);
}
