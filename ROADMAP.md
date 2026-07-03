# Valid Blockchain - Development Roadmap

**Current Version:** v0.7.2
**Status:** v0.7.2 released (Testnet development)

---

## Release Philosophy

Valid Blockchain follows staged implementation:
- **Foundation first:** Core consensus and transaction handling
- **Economics second:** Tokenomics and validator incentives
- **Optimization third:** Pruning, performance, and scaling

Features are documented **after** implementation to prevent roadmap drift.

---

## Version History (Completed)

### v0.7.2 - TLS 1.3 P2P Transport Encryption (Jul 2026)
- ✅ TLS 1.3 on all P2P connections — inbound and outbound
- ✅ Ephemeral self-signed certificates generated in memory at startup — never persisted
- ✅ FingerprintVerifier — peer certificate fingerprints logged for observability
- ✅ Shared TLS configs via Arc — generated once at startup, not per connection
- ✅ Framed message protocol generalized over AsyncRead + AsyncWrite + Unpin
- ✅ Peer identity model preserved above transport layer
- ✅ rust-toolchain.toml bumped to 1.88.0
- ⚠️ P2P TLS trust anchoring and fingerprint pinning deferred
- ⚠️ RPC sync transport hardening deferred

### v0.7.1 - Validator IP Hashing and Peer Identity/Transport Separation (Jul 2026)
- ✅ Epoch-salted peer address hashing — raw IPs never stored as peer identity
- ✅ PeerManager identity/transport split — peers HashMap keyed by hash, dial_targets HashMap for raw addresses
- ✅ Inbound peer registration deferred until handshake — no ephemeral source port hashing
- ✅ Inbound identity from advertised peer_addr — stable across reconnects
- ✅ Outbound provisional identity from dial target — reconciled via handshake normalization
- ✅ Broadcast dials raw transport targets — logs hashes only
- ✅ Gossip stays dialable — known_peers returns raw addresses
- ✅ PeerInfo.address renamed to PeerInfo.peer_hash — semantic alignment
- ⚠️ resolve_dial_addr() handles 0.0.0.0:port only — hostname/IPv6 deferred
- ⚠️ Wildcard RPC normalization uses advertised address — resolved dial target deferred

### v0.7.0 - Handshake Cleanup and TPI Identity Release (Jun 2026)
- ✅ validator_id removed from peer handshake entirely
- ✅ Peer connections are identity-free at the transport layer
- ✅ SPO delegation dropped — Valid Blockchain is a TPI chain, not PoS
- ✅ Quorum gating replaced by sync-complete readiness
- ✅ delegations removed from ChainState and snapshot
- ✅ TPI proves validator legitimacy through block production, not handshake declarations
- ✅ Bootstrap nodes remain temporary scaffolding, removed once gossip is self-sustaining

### v0.6.7 - Arweave Archive Publication Sidecar (Jun 2026)
- ✅ Backend-neutral publication contract (manifest/receipt/status)
- ✅ Arweave uploader — JWK wallet loading, deep hash, RSA-PSS signing, inline upload
- ✅ Background publisher loop — 5-minute scan, retry on failure, skip terminal statuses
- ✅ Oversize guard — segments > 8MB deferred, configurable via ARWEAVE_INLINE_MAX_BYTES
- ✅ Tag schema — chain-native metadata embedded in every archive upload
- ✅ Prune correctness never gated on remote upload success
- ✅ Audit config — inapplicable advisories documented and ignored
- ⚠️ Arweave Merkle data_root pending live-network validation — requires funded wallet submission
- ⚠️ Chunked upload deferred to future release

### v0.6.6 - Archive Lock-Scope and Concurrency Hardening (Jun 2026)
- ✅ ChainState write lock no longer held during archive disk I/O
- ✅ Archive file I/O moved off Tokio worker threads via spawn_blocking
- ✅ Duplicate concurrent archive task guard (in-memory HashSet)
- ✅ Archive work fully decoupled from block production/receipt critical path
- ✅ 11 new archive unit tests — checksum, version, block-count, round-trip coverage
- ✅ Prune range fixed deterministically at trigger time

### v0.6.5 - RPC Error Handling Hardening (Jun 2026)
- ✅ /submit returns real success/failure instead of always reporting success
- ✅ MempoolRejection enum distinguishes Duplicate vs Full
- ✅ /balance and /block reject malformed requests with 400 instead of silent defaults
- ✅ Consistent ErrorResponse body across RPC error paths
- ✅ Backward-compatible — all existing tests pass unchanged

