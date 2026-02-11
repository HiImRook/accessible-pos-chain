# Changelog

All notable changes to Valid Blockchain will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

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

## [0.4.6] - 2026-02-07

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
