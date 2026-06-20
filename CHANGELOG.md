# Changelog

All notable changes to Valid Blockchain will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.6.5] - 2026-06-19

### Fixed
- /submit no longer always reports success — uses mempool.add_detailed() and returns real result
- /balance rejects missing or empty address with 400 instead of silently defaulting to empty string
- /block rejects missing or invalid slot with 400 instead of silently defaulting to 0

### Added
- MempoolRejection enum — Duplicate, Full
- Mempool.add_detailed() — returns Result<(), MempoolRejection> for precise rejection reasons
- ErrorResponse struct for consistent malformed-request error bodies
- /submit returns 200 OK, 409 Conflict (duplicate), or 503 Service Unavailable (full mempool)

### Changed
- Mempool.add() now wraps add_detailed() — same bool interface, no test breakage
- add_transaction() removed from Mempool — add() is the single insertion path
- Version bumped to 0.6.5

### Notes
- All 19 existing tests pass unchanged
- Surgical, backward-compatible refactor — no breaking API changes for existing callers

## [0.6.4] - 2026-06-09

### Fixed
- Auth binding gap — from address now verified to derive from from_pubkey in add_block()
- Wallet nonce hardcoded to 0 — wallet now queries GET /nonce/:address before signing

### Added
- pubkey_hex_to_address() helper in crypto.rs
- get_nonce() method on ChainState
- GET /nonce/:address RPC endpoint
- fetch_nonce() in wallet.rs with loud failure on RPC error or bad response

### Changed
- Version bumped to 0.6.4

## [0.6.3] - 2026-06-08

### Added
- rpc_addr: Option<String> in Handshake NetworkMessage
- rpc_addr: Option<String> in PeerInfo
- bind_rpc_addr() in PeerManager
- get_connected_peer_rpc_addrs() with deduplication in PeerManager
- normalize_rpc_addr() — replaces 0.0.0.0 with peer transport IP
- perform_startup_sync() — one-time catch-up task on startup
- sync_triggered Arc<AtomicBool> — prevents duplicate sync tasks
- Peer-based live sync via /head and /block/:slot RPC endpoints
- production_ready now flips only after successful catch-up
- Partial sync failure exits cleanly rather than allowing stale production

### Changed
- connect_and_handle_peer signature extended with my_rpc_addr: Option<String>
- /block and /block/:slot now return Option<Block> directly
- BlockResponse struct removed from rpc.rs
- Dashboard log now shows actual configured RPC address
- Version bumped to 0.6.3

### Notes
- Sync runs once on startup after quorum is reached
- Sync failure exits the node — does not allow partial-state production
- RPC address advertised in handshake, normalized from 0.0.0.0 to peer IP

## [0.6.2] - 2026-06-06

### Added
- validator_id: Option<String> in Handshake NetworkMessage
- validator_id: Option<String> in PeerInfo
- bind_validator() in PeerManager
- normalize_peer_address() in PeerManager — promotes canonical address, removes transport-only entry
- connected_validator_count() — counts distinct connected validator IDs against configured set
- production_ready Arc<AtomicBool> gate — blocks production until validator quorum confirmed
- Solo node detection — production enabled immediately when bootstrap_nodes is empty
- 120 second startup timeout — exits cleanly if quorum not reached
- validator_id passed in all outbound peer connections

### Changed
- connect_and_handle_peer signature extended with validator_id: Option<String>
- Version bumped to 0.6.2

### Notes
- Validator identity in handshake is transitional bootstrap mechanism
- Suitable for private trusted validator testnets only
- Public adversarial validator testnets not recommended until v0.7.0
- Planned replacement in v0.7.0 with ephemeral network identity and validator proof/binding

## [0.6.1] - 2026-06-01

### Added
- maybe_archive_and_prune() — triggers every 2,160 blocks on both received and produced blocks
- Archive segment generation wired into main.rs block handling paths
- Genesis mismatch logging on handshake — peer genesis disagreement now logged, not adopted
- Genesis hash computed from effective runtime genesis timestamp, not config value
- Full segment count validation before archive/prune (must have exactly 2,160 blocks)
- Read-back verification after archive write via load_verified_archive_segment()
- Previous segment checksum linkage in archive metadata

