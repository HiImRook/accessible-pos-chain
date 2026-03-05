# Valid Blockchain - Development Roadmap

**Current Version:** v0.5.0-final
**Status:** v0.5.0-final released (Testnet development)

---

## Release Philosophy

Valid Blockchain follows staged implementation:
- **Foundation first:** Core consensus and transaction handling
- **Economics second:** Tokenomics and validator incentives
- **Optimization third:** Pruning, performance, and scaling

Features are documented **after** implementation to prevent roadmap drift.

---

## Version History (Completed)

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

### v0.6.0 - State Management & Error Handling (Target: Q3 2026)

**Scope:** Production-ready state management with proper error handling

#### Features:
- **Pruning system**
  - Keep 2,160 blocks (1 epoch) in RAM
  - Archive older blocks to disk snapshots
- **Snapshot mechanism**
  - Periodic in-memory state dumps to disk
  - Fast validator recovery from snapshots
  - No external database dependencies
- **State sync protocol**
  - Download snapshots from peers
  - Verify via merkle proofs
- **Error handling refactor**
  - Replace `.unwrap()` with `Result<T, E>`
  - Graceful failure modes
  - Proper error propagation
- **Integration tests**
  - Full validator workflow tests
  - State recovery scenarios
  - Network partition handling

#### Dependencies:
- Requires v0.5.0 tokenomics ✅

---

## Upcoming Releases

### v0.7.0 - Validator Economics & Network Security (Target: Q4 2026)

**Scope:** Stake Pool Operator (SPO) model with production network security

#### Features:
- **SPO delegation system**
  - Users choose validators
  - Transaction fees route to chosen validator
- **Fee distribution logic**
  - Corrected routing (block producer's delegate)
  - Saturation mechanics (prevent whale dominance)
- **Merit system refinement**
  - XP-based voting power
  - Anti-gaming measures
- **Validator slashing**
  - Penalties for TPI consensus failures
  - Economic security enforcement
- **Network security hardening**
  - TLS encryption for P2P connections
  - Authentication beyond IP address
  - Per-peer rate limiting
- **Type safety improvements**
  - Refactor String hashes to [u8; 32] (breaking change)
  - Address type wrappers

#### Dependencies:
- Requires v0.5.0 tokenomics ✅
- Requires v0.6.0 pruning (mainnet-ready state)

---

## Future Considerations (v0.8.0+)

### Performance & Scaling
- Transaction mempool improvements
  - Duplicate detection
  - Fee market and priority ordering
- Cryptographic randomness
  - Replace XOR-based selection with VRF
- Network optimizations
  - Bandwidth reduction
  - Message batching

### Developer Experience
- Comprehensive test suite
  - Unit tests for crypto operations
  - Integration tests for TPI consensus
  - Load testing (100 TPS target)
- Logging framework
  - Replace `println!` with `tracing` crate
  - Structured logging for analysis
- Documentation
  - Developer guide
  - API reference
  - Consensus specification

### Layer 2 Infrastructure
- VNS (Valid Name Service)
  - Blockchain-based DNS
- VIPFS (Valid IPFS)
  - Content distribution network
- KEVIN (Distributed AI)
  - AI inference network

---

## Known Limitations (v0.5.0-final)

⚠️ **No pruning:** All blocks stored in RAM indefinitely
⚠️ **No state persistence:** Node restart = chain loss
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

**Last Updated:** March 5, 2026
