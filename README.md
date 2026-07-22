# Accessible TPI Chain — Main Branch

A lightweight **TPI (Three-Party Integrity)** blockchain focused on accessibility, decentralization, and merit-based participation. Designed to run efficiently on modest hardware in developing regions while supporting advanced Layer 2 networks.

---

> ✅ **Network Identity Notice — v0.7.6**
>
> Raw IP addresses are no longer stored as peer identity. Peer identity is epoch-salted and hashed from canonicalized addresses. Malformed handshake identities are dropped before hashing. Inbound connections and peer messages are rate limited. All P2P connections are encrypted with TLS 1.3. Certificate fingerprint pinning is now configurable. Certificates are ephemeral, generated in memory at startup and never persisted as identity artifacts. Archive segments are published to Arweave mainnet, transaction correctness validated live. Bootstrap remains a private ceremony between trusted partners. See [NETWORKING.md](NETWORKING.md) for full details.

---

## This is a TPI Chain

Valid Blockchain uses **Three-Party Integrity (TPI)** — an original consensus mechanism, not a variant of Proof of Stake, Proof of Work, or Delegated PoS.

**How TPI works:**
- Exactly 3 validators are randomly selected from a pool of participants per block slot
- Each computes a candidate block hash independently
- The highest-merit validator among those in agreement produces the block
- The other two verify — finality requires 2/3 agreement
- Bad behavior loses standing, not tokens
- No capital at stake and no computational race. Merit is gained through participation.

TPI is not borrowed from anywhere. I happily spent the past several years developing this as a counter-reaction to unnecessarily heavy blockchain consensus, finality, and state bloat.

---

## Core Features

**Consensus:**
- TPI (Three-Party Integrity) — original consensus mechanism
- Merit-based validator selection — no capital required and no SPOs needed
- Racer backup system for network resilience
- 10-second block times with sub-second finality

**Token Economics:**
- 33 million VLid supply emitted over 21 years
- Tokens mint only when work is proven
- Nonce-based replay protection
- Low flat transaction fees

**Infrastructure:**
- 6-hour archive segments for durable chain persistence
- Archive segments published to Arweave as permanent off-chain record — live mainnet validated
- Archive/prune operations decoupled from chain lock and async runtime threads
- TLS 1.3 encrypted P2P transport — ephemeral self-signed certificates, never persisted
- Configurable TLS fingerprint pinning — trusted_peer_fingerprints allowlist in config.toml
- Address canonicalization — wildcard, localhost, IPv6, and hostname normalization
- Malformed handshake identities dropped before hashing
- Per-IP inbound connection rate limiting — abuse stopped before TLS handshake
- Per-peer inbound message rate limiting — flooding peers disconnected immediately
- Gossiped peer addresses and RPC addresses validated before ingestion
- Peer-based live sync catch-up on startup
- WebSocket real-time updates
- Built-in metrics dashboard
- Vendored dependencies for supply-chain security

## Current Status: v0.7.6

