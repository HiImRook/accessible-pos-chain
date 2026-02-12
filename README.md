# Accessible PoS Chain — Valid Blockchain

A lightweight proof-of-stake blockchain focused on accessibility, decentralization, and merit-based participation. Designed to run efficiently on modest hardware in developing regions while supporting advanced Layer 2 networks.

## Core Features

**Consensus:**
- TPI (Three-Party Integrity) consensus with 3 validators per block
- Merit-based validator selection
- Racer backup system for network resilience
- 10-second block times with sub-second finality

**Token Economics:**
- 33 million VLid supply over 21 years
- Proof-of-work minting (tokens mint when work is proven)
- Nonce-based replay protection
- Ultra-low transaction fees with SPO delegation

**Infrastructure:**
- Snapshot system (6-hour intervals)
- WebSocket real-time updates
- Built-in metrics dashboard
- Vendored dependencies for supply-chain security

## Current Status: v0.5.0-beta1

**Completed:**
- ✅ TPI consensus with merit-based selection
- ✅ Transaction nonces and fee structure
- ✅ Racer backup system
- ✅ Snapshot archival (Arweave)
- ✅ RPC server with WebSocket support
- ✅ Wallet CLI
- ✅ Token foundation (supply tracking, epoch calculations)
- ✅ Mempool duplicate detection and size limits
- ✅ Block hash security hardening
- ✅ Block reward minting (validators earn 0.0808 VLid/block)
- ✅ Supply cap enforcement (33M VLid hard limit)
- ✅ Comprehensive test suite (31 tests, 40% coverage)

**In Development:**
- 🔄 v0.5.0-rc: Genesis allocation, fee priority ordering, additional tests (target: 70% coverage)
- 🔄 Layer 2 networks (VNS, VIPFS, KEVIN)

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

### Phase 3: Tokenomics & Testing 🔄 (Active Development - v0.5.0)
- ✅ Block reward minting (0.0808 VLid/block in Epoch 0)
- ✅ Supply cap enforcement (33M VLid)
- ✅ Epoch-based reward decay
- 🔄 TPI participation rewards
- 🔄 Racer save bonuses
- 🔄 Snapshot upload rewards
- 🔄 Genesis allocation (33K VLid distribution)
- 🔄 Fee priority ordering
- 🔄 Comprehensive test suite (40% → 70% coverage target)

### Phase 4: State Management 📋 (Planned - v0.6.0)
- Memory pruning (2,160 block retention)
- Snapshot system for recovery
- Error handling refactor
- Integration testing

### Phase 5: Network Security & SPO 📋 (Planned - v0.7.0)
- Stake Pool Operator (SPO) delegation
- TLS encryption for P2P
- Authentication and rate limiting
- Type safety improvements

### Phase 6: Layer 2 Networks 📋 (Future - v0.8.0+)
- VNS (Valid Name Service - domain registry)
- VIPFS (Valid IPFS - content distribution)
- KEVIN (Distributed AI inference)
- L2 validator rewards

Phase 7: Community Governance 📋 (Future)
- Merit-based voting (XP + wallet age, not token balance)
- Development grants (mint-on-milestone)
- Protocol parameter voting
- No treasury, no foundation needed

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
- Rust 1.70+ ([Install Rust](https://rustup.rs/))
- 4GB RAM recommended
- Internet connection

### Build from Source
```bash
git clone https://github.com/HiImRook/accessible-pos-chain.git
cd accessible-pos-chain
cargo build --release
```

### Run a Validator
```bash
# Generate validator keypair
cargo run --bin keygen

# Start validator node
cargo run --release --bin validator -- \
  --keys validator_keys.json \
  --rpc 0.0.0.0:3000 \
  --p2p 0.0.0.0:4000 \
  --bootstrap /ip4/seed.validchain.io/tcp/4000
```

### Use the Wallet
```bash
# Create wallet
cargo run --bin wallet new

# Check balance
cargo run --bin wallet balance http://localhost:3000

# Send transaction
cargo run --bin wallet send <recipient> <amount> http://localhost:3000
```

## Token Economics (VLid)

**Supply Model:**
- **Total Cap:** 33 million VLid
- **Timeline:** 21 years (3 epochs × 7 years)
- **Decimals:** 9 (nanoVLid = 0.000000001 VLid)
- **Genesis:** ~33,000 VLid (minimal bootstrap)

**Emission Schedule (Divide by 3 every 7 years):**
```
Year 0-7:   60% of supply (19.8B)
Year 7-14:  30% of supply (9.9B)
Year 14-21: 10% of supply (3.3B)
```

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

**Zero-Comment Code:**
Self-documenting variable names eliminate need for comments. Complexity that requires explanation is unnecessary and just an extra layer of work.
**In-Memory State:**
Complete state management using HashMaps. No external database dependencies ensures sovereignty and auditability.

**Vendored Dependencies:**
All dependencies vendored for supply-chain security.

**One Validator Per IP:**
Anti-Sybil protection at network level. This provides decentralization through geographic distribution.

## Related Projects

- **Valid Blockchain Wallet:** https://github.com/HiImRook/Valid-Blockchain-Wallet
- **K.E.V.I.N. AI Agent:** https://github.com/HiImRook/K.E.V.I.N.
- **NFT Assembler:** https://github.com/HiImRook/nft-assembler
- **Valid Browser:** (Brave fork) - In development

## Contributing

Contributions welcome! This project maintains a compact, readable codebase with strict architectural principles.

**High Priority:**
- Multi-validator testing and optimization
- Snapshot system stress testing
- Network partition recovery
- Comprehensive test coverage
- Tokenomics implementation (v0.5.0)

**Guidelines:**
- Open issue for large changes first
- Include tests with all PRs
- Follow existing code style:
  - Zero comments (self-documenting names)
  - In-memory state management (Maps/HashMaps)
  - Constants in SCREAMING_SNAKE_CASE
  - Complete file implementations (no fragments)

**Code Review Philosophy:**
Only change what's absolutely necessary. Preserve established patterns even if they appear inefficient. Ask permission before optimizations.

## Security

**Vulnerability Reporting:**
Report security issues via GitHub Security Advisories or direct message on Discord.

**Supply Chain:**
All dependencies vendored. CI runs `cargo audit` on every commit. GPG-signed commits recommended.

**Audit Status:**
Pre-mainnet. Community audits welcome. Professional audit planned before mainnet launch.

## License

MIT License - See LICENSE file

Copyright (c) 2024-2026 Rook

## Acknowledgements

Built and maintained by Rook.

Questions or inquiries welcome via GitHub issues, or:
- **Join the Discord:** https://discord.gg/2SP383cJs9

---

**"Valid, empowering Communities with Sovereign, Decentralized, and Accessible In-Memory Tools, Fostering Freedom and Transparency Through Open-Source Self-Reliance."**
