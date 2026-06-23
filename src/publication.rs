use serde::{Deserialize, Serialize};
use crate::archive::ArchiveMetadata;

pub const PUBLISH_QUEUE_DIR: &str = "./publish_queue";
pub const PUBLISH_RECEIPTS_DIR: &str = "./publish_receipts";

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub enum PublicationStatus {
    Pending,
    Processing,
    Submitted,
    Failed,
    SkippedOversize,
    DeferredChunkingRequired,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ArchiveArtifact {
    pub file_path: String,
    pub metadata: ArchiveMetadata,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PublicationManifest {
    pub backend: String,
    pub artifact: ArchiveArtifact,
    pub content_type: String,
    pub created_at: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PublicationReceipt {
    pub backend: String,
    pub object_id: Option<String>,
    pub local_path: String,
    pub segment_start_slot: u64,
    pub segment_end_slot: u64,
    pub payload_checksum: String,
    pub recorded_at: u64,
    pub status: PublicationStatus,
    pub error_message: Option<String>,
}

fn manifest_path(segment_start: u64, segment_end: u64) -> String {
    format!("{}/segment_{}_{}.manifest.json", PUBLISH_QUEUE_DIR, segment_start, segment_end)
}

fn receipt_path(segment_start: u64, segment_end: u64) -> String {
    format!("{}/segment_{}_{}.receipt.json", PUBLISH_RECEIPTS_DIR, segment_start, segment_end)
}

pub fn build_publication_manifest(
    file_path: String,
    metadata: ArchiveMetadata,
    backend: String,
    created_at: u64,
) -> PublicationManifest {
    PublicationManifest {
        backend,
        artifact: ArchiveArtifact { file_path, metadata },
        content_type: "application/json".to_string(),
        created_at,
    }
}

pub fn write_publication_manifest(manifest: &PublicationManifest) -> Result<(), Box<dyn std::error::Error>> {
    std::fs::create_dir_all(PUBLISH_QUEUE_DIR)?;
    let path = manifest_path(manifest.artifact.metadata.segment_start_slot, manifest.artifact.metadata.segment_end_slot);
    let temp_path = format!("{}.tmp", path);
    let json = serde_json::to_string_pretty(manifest)?;
    std::fs::write(&temp_path, &json)?;
    std::fs::rename(&temp_path, &path)?;
    Ok(())
}

pub fn read_publication_manifest(segment_start: u64, segment_end: u64) -> Result<PublicationManifest, Box<dyn std::error::Error>> {
    let path = manifest_path(segment_start, segment_end);
    let json = std::fs::read_to_string(&path)?;
    let manifest: PublicationManifest = serde_json::from_str(&json)?;
    Ok(manifest)
}

pub fn write_publication_receipt(receipt: &PublicationReceipt) -> Result<(), Box<dyn std::error::Error>> {
    std::fs::create_dir_all(PUBLISH_RECEIPTS_DIR)?;
    let path = receipt_path(receipt.segment_start_slot, receipt.segment_end_slot);
    let temp_path = format!("{}.tmp", path);
    let json = serde_json::to_string_pretty(receipt)?;
    std::fs::write(&temp_path, &json)?;
    std::fs::rename(&temp_path, &path)?;
    Ok(())
}

pub fn read_publication_receipt(segment_start: u64, segment_end: u64) -> Result<PublicationReceipt, Box<dyn std::error::Error>> {
    let path = receipt_path(segment_start, segment_end);
    let json = std::fs::read_to_string(&path)?;
    let receipt: PublicationReceipt = serde_json::from_str(&json)?;
    Ok(receipt)
}