**Completed:**
* ✅ TPI consensus — original mechanism, merit-based, no capital stake
* ✅ validator_id removed from peer handshake entirely
* ✅ Peer connections are identity-free at the transport layer
* ✅ SPO delegation dropped — this is a TPI chain, not a PoS
* ✅ Startup quorum gating replaced by sync-complete readiness
* ✅ Raw IP addresses never stored as peer identity
* ✅ Epoch-salted peer address hashing — 24-hour rotation window derived from genesis hash
* ✅ Peer identity/transport separated — hashed identity layer, raw address transport layer
* ✅ Inbound peers registered only after handshake — no ephemeral source port hashing
* ✅ Inbound identity derived from canonicalized advertised peer_addr — stable across reconnects
* ✅ Address canonicalization module — wildcard, localhost, IPv6, hostname normalization
* ✅ Malformed handshake identities dropped before hashing — never hash a non-address string
* ✅ bind_canonical_dial_target() — stale provisional dial targets upgraded on every handshake
* ✅ normalize_peer_address() overwrites inherited dial target on canonical migration
* ✅ Outbound provisional identity reconciled via handshake normalization
* ✅ Broadcast dials raw transport targets — logs hashes only
* ✅ Gossip stays dialable — known peers distributed as raw addresses
* ✅ Gossiped peer addresses validated before entering PeerManager
* ✅ RPC addresses validated after canonicalization — invalid normalized RPC not bound
* ✅ Invalid their_addr rejects all handshake data including gossip and RPC
* ✅ PeerManager::apply_handshake_metadata() — handshake policy in testable helper
* ✅ Per-IP inbound connection rate limiting — 5 attempts per 60 seconds, keyed by source IP only
* ✅ Per-peer inbound message rate limiting — 100 messages per 10 seconds
* ✅ Handshake counts against peer message budget — rate-limited peers disconnected immediately
* ✅ Rate check before update_seen() — abusive messages do not mutate peer liveness
* ✅ message_timestamps migrated during normalize_peer_address() with stale entry pruning
* ✅ TLS 1.3 on all P2P connections — inbound and outbound
* ✅ Ephemeral self-signed certificates generated in memory at startup — never persisted
* ✅ Peer certificate fingerprints logged for observability
* ✅ Shared TLS configs via Arc — generated once at startup, not per connection
* ✅ trusted_peer_fingerprints config field — optional SHA-256 fingerprint allowlist
* ✅ tls_trust_mode config field — validated at startup, unsupported modes exit immediately
* ✅ validate_peer_certificate() — shared cert extraction and trust check for all outbound paths
* ✅ is_trusted_fingerprint() — case-insensitive, whitespace-tolerant matching
* ✅ Outbound connections and broadcasts enforce fingerprint allowlist
* ✅ LoggingOnlyVerifier — naming reflects actual behavior at rustls layer
* ✅ Transaction nonces and fee structure
* ✅ Racer backup system
* ✅ RPC server with WebSocket support
* ✅ Wallet CLI
* ✅ Token foundation (supply tracking, epoch calculations)
* ✅ Mempool duplicate detection and size limits
* ✅ Block hash security hardening
* ✅ Block reward minting (validators earn 0.0808 VLid/block)
* ✅ Supply cap enforcement (33M VLid hard limit)
* ✅ Fee priority ordering (high-fee transactions first)
* ✅ Ed25519 signature verification on block acceptance
* ✅ Comprehensive test suite (119 tests)
* ✅ Snapshot primitives with deterministic checksums and atomic writes
* ✅ Recovery RPC endpoints (GET /head, GET /block/:slot)
* ✅ Archive segment module — 6-hour durable chain persistence unit
* ✅ Archive segment generation wired into node — triggers every 2,160 blocks
* ✅ Genesis identity fixed at startup — peer adoption removed
* ✅ Genesis mismatch logging on handshake
* ✅ Canonical peer address normalization
* ✅ RPC address advertised in handshake — explicit peer sync endpoint discovery
* ✅ Peer-based live sync — one-time catch-up on startup via /head and /block/:slot
* ✅ Sync failure exits cleanly — no partial-state production
* ✅ Auth binding gap fixed — from address verified against from_pubkey
* ✅ Wallet nonce fixed — live nonce fetched from RPC before signing
* ✅ GET /nonce/:address RPC endpoint
* ✅ /submit returns real success/failure with proper HTTP status codes
* ✅ /balance and /block reject malformed requests with 400 instead of silent defaults
* ✅ MempoolRejection enum for precise rejection reasons
* ✅ Archive/prune no longer holds the ChainState write lock during disk I/O
* ✅ Archive file I/O moved off Tokio worker threads via spawn_blocking
* ✅ Duplicate concurrent archive task guard prevents racing on the same segment
* ✅ Backend-neutral archive publication contract (manifest/receipt/status)
* ✅ Arweave uploader — JWK wallet loading, deep hash, RSA-PSS signing, inline upload
* ✅ Arweave transaction construction validated live on mainnet — signing field order and data_root correctness confirmed
* ✅ Arweave wallet loading from file path via ARWEAVE_WALLET_PATH
* ✅ Background publisher loop — 5-minute scan, retry on failure, skip terminal statuses
* ✅ Oversize guard — segments over 8MB deferred, configurable via ARWEAVE_INLINE_MAX_BYTES
* ✅ Chain-native tag schema embedded in every Arweave archive upload
* ✅ Prune correctness never gated on remote upload success
* ✅ Inline upload confirmed sufficient — chunked upload not required at current segment sizes