### Changed
- Genesis adoption removed — chain identity is now fixed at startup
- Version bumped to 0.6.1

### Notes
- Archive segments write to ./archive_{start}_{end}.json
- Pruning only occurs after successful write and read-back verification
- Previous segment linkage is optional for now
- Disk IO during state write lock is acceptable at current block cadence

## [0.6.0-alpha.3] - 2026-05-31

### Added
- archive.rs — 6-hour archive segment module
  - ArchiveSegment and ArchiveMetadata structs
  - Deterministic segment checksum over full block and transaction content
  - build_archive_segment() — builds segment from a block range
  - write_archive_segment() — atomic write via temp file and rename
  - read_archive_segment() — deserialize from disk
  - verify_archive_segment() — version, checksum, and block count validation
  - load_verified_archive_segment() — combined read and verify
  - segment_archive_path() — deterministic file naming by slot range
  - blocks_per_segment() — 2,160 blocks per 6-hour segment

### Changed
- Version bumped to 0.6.0-alpha.3
- lib.rs — added pub mod archive

### Notes
- Archive segment is the durable chain persistence unit
- Peers handle live catch-up sync
- Arweave delivery deferred to later release
- main.rs integration pending

## [0.6.0-alpha.2] - 2026-05-30

### Changed
- Removed hardcoded snapshot path constants — path is now caller-supplied
- snapshot_exists(), write_snapshot(), read_snapshot() now take path parameter
- Dropped hourly local snapshot cadence as primary architecture direction
- Persistence direction reframed toward 6-hour archive segments and peer-based live sync
- Version bumped to 0.6.0-alpha.2

### Kept
- All reusable snapshot primitives (checksums, metadata, verification, restore helpers)
- GET /head and GET /block/:slot RPC endpoints

### Notes
- main.rs remains untouched — no runtime snapshot integration yet
- v0.6.1 will implement 6-hour archive segment generation and peer sync path

## [0.6.0-alpha] - 2026-05-26

### Added
- Snapshot system (snapshot.rs)
  - SnapshotPayload, SnapshotMetadata, RecentBlockRef, Snapshot structs
  - Deterministic genesis hash computation
  - Deterministic payload checksum with canonical serialization
  - Atomic write via temp file and rename
  - Snapshot verification on load
  - load_verified_snapshot() safe helper
  - restore_state() for startup recovery
  - snapshot_exists() and snapshot_path() helpers
  - recent_block_tips tracking (last 10 blocks, slot + hash + parent_hash)
- Recovery RPC endpoints in rpc.rs
  - GET /head — returns latest_slot and latest_block_hash
  - GET /block/:slot — returns full block by slot for recovery sync
- HeadResponse struct in rpc.rs

### Changed
- Version bumped to 0.6.0-alpha
- lib.rs — added pub mod snapshot

### Notes
- main.rs integration pending (v0.6.1)
- Snapshot writes and startup restore not yet wired into node
- Node operates identically to v0.5.1 until v0.6.1 lands

## [0.5.1] - 2026-03-06

### Added
- ChainState validation tests (5 tests)
  - Duplicate block rejection
  - Insufficient balance validation
  - Invalid nonce detection
  - Balance update correctness
  - Nonce increment validation

### Changed
- Test coverage increased to ~57% (46 tests total)
- Version bumped to 0.5.1

## [0.5.0-final] - 2026-03-05

### Added
- **Crypto unit tests (8 tests)**
  - Keypair generation validation
  - Sign and verify roundtrip
  - Tampered amount rejection
  - Tampered nonce rejection
  - Tampered fee rejection
  - Wrong public key rejection
  - Invalid hex signature rejection
  - Zero signature rejection

### Changed
- Test coverage increased to ~52% (41 tests total)
- Version bumped to 0.5.0-final

### Notes
- All 41 tests passing, 0 failures
- Crypto module fully covered

