use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256, Sha384};
use rsa::{pss::BlindedSigningKey, RsaPrivateKey};
use rsa::pkcs8::DecodePrivateKey;
use rsa::signature::{RandomizedSigner, SignatureEncoding};
use rsa::traits::PublicKeyParts;
use data_encoding::BASE64URL_NOPAD;
use jsonwebkey as jwk;
use std::time::{SystemTime, UNIX_EPOCH};
use crate::publication::{PublicationManifest, PublicationReceipt, PublicationStatus};

const ARWEAVE_CHUNK_SIZE: usize = 262_144;
const ARWEAVE_NOTE_SIZE: usize = 32;
const ARWEAVE_FORMAT: &str = "2";
const ARWEAVE_TAG_LIMIT_BYTES: usize = 2048;

fn arweave_inline_max_bytes() -> u64 {
    std::env::var("ARWEAVE_INLINE_MAX_BYTES")
        .ok()
        .and_then(|v| v.parse().ok())
        .unwrap_or(8_388_608)
}

struct ArweaveWallet {
    private_key: RsaPrivateKey,
    owner: Vec<u8>,
}

impl ArweaveWallet {
    fn from_env() -> Result<Self, String> {
        let jwk_json = std::env::var("ARWEAVE_JWK_JSON")
            .map_err(|_| "ARWEAVE_JWK_JSON not set".to_string())?;
        let parsed: jwk::JsonWebKey = jwk_json.parse()
            .map_err(|e| format!("JWK parse failed: {}", e))?;
        let der = parsed.key.try_to_der()
            .map_err(|e| format!("JWK to DER failed: {}", e))?;
        let private_key = RsaPrivateKey::from_pkcs8_der(&der)
            .map_err(|e| format!("DER to RSA key failed: {}", e))?;
        let owner = private_key.n().to_bytes_be();
        let mut hasher = Sha256::new();
        hasher.update(&owner);
        let address = BASE64URL_NOPAD.encode(&hasher.finalize());
        println!("[PUBLISH] Arweave wallet loaded: {}", address);
        Ok(Self { private_key, owner })
    }

    fn sign(&self, message: &[u8]) -> Result<Vec<u8>, String> {
        let mut rng = rand::thread_rng();
        let signing_key = BlindedSigningKey::<Sha256>::new(self.private_key.clone());
        let sig = signing_key.sign_with_rng(&mut rng, message);
        Ok(sig.to_bytes().to_vec())
    }
}

enum DeepHashItem {
    Blob(Vec<u8>),
    List(Vec<DeepHashItem>),
}

fn sha384_bytes(data: &[u8]) -> Vec<u8> {
    let mut h = Sha384::new();
    h.update(data);
    h.finalize().to_vec()
}

fn deep_hash(item: DeepHashItem) -> Vec<u8> {
    match item {
        DeepHashItem::Blob(data) => {
            let tag = format!("blob{}", data.len());
            sha384_bytes(&[sha384_bytes(tag.as_bytes()), sha384_bytes(&data)].concat())
        }
        DeepHashItem::List(children) => {
            let tag = format!("list{}", children.len());
            children.into_iter().fold(sha384_bytes(tag.as_bytes()), |acc, child| {
                sha384_bytes(&[acc, deep_hash(child)].concat())
            })
        }
    }
}

fn sha256_bytes(data: &[u8]) -> Vec<u8> {
    let mut h = Sha256::new();
    h.update(data);
    h.finalize().to_vec()
}

fn encode_note(offset: u64) -> Vec<u8> {
    let mut buf = vec![0u8; ARWEAVE_NOTE_SIZE];
    buf[ARWEAVE_NOTE_SIZE - 8..].copy_from_slice(&offset.to_be_bytes());
    buf
}

fn leaf_hash(data: &[u8], offset: u64) -> Vec<u8> {
    let note = encode_note(offset);
    sha256_bytes(&[sha256_bytes(data), sha256_bytes(&note)].concat())
}

fn branch_hash(left: &[u8], right: &[u8], right_max: u64) -> Vec<u8> {
    let note = encode_note(right_max);
    sha256_bytes(&[sha256_bytes(left), sha256_bytes(right), sha256_bytes(&note)].concat())
}

fn compute_data_root(data: &[u8]) -> Vec<u8> {
    if data.is_empty() {
        return vec![];
    }
    let mut offset = 0u64;
    let mut leaves: Vec<(Vec<u8>, u64)> = data.chunks(ARWEAVE_CHUNK_SIZE)
        .map(|chunk| {
            offset += chunk.len() as u64;
            (leaf_hash(chunk, offset), offset)
        })
        .collect();
    while leaves.len() > 1 {
        let mut next = vec![];
        let mut i = 0;
        while i < leaves.len() {
            if i + 1 < leaves.len() {
                let b = branch_hash(&leaves[i].0.clone(), &leaves[i + 1].0, leaves[i + 1].1);
                next.push((b, leaves[i + 1].1));
                i += 2;
            } else {
                next.push(leaves[i].clone());
                i += 1;
            }
        }
        leaves = next;
    }
    leaves.remove(0).0
}