**In Development:**
* 📋 Layer 2 networks (VNS, VIPFS, KEVIN)

## Development Phases

### Phase 1: Foundation ✅ (Complete)
- Core blockchain infrastructure
- TPI consensus mechanism
- P2P networking with discovery
- Basic transaction system

### Phase 2: Validator Economy ✅ (Complete)
- Merit-based validator selection
- Racer backup system
- Snapshot system
- Token foundation prep (nonces, fees, supply tracking)
- Mempool security hardening (duplicate detection, size limits)

### Phase 3: Tokenomics & Testing ✅ (Complete - v0.5.1)
- ✅ Block reward minting (0.0808 VLid/block in Epoch 0)
- ✅ Supply cap enforcement (33M VLid)
- ✅ Epoch-based reward decay (60%/30%/10% over 21 years)
- ✅ Fee priority ordering (high-fee transactions first)
- ✅ Fees 100% to block producer (temporary — future governance decision)
- ✅ Ed25519 signature verification on block acceptance
- ✅ Transaction nonce enforcement (replay protection)
- ✅ Comprehensive test suite (119 tests)
  - Mempool tests (6)
  - Minting tests (7)
  - Tokenomics tests (8 external + 6 inline)
  - TPI consensus tests (6)
  - Crypto unit tests (8)
  - ChainState validation tests (5)
  - Archive tests (11)
  - Address canonicalization tests (18)
  - Peer manager reconciliation tests (8)
  - Handshake validation tests (23)
  - Rate limiting tests (7)
  - TLS trust tests (12)

### Phase 4: State Management ✅ (Complete - v0.6.x)
- ✅ Snapshot primitives and deterministic checksums
- ✅ Atomic snapshot write and verified load
- ✅ Recovery RPC endpoints (GET /head, GET /block/:slot)
- ✅ Archive segment module with deterministic checksums and atomic writes
- ✅ Archive generation wired into node (every 2,160 blocks)
- ✅ Genesis identity hardened — fixed at startup
- ✅ RPC address in handshake and peer-based live sync catch-up
- ✅ Auth binding and wallet nonce correctness fixes
- ✅ RPC error handling hardening (precise rejection reasons, proper HTTP status codes)
- ✅ Archive/prune lock-scope and concurrency hardening (v0.6.6)
- ✅ 11 new archive unit tests — checksum, version, block-count, round-trip coverage
- ✅ Arweave archive publication sidecar (v0.6.7)

### Phase 5: TPI Identity and Network Cleanup ✅ (Complete - v0.7.0)
- ✅ validator_id removed from peer handshake
- ✅ Peer connections are identity-free at the transport layer
- ✅ SPO delegation dropped — TPI chain, not PoS
- ✅ Quorum gating replaced by sync-complete readiness
- ✅ Bootstrap is a private ceremony between trusted partners

### Phase 6: Network Hardening and Archive Publication ✅ (Complete - v0.7.1 through v0.7.6)
- ✅ Validator IP hashing with epoch-based salt
- ✅ Peer identity/transport separation — Zero Footprint applied to network layer
- ✅ TLS 1.3 P2P transport encryption — ephemeral self-signed certificates
- ✅ Shared TLS config lifecycle — generated once, not per connection
- ✅ Address canonicalization module — wildcard, localhost, IPv6, hostname normalization
- ✅ Malformed handshake identities dropped before hashing
- ✅ Dial target upgrade on every handshake via bind_canonical_dial_target()
- ✅ Gossiped peer and RPC address validation
- ✅ Per-IP connection rate limiting and per-peer message rate limiting
- ✅ TLS fingerprint pinning scaffolding — configurable allowlist, enforced on all outbound paths
- ✅ Arweave publication pipeline validated live on mainnet
- ✅ 68 new network hardening tests

