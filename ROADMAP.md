# Valid Blockchain - Development Roadmap

**Current Version:** v0.7.6
**Status:** v0.7.6 released (Testnet development)

---

## Release Philosophy

Valid Blockchain follows staged implementation:
- **Foundation first:** Core consensus and transaction handling
- **Economics second:** Tokenomics and validator incentives
- **Optimization third:** Pruning, performance, and scaling

Features are documented **after** implementation to prevent roadmap drift.

---

## Version History (Completed)

### v0.7.6 - Arweave Publication Validation (Jul 2026)
- ✅ Arweave signing field order corrected to format 2 spec — format, owner, target, quantity, reward, anchor, tags, data_size, data_root
- ✅ data_root leaf hash corrected — chunk bytes pre-hashed before leaf_hash() — matches arweave-js merkle.ts
- ✅ Wallet loading supports ARWEAVE_WALLET_PATH (file path) alongside ARWEAVE_JWK_JSON
- ✅ Wallet address added to ArweaveWallet struct and logged at startup
- ✅ Full POST /tx response body logged for observability
- ✅ data_root and tx_id logged before submission
- ✅ src/bin/arweave_test.rs — standalone test binary for live mainnet validation
- ✅ Real archive segment submitted to Arweave mainnet — tx_id: 71o-aNdFvGGPEcIvK6b4MCFKQjS-FJD_-KAIKDeiKCA
- ✅ data_root correctness confirmed against live network
- ✅ RSA-PSS signing path confirmed correct
- ✅ Inline upload confirmed sufficient — chunked upload not required at current segment sizes

### v0.7.5 - TLS Trust Hardening Scaffolding (Jul 2026)
- ✅ trusted_peer_fingerprints config field — optional SHA-256 fingerprint allowlist
- ✅ tls_trust_mode config field — validated at startup, unsupported modes exit immediately
- ✅ validate_peer_certificate() — shared cert extraction and trust check helper
- ✅ is_trusted_fingerprint() — case-insensitive, whitespace-tolerant matching
- ✅ LoggingOnlyVerifier — naming reflects actual behavior (logging only, not enforcement at rustls layer)
- ✅ Outbound connection and broadcast path both enforce fingerprint allowlist
- ✅ Empty allowlist = trust all, backward compatible
- ✅ 12 TLS trust tests
- ⚠️ LoggingOnlyVerifier passes all certs at rustls layer — application-level check only
- ⚠️ Ephemeral certs regenerate at startup — fingerprints must be exchanged out-of-band per session
- ⚠️ Persistent validator identity key and session-stable cert pinning deferred

### v0.7.4 - Network Abuse Hardening (Jul 2026)
- ✅ Per-IP inbound connection rate limiting — 5 attempts per 60 seconds, keyed by source IP only
- ✅ Per-peer inbound message rate limiting — 100 messages per 10 seconds per connected peer
- ✅ Handshake counts against peer message budget — rate-limited peers disconnected immediately
- ✅ Rate check runs before update_seen() — rate-limited messages do not mutate peer liveness
- ✅ message_timestamps migrated during normalize_peer_address() with stale entry pruning
- ✅ cleanup_stale_peers() removes message_timestamps alongside peer and dial entries
- ✅ PeerManager::apply_handshake_metadata() — handshake policy extracted into testable helper
- ✅ Gossiped peer addresses validated before entering PeerManager
- ✅ RPC addresses validated after canonicalization — invalid normalized RPC not bound
- ✅ Invalid their_addr rejects all handshake data including gossip and RPC
- ✅ split_host_port() rejects empty bracketed hosts — []:8000 correctly rejected
- ✅ 7 rate limiting tests
- ✅ 23 handshake validation and address parser tests

### v0.7.3 - Peer and Address Canonicalization Hardening (Jul 2026)
- ✅ address.rs canonicalization module — wildcard, localhost, IPv6, hostname normalization
- ✅ is_valid_peer_addr() — malformed inbound handshake identities dropped before hashing
- ✅ Inbound identity derived from canonicalized advertised peer_addr
- ✅ bind_canonical_dial_target() — explicit dial target upgrade on handshake
- ✅ normalize_peer_address() correctly migrates dial target to canonical hash
- ✅ 18 address canonicalization tests
- ✅ 8 peer manager reconciliation tests
- ⚠️ Gossiped peer address validation completed in v0.7.4
- ⚠️ RPC address validation completed in v0.7.4

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
- ⚠️ Arweave Merkle data_root correctness confirmed in v0.7.6
- ⚠️ Chunked upload deferred — inline upload sufficient at current segment sizes

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

### v0.8.0 - Testnet Maturity and Network Hardening (Target: Q3 2026)
- Bootstrap node deployment and peer gossip self-sustainability verification
- Hostname and IPv6 normalization in peer address resolution
- RPC normalization against resolved dial targets
- Additional network abuse resistance and malformed-message handling
- Expanded integration and adversarial testing for networking, sync, and archival flows
- Testnet stability, observability, and operational hardening

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

⚠️ **Chunked upload not implemented** — segments over 8MB deferred; inline upload sufficient at current segment sizes
⚠️ **LoggingOnlyVerifier passes all certs at rustls layer** — application-level trust check only
⚠️ **Ephemeral certs regenerate at startup** — fingerprints must be exchanged out-of-band per session
⚠️ **Persistent validator identity key deferred** — session-stable cert pinning not yet implemented
⚠️ **RPC sync transport hardening deferred**
⚠️ **Genesis mismatch policy** — currently logged but handshake metadata still applied; future hardening decision

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

**Last Updated:** Jul 21, 2026
