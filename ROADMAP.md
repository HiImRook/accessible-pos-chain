# Valid Blockchain - Development Roadmap

**Current Version:** v0.4.8
**Status:** Pre-release (Testnet development)

---

## Release Philosophy

Valid Blockchain follows staged implementation:
- **Foundation first:** Core consensus and transaction handling
- **Economics second:** Tokenomics and validator incentives  
- **Optimization third:** Pruning, performance, and scaling

Features are documented **after** implementation to prevent roadmap drift.

---

## Version History (Completed)

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

### v0.5.0 - Tokenomics & Testing (Target: Q2 2026)

**Scope:** VLid token economics implementation with comprehensive test coverage

#### Features:
- **Block reward minting**
  - Epoch-based decay (÷3 every 7 years)
  - 0.0808 VLid/block → 0.0269 VLid → 0.0090 VLid
- **TPI participation rewards**
  - 0.0045 VLid per validator per slot (Epoch 0)
  - 0.0015 VLid → 0.0005 VLid (subsequent epochs)
- **Racer save rewards**
  - 0.00337 VLid per emergency activation (Epoch 0)
  - Decays ÷3 each epoch
- **Snapshot upload rewards**
  - Amount TBD (placeholder calculations for budget planning)
- **Total supply enforcement**
  - 33M VLid hard cap
  - Epoch budget tracking
- **Genesis allocation**
  - Initial validator distribution
  - Early supporter rewards
  - Testnet bootstrap strategy
- **Fee priority ordering**
  - High-fee transactions processed first
  - Economic incentives for users
- **Comprehensive test suite**
  - Unit tests for mempool behavior
  - TPI consensus scenario tests (3/3, 2/3, 2/2)
  - Transaction validation tests
  - Tokenomics function tests (minting, caps, rewards)
  - Target coverage: 70%+

#### Status:
- Token foundation complete (v0.4.5-v0.4.8)
- Fee distribution simplified (v0.4.7)
- Security hardening complete (v0.4.8)
- Minting logic: Design phase
- Test framework: Planning phase

---

## Upcoming Releases

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
- Requires v0.5.0 tokenomics (supply tracking)

---

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
- Requires v0.5.0 tokenomics (fees have value)
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
  - AI database cernter alternative

---

## Known Limitations (Pre-v0.5.0)

⚠️ **No tokenomics:** Validators don't earn rewards yet
⚠️ **No pruning:** All blocks stored in RAM indefinitely  
⚠️ **No state persistence:** Node restart = chain loss
⚠️ **Placeholder SPO logic:** Fee distribution incomplete

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

**Last Updated:** February 7, 2026
