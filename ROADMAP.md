# Valid Blockchain - Development Roadmap

**Current Version:** v0.6.7
**Status:** v0.6.7 released (Testnet development)

---

## Release Philosophy

Valid Blockchain follows staged implementation:
- **Foundation first:** Core consensus and transaction handling
- **Economics second:** Tokenomics and validator incentives
- **Optimization third:** Pruning, performance, and scaling

Features are documented **after** implementation to prevent roadmap drift.

---

## Version History (Completed)

### v0.6.7 - Arweave Archive Publication Sidecar (Jun 2026)
- ✅ Backend-neutral publication contract (manifest/receipt/status)
- ✅ Arweave uploader — JWK wallet loading, deep hash, RSA-PSS signing, inline upload
- ✅ Background publisher loop — 5-minute scan, retry on failure, skip terminal statuses
- ✅ Oversize guard — segments > 8MB deferred, configurable via ARWEAVE_INLINE_MAX_BYTES
- ✅ Tag schema — chain-native metadata embedded in every archive upload
- ✅ Prune correctness never gated on remote upload success
- ✅ Audit config — inapplicable advisories documented and ignored
- ⚠️ Merkle data_root requires ractual network validation with funded wallet
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
- ✅ Test coverage increased to ~57% (51 tests total after v0.6.6)

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
- ✅ Three-Person Integrity (TPI) consensus mechanism
- ✅ Merit-based broadcaster selection
- ✅ Racer backup validator system
- ✅ 10-second block finality

### v0.3.0 - Security Foundations (Nov 2025)
- ✅ Ed25519 cryptographic signatures
- ✅ SHA-256 block hashing
- ✅ Basic transaction validation

---

## Upcoming Releases

### v0.6.8 - Arweave Publication Validation (Target: Q3 2026)
- Real wallet submission test against Arweave mainnet
- Merkle data_root correctness confirmation or fix
- Chunked upload path if segments exceed inline limit in practice

### v0.7.0 - Network Identity Hardening (Target: Q4 2026)
- Ephemeral network identity — validator proof/binding without direct identity disclosure
- Stake Pool Operator (SPO) delegation
- TLS encryption for P2P connections
- Per-peer rate limiting and authentication
- Type safety improvements
- Validator IP hashing with epoch-based salt

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
- VIPFS (Valid IPFS — eventual replacement for Arweave publication)
- KEVIN (Distributed AI inference)

---

## Known Limitations

⚠️ **Arweave Merkle data_root unvalidated** - requires ntwork submission with funded wallet
⚠️ **Chunked upload not implemented** — segments over 8MB deferred
⚠️ **Validator identity transitional** — direct peer handshake carries validator ID until v0.7.0
⚠️ **No TLS** — P2P connections unencrypted until v0.7.0

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

**Last Updated:** Jun 24, 2026