struct RawTag {
    name: Vec<u8>,
    value: Vec<u8>,
}

#[derive(Serialize, Deserialize, Debug)]
struct ArweaveTag {
    name: String,
    value: String,
}

impl From<&RawTag> for ArweaveTag {
    fn from(t: &RawTag) -> Self {
        ArweaveTag {
            name: BASE64URL_NOPAD.encode(&t.name),
            value: BASE64URL_NOPAD.encode(&t.value),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct ArweaveTx {
    format: u32,
    id: String,
    last_tx: String,
    owner: String,
    tags: Vec<ArweaveTag>,
    target: String,
    quantity: String,
    data: String,
    data_root: String,
    data_size: String,
    reward: String,
    signature: String,
}

pub struct ArweaveClient {
    gateway: String,
    wallet: ArweaveWallet,
    client: reqwest::Client,
}

impl ArweaveClient {
    pub fn from_env() -> Result<Self, String> {
        let gateway = std::env::var("ARWEAVE_GATEWAY")
            .unwrap_or_else(|_| "https://arweave.net".to_string());
        let wallet = ArweaveWallet::from_env()?;
        let client = reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(60))
            .build()
            .map_err(|e| format!("HTTP client build failed: {}", e))?;
        Ok(Self { gateway, wallet, client })
    }

    async fn fetch_tx_anchor(&self) -> Result<String, String> {
        let url = format!("{}/tx_anchor", self.gateway);
        self.client.get(&url).send().await
            .map_err(|e| format!("anchor fetch failed: {}", e))?
            .text().await
            .map_err(|e| format!("anchor read failed: {}", e))
    }

    async fn fetch_price(&self, bytes: u64) -> Result<String, String> {
        let url = format!("{}/price/{}", self.gateway, bytes);
        self.client.get(&url).send().await
            .map_err(|e| format!("price fetch failed: {}", e))?
            .text().await
            .map_err(|e| format!("price read failed: {}", e))
    }

    fn build_raw_tags(manifest: &PublicationManifest) -> Result<Vec<RawTag>, String> {
        let meta = &manifest.artifact.metadata;
        let tags = vec![
            RawTag { name: b"App-Name".to_vec(), value: b"accessible-pos-chain".to_vec() },
            RawTag { name: b"App-Version".to_vec(), value: b"v0.6.7".to_vec() },
            RawTag { name: b"Content-Type".to_vec(), value: b"application/json".to_vec() },
            RawTag { name: b"Archive-Type".to_vec(), value: b"block-segment".to_vec() },
            RawTag { name: b"Chain-Genesis".to_vec(), value: meta.genesis_hash.as_bytes().to_vec() },
            RawTag { name: b"Segment-Start".to_vec(), value: meta.segment_start_slot.to_string().into_bytes() },
            RawTag { name: b"Segment-End".to_vec(), value: meta.segment_end_slot.to_string().into_bytes() },
            RawTag { name: b"Segment-Checksum".to_vec(), value: meta.payload_checksum.as_bytes().to_vec() },
            RawTag { name: b"Archive-Version".to_vec(), value: meta.archive_version.to_string().into_bytes() },
        ];
        let total_bytes: usize = tags.iter().map(|t| t.name.len() + t.value.len()).sum();
        if total_bytes > ARWEAVE_TAG_LIMIT_BYTES {
            return Err(format!("tag bytes {} exceed Arweave limit {}", total_bytes, ARWEAVE_TAG_LIMIT_BYTES));
        }
        Ok(tags)
    }

    pub async fn upload_manifest(&self, manifest: &PublicationManifest) -> PublicationReceipt {
        let segment_start = manifest.artifact.metadata.segment_start_slot;
        let segment_end = manifest.artifact.metadata.segment_end_slot;
        let local_path = manifest.artifact.file_path.clone();
        let checksum = manifest.artifact.metadata.payload_checksum.clone();
        let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();

        let data_bytes = match std::fs::read(&local_path) {
            Ok(b) => b,
            Err(e) => {
                println!("[PUBLISH] Failed to read {}: {}", local_path, e);
                return PublicationReceipt {
                    backend: "arweave".to_string(),
                    object_id: None,
                    local_path,
                    segment_start_slot: segment_start,
                    segment_end_slot: segment_end,
                    payload_checksum: checksum,
                    recorded_at: now,
                    status: PublicationStatus::Failed,
                    error_message: Some(format!("file read failed: {}", e)),
                };
            }
        };

        let data_size = data_bytes.len() as u64;
        let inline_max = arweave_inline_max_bytes();

        if data_size > inline_max {
            println!("[PUBLISH] Segment {}-{} exceeds inline limit ({} bytes) — deferred", segment_start, segment_end, data_size);
            return PublicationReceipt {
                backend: "arweave".to_string(),
                object_id: None,
                local_path,
                segment_start_slot: segment_start,
                segment_end_slot: segment_end,
                payload_checksum: checksum,
                recorded_at: now,
                status: PublicationStatus::DeferredChunkingRequired,
                error_message: Some(format!("segment {} bytes exceeds inline max {}", data_size, inline_max)),
            };
        }

        match self.build_sign_and_submit(manifest, data_bytes, data_size).await {
            Ok(tx_id) => {
                println!("[PUBLISH] Segment {}-{} submitted: {}", segment_start, segment_end, tx_id);
                PublicationReceipt {
                    backend: "arweave".to_string(),
                    object_id: Some(tx_id),
                    local_path,
                    segment_start_slot: segment_start,
                    segment_end_slot: segment_end,
                    payload_checksum: checksum,
                    recorded_at: now,
                    status: PublicationStatus::Submitted,
                    error_message: None,
                }
            }
            Err(e) => {
                println!("[PUBLISH] Segment {}-{} failed: {}", segment_start, segment_end, e);
                PublicationReceipt {
                    backend: "arweave".to_string(),
                    object_id: None,
                    local_path,
                    segment_start_slot: segment_start,
                    segment_end_slot: segment_end,
                    payload_checksum: checksum,
                    recorded_at: now,
                    status: PublicationStatus::Failed,
                    error_message: Some(e),
                }
            }
        }
    }

    async fn build_sign_and_submit(
        &self,
        manifest: &PublicationManifest,
        data_bytes: Vec<u8>,
        data_size: u64,
    ) -> Result<String, String> {
        let anchor = self.fetch_tx_anchor().await?;
        let anchor = anchor.trim().to_string();
        let reward = self.fetch_price(data_size).await?;
        let reward = reward.trim().to_string();

        let data_root_bytes = compute_data_root(&data_bytes);
        let raw_tags = Self::build_raw_tags(manifest)?;

        let anchor_raw = BASE64URL_NOPAD.decode(anchor.as_bytes())
            .map_err(|e| format!("anchor decode failed: {}", e))?;

        let tag_items: Vec<DeepHashItem> = raw_tags.iter().map(|t| {
            DeepHashItem::List(vec![
                DeepHashItem::Blob(t.name.clone()),
                DeepHashItem::Blob(t.value.clone()),
            ])
        }).collect();

        let signing_input = DeepHashItem::List(vec![
            DeepHashItem::Blob(ARWEAVE_FORMAT.as_bytes().to_vec()),
            DeepHashItem::Blob(self.wallet.owner.clone()),
            DeepHashItem::Blob(vec![]),
            DeepHashItem::Blob(data_root_bytes.clone()),
            DeepHashItem::Blob(data_size.to_string().into_bytes()),
            DeepHashItem::Blob(b"0".to_vec()),
            DeepHashItem::Blob(reward.as_bytes().to_vec()),
            DeepHashItem::Blob(anchor_raw),
            DeepHashItem::List(tag_items),
        ]);

        let signing_data = deep_hash(signing_input);
        let signature_bytes = self.wallet.sign(&signing_data)?;

        let mut id_hasher = Sha256::new();
        id_hasher.update(&signature_bytes);
        let tx_id = BASE64URL_NOPAD.encode(&id_hasher.finalize());

        let http_tags: Vec<ArweaveTag> = raw_tags.iter().map(|t| t.into()).collect();

        let tx = ArweaveTx {
            format: 2,
            id: tx_id.clone(),
            last_tx: anchor,
            owner: BASE64URL_NOPAD.encode(&self.wallet.owner),
            tags: http_tags,
            target: String::new(),
            quantity: "0".to_string(),
            data: BASE64URL_NOPAD.encode(&data_bytes),
            data_root: BASE64URL_NOPAD.encode(&data_root_bytes),
            data_size: data_size.to_string(),
            reward,
            signature: BASE64URL_NOPAD.encode(&signature_bytes),
        };

        let url = format!("{}/tx", self.gateway);
        let resp = self.client.post(&url).json(&tx).send().await
            .map_err(|e| format!("POST /tx failed: {}", e))?;

        let status = resp.status();
        if !status.is_success() {
            let body = resp.text().await.unwrap_or_default();
            return Err(format!("POST /tx rejected {}: {}", status, body));
        }

        Ok(tx_id)
    }
}