### Phase 7: Network Maturity and Testnet Hardening 📋 (Future - v0.8.0+)
- Further peer identity and canonicalization hardening
- Hostname and IPv6 normalization in peer address resolution
- RPC normalization against resolved dial targets
- Additional network abuse resistance and malformed-message handling
- Expanded integration and adversarial testing for networking, sync, and archival flows
- Testnet stability, observability, and operational hardening

### Phase 8: Community Governance and Programmable Token Layer 📋 (Future)
- Merit-based voting (XP + wallet age, not token balance)
- Development grants (mint-on-milestone)
- Protocol parameter voting
- Programmable token layer
- No treasury, no foundation needed

### Phase 9: Layer 2 Networks 📋 (Future)
- VNS (Valid Name Service - domain registry)
- VIPFS (Valid IPFS - eventual replacement for Arweave publication backend)
- KEVIN (Distributed AI inference)
- L2 validator rewards

## Hardware Requirements

### MINIMUM - Developing Regions/Experimental Builds
*Works, but not ideal*

- **RAM:** 2 GB
- **Disk:** 500 MB free
- **Internet:** 10 Mbps down / 5 Mbps up
- **Bandwidth:** 10 GB/month (uses 2.6-3.7 GB)

### RECOMMENDED - Raspberry Pi Equivalent
*Goldilocks zone, plenty of clearance*

- **RAM:** 4 GB
- **Disk:** 1 GB free
- **Internet:** 50 Mbps down / 10 Mbps up
- **Bandwidth:** No concern (<4 GB/month)

### MODERN - Most PCs/Laptops
*Overkill, tons of headroom*

- **RAM:** 8 GB
- **Disk:** 5 GB free
- **Internet:** 100 Mbps down / 100 Mbps up
- **Bandwidth:** Negligible

## Quick Start