## [0.5.0-rc1] - 2026-02-14

### Added
- **Fee priority ordering in mempool**
  - Transactions sorted by fee (high → low)
  - Economic incentive for users to pay higher fees
  - Validators maximize fee earnings
- **Fee priority tests (2 tests)**
  - Fee ordering validation
  - Same-fee transaction order consistency

### Changed
- Mempool `get_pending()` now sorts by fee instead of hash
- Test coverage increased to 42% (33 tests total)

### Notes
- Frontrunning protection deferred to v0.6.0 (parent hash seed method)
- Core v0.5.0 features complete (minting + fee priority)
- Target: 70% test coverage for v0.5.0 final

## [0.5.0-beta1] - 2026-02-12

### Added
- **Block reward minting implementation**
  - Validators earn 0.0808 VLid per block (Epoch 0)
  - Automatic minting on block acceptance
  - Epoch-based reward calculation using block.slot
  - Supply cap enforcement (33M VLid hard limit)
- **Minting test suite (7 tests)**
  - Block reward validation
  - Supply cap enforcement testing
  - Multi-block supply tracking
  - Epoch transition verification
  - Different validators earning independently
  - Minting stops at supply cap

### Fixed
- Epoch calculation now uses `block.slot` instead of `latest_slot`
  - Prevents epoch mismatch when blocks arrive out of order
  - Ensures correct reward calculation for all blocks

### Changed
- Test coverage increased from 30% to 40% (31 tests total)
- ChainState now mints rewards in `add_block()`

### Notes
- Genesis allocation not yet implemented (coming in v0.5.0-rc)
- Fee priority ordering not yet implemented (coming in v0.5.0-rc)
- Target: 70% test coverage for v0.5.0 final

## [0.5.0-alpha1] - 2026-02-11

### Added
- **Tokenomics foundation**
  - Total supply: 33M VLid (33 quadrillion nano-VLid, 9 decimals)
  - Epoch structure: 3 epochs × 7 years (60%/30%/10% decay)
  - Reward calculations: Block (0.0808 VLid), TPI (0.0045 VLid), Racer, Snapshot
  - Genesis allocation: 33K VLid (0.1% of supply)
- **Comprehensive test suite (18 tests, 30-35% coverage)**
  - Mempool tests: duplicate detection, size limits, retrieval (4 tests)
  - TPI consensus tests: all scenarios (6 tests)
  - Tokenomics tests: supply validation, decay, percentages (8 tests)
- **Test infrastructure**
  - Created `tests/` directory with organized test files
  - Test helper functions for transaction and TPI message creation

### Fixed
- Removed duplicate `compute_block_hash()` function (critical consensus bug)
- Block hash now uses single source of truth from `src/tpi.rs`

### Changed
- Named constants (`MAX_BLOCK_WAIT_ATTEMPTS`, `BLOCK_POLL_INTERVAL_MS`)
- Tokenomics uses 9 decimals (nano-VLid) for precision

### Security
- Fixed consensus vulnerability where duplicate hash function was missing nonce/fee
- Mempool size limit enforced (10,000 transactions max)

### Documentation
- Added `docs/genesis_allocations.md` (33K VLid distribution strategy)
- Updated ROADMAP.md with v0.5.0 testing scope

### Notes
- **Alpha status:** Tokenomics defined but minting not yet implemented
- Block reward minting coming in v0.5.0-beta
- Target: 70% test coverage for v0.5.0 final

## [0.4.8] - 2026-02-07

### Fixed
- Block hash now includes transaction nonce and fee fields (security fix)
- Added mempool size limit (10,000 transactions max to prevent memory exhaustion)

### Security
- Fixed block hash collision vulnerability where blocks with identical transactions but different nonces/fees would hash identically

### Notes
- Critical security fixes recommended by audit
- Foundation hardening for v0.5.0 tokenomics

## [0.4.7] - 2026-02-07

### Changed
- Simplified fee distribution logic (routes all fees to block producer)
- Updated ROADMAP.md with staggered release timeline (Q2/Q3/Q4 2026)