### v0.6.4 - Auth Binding and Nonce Fixes (Jun 2026)
- ✅ Auth binding gap fixed — from address verified against from_pubkey
- ✅ Wallet nonce hardcoded 0 fixed — live nonce fetched from RPC before signing
- ✅ GET /nonce/:address RPC endpoint added
- ✅ pubkey_hex_to_address() helper in crypto.rs
- ✅ fetch_nonce() fails loudly on RPC error

### v0.6.3 - Peer-Based Live Sync (Jun 2026)
- ✅ RPC address carried in peer handshake
- ✅ RPC address normalization (0.0.0.0 → peer IP)
- ✅ One-time startup catch-up sync via /head and /block/:slot
- ✅ production_ready flips only after successful sync
- ✅ Partial sync failure exits cleanly
- ✅ Duplicate sync task prevention via sync_triggered flag

### v0.6.2 - Validator-Aware Network Identity (Jun 2026)
- ✅ Validator ID carried in direct peer handshake
- ✅ Canonical peer address normalization
- ✅ Distinct validator ID counting for quorum
- ✅ production_ready gate — blocks production until quorum confirmed
- ✅ Solo node exception — immediate production when no bootstrap nodes
- ✅ 120 second startup timeout with clean exit

### v0.6.1 - Archive Segment Integration (Jun 2026)
- ✅ Archive segment generation wired into main.rs
- ✅ Triggers every 2,160 blocks on both received and produced blocks
- ✅ Full segment count required before archive/prune
- ✅ Read-back verification after write
- ✅ Genesis identity fixed at startup — no peer adoption
- ✅ Genesis mismatch logging on handshake

### v0.6.0-alpha.3 - Archive Segment Module (May 2026)
- ✅ archive.rs — 6-hour archive segment module
- ✅ ArchiveSegment and ArchiveMetadata structs
- ✅ Deterministic segment checksum over full block and transaction content
- ✅ build_archive_segment() from block range
- ✅ Atomic write, read, verify, and load_verified_archive_segment()
- ✅ segment_archive_path() deterministic file naming by slot range
- ✅ 2,160 blocks per segment (6-hour window)

### v0.6.0-alpha.2 - Snapshot Cleanup and Persistence Reframe (May 2026)
- ✅ Removed hardcoded snapshot path constants
- ✅ snapshot_exists(), write_snapshot(), read_snapshot() now take path parameter
- ✅ Dropped hourly local snapshot cadence as primary architecture direction
- ✅ Persistence direction reframed toward 6-hour archive segments
- ✅ Peer-based live sync established as primary catch-up path
- ✅ GET /head and GET /block/:slot RPC endpoints retained

### v0.6.0-alpha - Snapshot System Foundation (May 2026)
- ✅ Snapshot structs (SnapshotPayload, SnapshotMetadata, RecentBlockRef, Snapshot)
- ✅ Deterministic genesis hash computation
- ✅ Deterministic payload checksum with canonical serialization
- ✅ Atomic snapshot write via temp file and rename
- ✅ Snapshot verification and load_verified_snapshot() helper
- ✅ restore_state() for startup recovery
- ✅ recent_block_tips tracking (last 10 blocks, slot + hash + parent_hash)
- ✅ GET /head RPC endpoint (latest_slot + latest_block_hash)
- ✅ GET /block/:slot RPC endpoint (full block by slot)

### v0.5.1 - ChainState Validation Testing (Mar 2026)
- ✅ ChainState validation tests (5 tests)
- ✅ Test coverage increased to 51 tests total

### v0.5.0-final - Tokenomics & Testing (Mar 2026)
- ✅ Block reward minting (0.0808 VLid/block, Epoch 0)
- ✅ Supply cap enforcement (33M VLid hard limit)
- ✅ Epoch-based reward decay (60%/30%/10% over 21 years)
- ✅ Fee priority ordering (high-fee transactions first)
- ✅ Fees 100% to block producer
- ✅ Transaction nonce enforcement (replay protection)
- ✅ Ed25519 signature verification on block acceptance

### v0.4.8 - Security Hardening (Feb 2026)
- ✅ Block hash includes transaction nonce and fee
- ✅ Mempool size limit enforced (10,000 transactions max)

