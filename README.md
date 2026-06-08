# Accessible PoS Chain — Main Branch

A lightweight proof-of-stake blockchain focused on accessibility, decentralization, and merit-based participation. Designed to run efficiently on modest hardware in developing regions while supporting advanced Layer 2 networks.

---

> ⚠️ **Network Identity Notice — v0.6.3**
>
> Validator identity is carried in the direct peer handshake as a transitional bootstrap mechanism. Validator IDs are visible to directly connected peers. **Public or adversarial validator testnets are not recommended until v0.7.0 network identity hardening lands.** Forks should keep validator testnets private until then. See [NETWORKING.md](NETWORKING.md) for full details. Contact me directly for questions or guidance regarding this matter.

---

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
- Low flat transaction fees

**Infrastructure:**
- 6-hour archive segments for durable chain persistence
- Peer-based live sync catch-up on startup
- WebSocket real-time updates
- Built-in metrics dashboard
- Vendored dependencies for supply-chain security

## Current Status: v0.6.3

**Completed:**
* ✅ TPI consensus with merit-based selection
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
* ✅ Comprehensive test suite (46 tests, ~57% coverage)
* ✅ Snapshot primitives with deterministic checksums and atomic writes
* ✅ Recovery RPC endpoints (GET /head, GET /block/:slot)
* ✅ Archive segment module — 6-hour durable chain persistence unit
* ✅ Archive segment generation wired into node — triggers every 2,160 blocks
* ✅ Genesis identity fixed at startup — peer adoption removed
* ✅ Genesis mismatch logging on handshake
* ✅ Validator-aware peer handshake — validator ID binding and quorum gate
* ✅ production_ready gate — blocks production until validator quorum confirmed
* ✅ Canonical peer address normalization
* ✅ 120 second startup timeout with clean exit
* ✅ RPC address advertised in handshake — explicit peer sync endpoint discovery
* ✅ Peer-based live sync — one-time catch-up on startup via /head and /block/:slot
* ✅ Sync failure exits cleanly — no partial-state production

**In Development:**
* 📋 Error handling refactor
* 📋 Memory pruning (2,160 block retention)
* 📋 v0.7.0 network identity hardening — ephemeral network identity, validator proof/binding
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
- ✅ Fees 100% to block producer
- ✅ Ed25519 signature verification on block acceptance
- ✅ Transaction nonce enforcement (replay protection)
- ✅ Comprehensive test suite (46 tests, ~57% coverage)
  - Mempool tests (6)
  - Minting tests (7)
  - Tokenomics tests (8 external + 6 inline)
  - TPI consensus tests (6)
  - Crypto unit tests (8)
  - ChainState validation tests (5)

### Phase 4: State Management 🔄 (In Progress - v0.6.x)
- ✅ Snapshot primitives and deterministic checksums
- ✅ Atomic snapshot write and verified load
- ✅ Recovery RPC endpoints (GET /head, GET /block/:slot)
- ✅ Archive segment module with deterministic checksums and atomic writes
- ✅ Archive generation wired into node (every 2,160 blocks)
- ✅ Genesis identity hardened — fixed at startup
- ✅ Validator-aware handshake and production readiness gate
- ✅ RPC address in handshake and peer-based live sync catch-up
- 📋 Memory pruning (2,160 block retention)
- 📋 Error handling refactor

### Phase 5: Network Security & SPO 📋 (Planned - v0.7.0)
- Ephemeral network identity — validator proof/binding without direct identity disclosure
- Stake Pool Operator (SPO) delegation
- TLS encryption for P2P
- Authentication and rate limiting
- Type safety improvements

### Phase 6: Layer 2 Networks 📋 (Future - v0.8.0+)
- VNS (Valid Name Service - domain registry)
- VIPFS (Valid IPFS - content distribution)
- KEVIN (Distributed AI inference)
- L2 validator rewards

### Phase 7: Community Governance 📋 (Future)
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

**Zero-Comment Code:**
Self-documenting variable names eliminate need for comments. Complexity that requires explanation is unnecessary and just an extra layer of work.

**In-Memory State:**
Complete state management using HashMaps. No external database dependencies ensures sovereignty and auditability.

**6-Hour Archive Segments:**
Every 2,160 blocks, the retiring block range is written as a durable archive segment. This is the chain's long-term memory — not a restore checkpoint, but a permanent historical record. Peers handle live catch-up sync.

**Peer-Based Live Sync:**
On startup, after validator quorum is confirmed, the node queries peers for their current head and fetches any missing blocks sequentially. Production only begins after successful catch-up. Partial sync failure exits cleanly rather than allowing stale-state production.

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

**"Valid, empowering Communities with Sovereign, Decentralized, and Accessible In-Memory Tools, Fostering Freedom and Transparency Through Open-Source Self-Reliance."**