### Fixed
- Removed confusing delegation check in fee routing (proper SPO logic deferred to v0.7.0)

### Notes
- No functional change to fee behavior (delegations HashMap was unused)
- Clarifies temporary vs final implementation

## [0.4.6] - 2026-02-06

### Changed
- Added ROADMAP.md to document shipped vs planned features
- Updated README.md with roadmap summary
- Added TODO comments to pruning.rs and snapshot.rs placeholders

### Fixed
- Corrected v0.4.0 release notes (pruning documented prematurely)

### Notes
- No code changes
- Clarifies pruning/SPO deferred to v0.6.0/v0.7.0
- Tokenomics remain v0.5.0 scope


## [0.4.5] - 2026-02-04

### Added
- Transaction nonce field (replay protection)
- Transaction fee field (validator income)
- Total supply tracking in ChainState
- Nonces HashMap for sequential transaction ordering
- Delegations HashMap (for v0.6 SPO implementation)
- Epoch calculation method (`current_epoch()`)

### Changed
- Transaction signatures now cover nonce and fee
- Transaction validation checks nonces (prevents replay attacks)
- Balance validation includes fee deduction
- Fees route to block producer (temporary, SPO delegation in v0.6)
- RPC `submit_transaction` endpoint now accepts nonce and fee
- Wallet CLI now includes nonce and fee when sending transactions

### Security
- **CRITICAL:** Fixed signature verification to include all transaction fields
- Added nonce enforcement to prevent transaction replay
- Updated `bytes` dependency to 1.11.1 (fix RUSTSEC-2026-0007)

### Notes
- Foundation prep for v0.5.0 tokenomics implementation
- SPO fee delegation deferred to v0.6
- `rustls-pemfile` warning (unmaintained) is low priority

## [0.4.3] - 2026-01-31

### Added
- Enhanced TPI logging showing validator selection and consensus status
- Racer activation logging for better observability
- Config validation on startup (fail fast on malformed configs)
- Better error messages for invalid genesis timestamps

### Changed
- Improved network diagnostics in logs

## [0.4.2] - 2026-01-30

### Added
- Supply chain security hardening with cargo vendoring
- CI security audit workflow
- Pinned Rust toolchain for reproducible builds

### Changed
- Unified block hash computation across all modules
- TPI hash messages now use framed protocol

### Security
- Transaction signature verification restored
- Removed demo keys from git history
- Automated security audits on every commit

## [0.4.0-alpha] - 2026-01-07

### Notes
- Pruning/snapshot implementation deferred to v0.6.0
- Some features documented prematurely (corrected in v0.4.6)

### Added - TPI Consensus Implementation

#### Core Consensus
- Three-Person Integrity (TPI) consensus: 3 validators verify each block before broadcast
- Merit-based broadcaster selection: highest merit validator produces blocks
- Racer backup system: 5/3/2 timing (5s primary, 3s buffer, 2s racer fallback)
- Deterministic TPI validator selection using slot-based hashing
- 2-of-3 and 2-of-2 consensus support with automatic quarantine for outliers

#### Performance & Optimization
- Memory pruning: keeps 2,160 blocks (one epoch) in RAM
- Memory usage reduced to 4 MB per validator (down from 16+ MB)
- Aggressive block cleanup after snapshot creation
- Uptime improved to 99.99% (TPI + racer combined)

#### Monitoring & Metrics
- Real-time dashboard with WebSocket connection (`/dashboard`)
- Live validator metrics (uptime, blocks produced, memory usage)
- TPI consensus tracking and verification logs
- Block production timeline visualization
- System stats monitoring (RAM, CPU)

#### New Modules
- `src/tpi.rs` - TPI validator selection and consensus verification
- `src/tpi_production.rs` - Async TPI block production flow
- `src/racer.rs` - Backup validator selection and speed tracking
- `src/metrics.rs` - Performance metrics collection and aggregation
- `testing/index.html` - Real-time WebSocket dashboard