### v0.4.5 - Token Foundation Prep (Feb 2026)
- ✅ Transaction nonce field
- ✅ Transaction fee field
- ✅ Total supply tracking
- ✅ Delegations HashMap

### v0.4.3 - TPI Consensus Hardening (Jan 2026)
- ✅ Enhanced TPI logging and metrics
- ✅ Supply-chain security (vendored dependencies)
- ✅ Unified block hashing

### v0.4.0 - TPI Consensus Foundation (Dec 2025)
- ✅ Three-Party Integrity (TPI) consensus mechanism
- ✅ Merit-based broadcaster selection
- ✅ Racer backup validator system
- ✅ 10-second block finality

### v0.3.0 - Security Foundations (Nov 2025)
- ✅ Ed25519 cryptographic signatures
- ✅ SHA-256 block hashing
- ✅ Basic transaction validation

---

## Upcoming Releases

### v0.7.3 - Network Hardening (Target: Q3 2026)
- Connection-level rate limiting
- Message-level rate limiting
- Further peer identity/canonicalization hardening
- Hostname and IPv6 normalization in resolve_dial_addr()
- RPC normalization against resolved dial target
- P2P TLS trust anchoring and fingerprint pinning

### v0.7.4 - Arweave Publication Validation (Target: Q3 2026)
- Real wallet submission test against Arweave mainnet
- Merkle data_root correctness confirmation or fix
- Chunked upload path if segments exceed inline limit in practice

---

## Future Considerations (v0.8.0+)

### Anti-Frontrunning
- Parent block hash as deterministic transaction ordering seed
- Eliminate MEV extraction without requiring a private mempool
- Preserve transparency while making ordering less predictable before inclusion

### Performance & Scaling
- Replace XOR-based producer selection with VRF
- Bandwidth reduction and message batching
- Load testing and throughput characterization
- Continued network and transport hardening

### Developer Experience
- Integration tests for TPI consensus
- Replace println! with tracing crate
- Stronger test coverage for networking, sync, and archival flows

### Layer 2: VNS (Valid Name Service)
- Domain registry built in the same minimal architectural style as L1
- L2 witnesses L1 payments via light-client style verification — no bridge
- Own-forever naming model rather than recurring renewal
- Premium-name policy and anti-spam economics to be defined during implementation
- Blacklist/governance enforcement, if any, to be narrowly scoped and explicitly documented
- VLid-native economic model — no separate token

### Layer 2: VIPFS (Valid IPFS)
- Distributed content seeding and retrieval layer
- Validators and users contribute availability based on storage and access patterns
- Content moderation and illegal-content handling model to be explicitly defined before implementation
- Can eventually replace Arweave as archive publication backend
- Publication sidecar already designed to allow backend replacement

### Layer 2: KEVIN (Distributed AI Inference)
- Distributed inference marketplace aligned with the broader Valid stack
- Likely multi-tier execution model spanning local, peer, and specialized validator compute
- Hardware requirements, pricing, and validator reward model deferred until architecture is defined
- VLid-native settlement model preferred over introducing a separate token
- Meet Kevin now in Discord and help with live testing

### Valid Browser
- Browser client with Valid-native integrations
- Built-in L1 wallet support as a first-class feature
- Native resolution for Valid naming systems
- Native access to Valid storage/publication layers
- Direct L1 payment signing and application-layer interactions
- Initial scope likely constrained to the Valid network first

### Additional Future L2s
- Media, social, compute, storage, and other specialized services
- Each L2 should match hardware requirements to its actual workload
- Settlement and economic coordination anchored to L1
- Avoid unnecessary per-L2 token proliferation

---

## Known Limitations

⚠️ **Arweave Merkle data_root pending live-network validation** — requires funded wallet submission
⚠️ **Chunked upload not implemented** — segments over 8MB deferred
⚠️ **P2P TLS trust anchoring and fingerprint pinning deferred**
⚠️ **RPC sync transport hardening deferred**
⚠️ **resolve_dial_addr() handles 0.0.0.0:port only** — hostname/IPv6 normalization deferred

**These are intentional staging decisions, not bugs, oversights, or knowledge gaps.**

---

## Contributing

Valid Blockchain is currently in solo development by Rook.

For questions or feedback:
- Join the Discord: https://discord.gg/2SP383cJs9

---

## License

MIT License - See LICENSE file for details

---

**Last Updated:** Jul 3, 2026
