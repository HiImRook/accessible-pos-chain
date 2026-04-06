# Valid Blockchain

Live `valid-blockchain` branch of the Accessible PoS Chain.

A sovereign proof-of-stake blockchain written from scratch in Rust. No frameworks, pallets, or inherited consensus primitives.

Designed as a reaction to the "heavy" blockchain concensus and PC requirements.

The `main` branch holds the forkable protocol base.

## Core Principles

- Single Rust binary: one `cargo build --release` executable
- Under 2,000 lines of code total
- Entire chain state lives in in-memory HashMaps. Future developers take note.
- All dependencies vendored
- Merit valued over capital: no token-weighted mechanics, period

## Consensus: TPI (Three-Party Integrity)

- Exactly 3 validators are randomly selected from the eligible pool for each block slot.
- The highest-merit validator among the three becomes the producer. The other two act as verifiers.
- The producer builds the block. The two verifiers independently re-execute and check the work.
- Finality requires 2/3 agreement (sub-second finality on 10-second blocks).
- Merit penalizes bad behavior. Mismatches trigger quarantine, which strengthens the validator set over time.
- Racer backup system provides automatic failover if any selected validator fails to participate.

## Token Economics (VLid)

- Hard cap: 33 million VLid over exactly 21 years (3 epochs of 7 years each).
- Tokens mint only when validated work is proven(when a block is produced).
- No VC allocations, no traditional treasury. Completely non-custodial
- Epoch 0 block reward: 0.0808 VLid(still a WiP).
- Fees: 100% to SPO auto-delegation, developer funding rounds, and legal services
- Genesis bootstrap: 33,000 VLid (0.1%).

Future governance will be merit-based (participation + wallet age).

## Current Status: v0.5.1

**Completed**
- Full TPI consensus (random trio + merit producer + 2/3 finality)
- Merit scoring, penalization, and quarantine logic
- Racer backup system
- In-memory ChainState using HashMaps
- 6-hour Arweave snapshots
- Custom P2P with discovery, one-per-IP enforcement, and gossip
- Mempool with fee priority and duplicate protection
- Ed25519 signature verification
- Wallet CLI
- WebSocket RPC and metrics dashboard
- 46 tests (~57% coverage) covering TPI, mempool, minting, tokenomics, and ChainState

**Next (v0.6.0)**
- Valid Network testnet
- Memory pruning (keep last ~2,160 blocks)
- Full snapshot recovery

## Hardware Requirements

**Minimum** (developing regions / experimental)
- 2 GB RAM
- 500 MB disk
- <3 GB/month bandwidth

**Recommended** (Raspberry Pi class)
- 4 GB RAM
- 1 GB disk
- <4 GB/month bandwidth

**Modern** (overkill / tons of headroom)
- 8+ GB RAM
- Can resume normal PC activity while minimized

## Quick Start
```bash
git clone -b valid-blockchain https://github.com/HiImRook/accessible-pos-chain.git
cd accessible-pos-chain
cargo build --release
```

Bootstrap peers and testnet details are announced on Discord before each launch.

**Join Discord to participate:** https://discord.gg/2SP383cJs9

## Architecture Highlights

- Pure in-memory state using HashMaps — no database or disk writes during operation
- Snapshot system — full state dumped to Arweave every 6 hours, frequent "updating" internal snapshots keep memory bounded and recoverable
- Custom P2P and racer system built from scratch
- All constants in SCREAMING_SNAKE_CASE (important for contributers)
- Complete file implementations, no partial modules, compact-by-design code base
- One validator per IP — Sybil resistance without staking minimums

## Valid Ecosystem

- **Valid CLI Wallet:** https://github.com/HiImRook/Valid-Blockchain-Wallet
- **K.E.V.I.N. AI Agent:** https://github.com/HiImRook/K.E.V.I.N.
- **NFT Assembler:** https://github.com/HiImRook/nft-assembler
- **Valid Terminal:** In development
- **Valid Browser:** In development

## Security

All dependencies vendored. CI runs `cargo audit` on *every* commit. GPG-signed commits recommended.

Protocol changes go through a community governance program. Merit-based, no token-weighted voting.

**Report vulnerabilities** Review our security policy and reporting process in [SECURITY.md](https://github.com/HiImRook/accessible-pos-chain/blob/main/SECURITY.md).

## License

MIT License — See LICENSE file

Copyright (c) 2024-2026 Rook

---

**"Valid, empowering Communities with Sovereign, Decentralized, and Accessible In-Memory Tools, Fostering Freedom and Transparency Through Open-Source Self-Reliance."**