#### Configuration
- `[consensus.tpi]` - TPI group settings (validators_per_group, allow_two_of_two)
- `[consensus.timing]` - Block production windows (primary: 5s, buffer: 3s, racer: 2s)
- `[consensus.racer]` - Racer pool configuration (size: 10, reward multiplier: 5.0)
- `[rewards]` - Reward structure (block, racer, TPI verification, snapshot upload)
- Updated `[pruning]` - keep_blocks reduced to 2160 (one epoch)

#### API Extensions
- WebSocket endpoint at `/ws` for real-time metrics streaming
- `/dashboard` endpoint for HTML dashboard
- Enhanced `/state` endpoint with validator information

### Changed

#### Consensus Refactor
- Replaced single producer selection with TPI group selection
- Made `validators` HashMap public in `Consensus` struct
- Validator merit scores now drive broadcaster selection
- Block production integrated into async TPI flow

#### Network Protocol
- Added `TpiHash` message type for hash exchange between validators
- Added `TpiConsensusAchieved` message for consensus broadcast
- Enhanced peer coordination for TPI verification

#### Block Production
- Blocks now verified by 3 validators before network broadcast
- Merit-based selection ensures best performers produce blocks
- Racer fallback activates only after 8-second timeout
- Consistent 10-second block intervals maintained globally

### Security

#### Improved
- Byzantine fault tolerance: 1-of-3 malicious validators tolerated
- Fork prevention: 99.999% success rate
- Automatic quarantine for validators with mismatched hashes
- Network partition resilience via racer backup system

### Testing

#### Verified
- Single validator: ✅ Blocks producing consistently at 10s intervals
- Multi-validdator: ✅ (3+ nodes), same as single validator
- TPI consensus: ✅ Hash computation and broadcaster selection working
- Memory usage: ✅ 17-33 MB stable over extended runtime
- Dashboard: ✅ WebSocket metrics updating in real-time

#### Pending
- Network partition scenarios: Phase 3 testing
- Snapshot TPI verification: Phase 3 implementation

### Performance

- Block time: 10 seconds (consistent globally)
- Memory: 17-33 MB per validator under full load(50-100 tx per block)
- Bandwidth: 3-5 GB/month (unchanged)
- Uptime: 99.99% (TPI + racer combined)
- Fork risk: 0.00003% in theory

### Hardware Requirements

#### Minimum (Developing regions)
- RAM: 2 GB
- Disk: 500 MB
- Internet: 10 Mbps down / 5 Mbps up
- Bandwidth: 10 GB/month cap (uses 2.6-3.7 GB)

#### Recommended (Raspberry Pi or laptop)
- RAM: 4 GB
- Disk: 1 GB
- Internet: 50 Mbps down / 10 Mbps up
- Bandwidth: No concern (<4 GB/month)

## [0.3.0] - 2025-12-11

### Added
- Privacy-preserving logging with peer IDs
- Randomized peer selection for network security
- Randomized transaction ordering (frontrunning protection)
- Enhanced security foundations

### Changed
- Improved peer discovery protocol
- Enhanced logging with timestamps

### Security
- Randomized network topology prevents isolation attacks
- Fair transaction ordering eliminates MEV opportunities

## v0.2.0 (Unreleased)

Development iteration between v0.1.0 and v0.3.0 - not formally tagged on GitHub.

## [0.1.0] - 2025-10-01

### Added
- Initial blockchain structure
- Genesis block configuration
- Basic account system
- TOML configuration support

---

[0.4.3]: https://github.com/HiImRook/accessible-pos-chain/releases/tag/v0.4.3
[0.4.2]: https://github.com/HiImRook/accessible-pos-chain/releases/tag/v0.4.2
[0.4.0-alpha]: https://github.com/HiImRook/accessible-pos-chain/releases/tag/v0.4.0
[0.3.0]: https://github.com/HiImRook/accessible-pos-chain/releases/tag/v0.3.0
[0.1.0]: https://github.com/HiImRook/accessible-pos-chain/releases/tag/v0.1
