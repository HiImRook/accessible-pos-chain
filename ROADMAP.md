# Valid Blockchain - Development Roadmap

**Current Version:** v0.6.5
**Status:** v0.6.0.5 released (Testnet development)

---

## Release Philosophy

Valid Blockchain follows staged implementation:
- **Foundation first:** Core consensus and transaction handling
- **Economics second:** Tokenomics and validator incentives
- **Optimization third:** Pruning, performance, and scaling

Features are documented **after** implementation to prevent roadmap drift.

---

## Version History (Completed)

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
- ⚠️ Transitional bootstrap mechanism — planned replacement in v0.7.0

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
  - Duplicate block rejection
  - Insufficient balance validation
  - Invalid nonce detection
  - Balance update correctness
  - Nonce increment validation
- ✅ Test coverage increased to ~57% (46 tests total)

### v0.5.0-final - Tokenomics & Testing (Mar 2026)
- ✅ Block reward minting (0.0808 VLid/block, Epoch 0)
- ✅ Supply cap enforcement (33M VLid hard limit)
- ✅ Epoch-based reward decay (60%/30%/10% over 21 years)
- ✅ Fee priority ordering (high-fee transactions first)
- ✅ Fees 100% to block producer
- ✅ Transaction nonce enforcement (replay protection)
- ✅ Ed25519 signature verification on block acceptance
- ✅ Comprehensive test suite (41 tests, ~52% coverage)
  - Mempool tests (6)
  - Minting tests (7)
  - Tokenomics tests (8 external + 6 inline)
  - TPI consensus tests (6)
  - Crypto unit tests (8)

### v0.4.8 - Security Hardening (Feb 2026)
- ✅ Block hash includes transaction nonce and fee (security fix)
- ✅ Mempool size limit enforced (10,000 transactions max)

### v0.4.6 - Documentation Clarity (Feb 2026)
- ✅ Clarified v0.4.0 deferred features
- ✅ Added placeholder TODOs for future work
- ✅ Created ROADMAP.md

### v0.4.5 - Token Foundation Prep (Feb 2026)
- ✅ Transaction nonce field (replay protection)
- ✅ Transaction fee field (validator income infrastructure)
- ✅ Signature verification includes all transaction fields
- ✅ Total supply tracking (foundation for v0.5.0)
- ✅ Delegations HashMap (foundation for v0.6.0)

### v0.4.3 - TPI Consensus Hardening (Jan 2026)
- ✅ Enhanced TPI logging and metrics
- ✅ Supply-chain security (vendored dependencies)
- ✅ Unified block hashing
- ✅ Transaction verification improvements

### v0.4.0 - TPI Consensus Foundation (Dec 2025)
- ✅ Three-Person Integrity (TPI) consensus mechanism
- ✅ Merit-based broadcaster selection
- ✅ Racer backup validator system
- ✅ 10-second block finality

### v0.3.0 - Security Foundations (Nov 2025)
- ✅ Ed25519 cryptographic signatures
- ✅ SHA-256 block hashing
- ✅ Basic transaction validation

---

## Current Priorities

### v0.6.0 - State Management (In Progress)

**Scope:** Durable chain persistence via 6-hour archive segments and peer-based sync

#### Completed in alpha series:
- ✅ Snapshot primitives and checksum utilities
- ✅ Recovery RPC endpoints
- ✅ Archive segment module (local build, verify, write)

#### Remaining:
- Wire archive segment generation into main.rs (triggered every 2,160 blocks)
- Peer-based live sync as primary catch-up path
- production_ready gate on peer connection
- Memory pruning (retire blocks after archive segment written)
- Error handling refactor (`.unwrap()` → `Result<T, E>`)

---

## Upcoming Releases

### v0.7.0 - Validator Economics & Network Security (Target: Q4 2026)

**Scope:** Stake Pool Operator (SPO) model with production network security

#### Features:
- SPO delegation system
- Fee distribution logic
- Merit system refinement
- Validator slashing
- TLS encryption for P2P connections
- Per-peer rate limiting
- Type safety improvements

---

## Future Considerations (v0.8.0+)

### Performance & Scaling
- Replace XOR-based selection with VRF
- Bandwidth reduction and message batching

### Developer Experience
- Integration tests for TPI consensus
- Load testing (100 TPS target)
- Replace `println!` with `tracing` crate

### Layer 2 Infrastructure
- VNS (Valid Name Service)
- VIPFS (Valid IPFS)
- KEVIN (Distributed AI inference)

---

## Known Limitations (v0.6.0-alpha.3)

⚠️ **No pruning:** All blocks stored in RAM indefinitely
⚠️ **No state persistence:** Node restart = chain loss
⚠️ **Archive segments not wired:** Generation not yet triggered in node
⚠️ **Placeholder SPO logic:** Fee distribution incomplete
⚠️ **No TLS:** P2P connections unencrypted

**These are intentional staging decisions, not bugs, oversights, or knowledge gaps.**

---

## Contributing

Valid Blockchain is currently in solo development by @HiImRook.

For questions or feedback:
- Join the Discord: https://discord.gg/2SP383cJs9

---

## License

MIT License - See LICENSE file for details

---

**Last Updated:** May 31, 2026