### Prerequisites
- Rust 1.88+ ([Install Rust](https://rustup.rs/))
- 4GB RAM recommended
- Internet connection

### Build from Source
```bash
git clone https://github.com/HiImRook/accessible-pos-chain.git
cd accessible-pos-chain
cargo build --release
```

## Token Economics (VLid)

**Supply Model:**
- **Total Cap:** 33 million VLid
- **Timeline:** 21 years (3 epochs × 7 years)
- **Decimals:** 9 (nanoVLid = 0.000000001 VLid)
- **Genesis:** 33,000 VLid (0.1% bootstrap allocation)

**Emission Schedule (Divide by 3 every 7 years):**
- **Year 0-7:** 60% of supply
- **Year 7-14:** 30% of supply
- **Year 14-21:** 10% of supply

**Distribution Categories:**
- **L1 Validators:** 15% (block production, TPI, snapshots)
- **L2 Validators:** 20% (VNS, VIPFS, KEVIN coordination)
- **P2P Hosters:** 40% (browser extension infrastructure)
- **Development Grants:** 25% (merit-based, mint-on-milestone)

**Philosophy:**
- Tokens mint ONLY when work is proven
- No pre-mine, no VC allocations
- No treasury, no foundation
- Merit-based governance (not token-weighted, anti-whale)

## Architecture Highlights

**TPI Consensus:**
Three-Party Integrity is an original consensus mechanism. Three validators are selected per slot from a pool of participants, each independently computes a candidate block hash, compares for authenticity, and the highest-merit validator among those in agreement produces. No capital at stake. No computational race. Validator legitimacy is proven through block production, not handshake declarations.

**Zero Footprint Network Layer:**
Raw IP addresses never exist as peer identity artifacts. Peer identity is epoch-salted and hashed from canonicalized addresses at the point of first contact. Malformed handshake identities are dropped before hashing — a non-address string never becomes identity material. Transport addresses live only in a separate mechanical-necessity layer. You cannot leak what you never kept.

**Address Canonicalization:**
All inbound peer addresses are canonicalized before hashing — wildcard bind addresses replaced with actual transport IP, localhost normalized, IPv6 correctly bracketed, hostnames lowercased. Stale provisional dial targets are upgraded on every handshake via explicit canonical upgrade. The identity and transport layers are cleanly separated at every path.

**Network Abuse Hardening:**
Inbound connections are rate limited per source IP before the TLS handshake — ephemeral port rotation does not bypass the limit. Post-handshake message floods are disconnected immediately. Gossiped peer addresses and advertised RPC endpoints are validated before ingestion. Invalid peer identity in a handshake causes all associated handshake data to be ignored.

**TLS 1.3 P2P Transport:**
All peer connections are encrypted with TLS 1.3. Certificates are ephemeral — generated in memory at startup and discarded on shutdown. Certificate fingerprint pinning is now configurable via trusted_peer_fingerprints in config.toml. All outbound connections and broadcasts enforce the allowlist. Empty allowlist means trust all — existing deployments require no changes.

**Arweave Publication Sidecar:**
After each verified local archive segment, a publication manifest is queued. A background task processes the queue every 5 minutes, uploading segments to Arweave as permanent off-chain storage. Transaction construction, deep hash, RSA-PSS signing, and data_root correctness are validated against Arweave mainnet. Prune correctness never depends on upload success as local durability always gates prune. When VIPFS is ready, it replaces Arweave as the publication backend without touching validator logic.

**Zero-Comment Code:**
Self-documenting variable names eliminate need for comments. Complexity that requires explanation is unnecessary and just an extra layer of work.

**In-Memory State:**
Complete state management using HashMaps. No external database dependencies ensures sovereignty and auditability.

**6-Hour Archive Segments:**
Every 2,160 blocks, the retiring block range is written as a durable archive segment. This is the chain's long-term memory, not a restore checkpoint, but a permanent historical record. Peers handle live catch-up sync. This method replaces the planned "memory pruning" update.

**Lock-Scoped Archive Generation:**
Archive segment building, writing, and verification run without holding the chain state lock and without blocking the async runtime. File I/O is isolated via spawn_blocking, and the chain lock is only briefly acquired to clone the block range and, after success, to prune it. Duplicate concurrent archive attempts for the same segment are prevented by an in-memory guard.

**Peer-Based Live Sync:**
On startup, the node queries peers for their current head and fetches any missing blocks sequentially. Production only begins after successful catch-up. Partial sync failure exits cleanly rather than allowing stale-state production.

**Precise RPC Error Handling:**
Malformed requests and mempool rejections return proper HTTP status codes with clear reasons rather than silently defaulting or always reporting success. /submit distinguishes accepted, duplicate, and full-mempool outcomes.

**Vendored Dependencies:**
All dependencies vendored for supply-chain security.

**One Validator Per IP:**
Anti-Sybil protection at network level. Decentralization through geographic distribution.

## Related Projects

- **Valid Blockchain Wallet:** https://github.com/HiImRook/Valid-Blockchain-Wallet
- **K.E.V.I.N. AI Agent:** https://github.com/HiImRook/K.E.V.I.N.
- **NFT Assembler:** https://github.com/HiImRook/nft-assembler
- **Valid Browser:** (Brave fork) - In development

## Contributing

Contributions welcome. This project maintains a compact, readable codebase with strict architectural principles.

**Guidelines:**
- Open issue for large changes first
- Include tests with all PRs
- Follow existing code style:
  - Zero comments (self-documenting names)
  - In-memory state management (Maps/HashMaps)
  - Constants in SCREAMING_SNAKE_CASE
  - Complete file implementations (no fragments)

## Security

**Vulnerability Reporting:**
Report security issues via GitHub Security Advisories or direct message on Discord.

**Supply Chain:**
All dependencies vendored. CI runs `cargo audit` on every commit. GPG-signed commits recommended.

**Audit Status:**
Pre-mainnet. Community audits welcome. Professional audit planned before mainnet launch.

## License

MIT License — See LICENSE file

Copyright (c) 2024-2026 Rook

## Acknowledgements

Built and maintained by Rook.

Questions or inquiries welcome via GitHub issues, or:
- **Join the Discord:** https://discord.gg/2SP383cJs9

---

**"Valid, empowering Communities with Sovereign, Decentralized, and Accessible In-Memory Tools. Fostering Freedom and Transparency Through Open-Source Self-Reliance."**
