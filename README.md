# Valid Blockchain

Live `valid-blockchain` branch of the Accessible PoS Chain.

A sovereign proof-of-stake blockchain written from scratch in Rust. No frameworks, pallets, or inherited consensus primitives.

Designed as a reaction to the "heavy" blockchain consensus and PC requirements.

The `main` branch holds the forkable protocol base.

---

> ⚠️ **Network Identity Notice — v0.6.6**
>
> Validator identity is carried in the direct peer handshake as a transitional bootstrap mechanism. Validator IDs are visible to directly connected peers. Public or adversarial validator testnets are not recommended until v0.7.0 network identity hardening lands. Forks should keep validator testnets private until then. See [NETWORKING.md](https://github.com/HiImRook/accessible-pos-chain/blob/main/NETWORKING.md) for full details. Contact me directly for questions or guidance regarding this matter.

---

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
- Tokens mint only when validated work is proven (when a block is produced).
- No VC allocations, no traditional treasury. Completely non-custodial.
- Epoch 0 block reward: 0.0808 VLid.
- Fees: 100% to block producer (temporary).
- Genesis bootstrap: 33,000 VLid (0.1%).

Future governance will be merit-based (participation + wallet age).

## Current Status: v0.6.6

**Completed**
- Full TPI consensus (random trio + merit producer + 2/3 finality)
- Merit scoring, penalization, and quarantine logic
- Racer backup system
- In-memory ChainState using HashMaps
- Custom P2P with discovery, one-per-IP enforcement, and gossip
- Mempool with fee priority and duplicate protection
- Ed25519 signature verification
- Wallet CLI
- WebSocket RPC and metrics dashboard
- 57 tests covering TPI, mempool, minting, tokenomics, ChainState, and archive segments
- Snapshot primitives with deterministic checksums and atomic writes
- Recovery RPC endpoints (GET /head, GET /block/:slot)
- Archive segment module — 6-hour durable chain persistence unit
- Archive segment generation wired into node — triggers every 2,160 blocks
- Genesis identity fixed at startup — peer adoption removed
- Genesis mismatch logging on handshake
- Validator-aware peer handshake — validator ID binding and quorum gate
- production_ready gate — blocks production until validator quorum confirmed
- Canonical peer address normalization
- 120 second startup timeout with clean exit
- RPC address advertised in handshake — explicit peer sync endpoint discovery
- Peer-based live sync — one-time catch-up on startup via /head and /block/:slot
- Sync failure exits cleanly — no partial-state production
- Auth binding gap fixed — from address verified against from_pubkey
- Wallet nonce fixed — live nonce fetched from RPC before signing
- GET /nonce/:address RPC endpoint
- /submit returns real success/failure with proper HTTP status codes
- /balance and /block reject malformed requests with 400 instead of silent defaults
- MempoolRejection enum for precise rejection reasons
- Archive/prune no longer holds the ChainState write lock during disk I/O
- Archive file I/O moved off Tokio worker threads via spawn_blocking
- Duplicate concurrent archive task guard prevents racing on the same segment

**Next**
- v0.6.7 Arweave upload wiring for archive segments
- v0.7.0 network identity hardening — ephemeral network identity, validator proof/binding

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

- Pure in-memory state using HashMaps. No database or disk writes during operation
- 6-hour archive segments for durable chain persistence and historical record
- Archive generation runs without holding the chain lock or blocking the async runtime. File I/O isolated via spawn_blocking, duplicate concurrent archive attempts prevented
- Peer-based live sync — one-time startup catch-up via peer RPC endpoints
- Precise RPC error handling — malformed requests and mempool rejections return proper HTTP status codes instead of silent defaults
- Custom P2P and racer system built from scratch
- All constants in SCREAMING_SNAKE_CASE (important for contributors)
- Complete file implementations, no partial modules, compact-by-design codebase
- One validator per IP. Sybil resistance without staking minimums

## Valid Ecosystem

- **Valid CLI Wallet:** https://github.com/HiImRook/Valid-Blockchain-Wallet
- **K.E.V.I.N. AI Agent:** https://github.com/HiImRook/K.E.V.I.N.
- **NFT Assembler:** https://github.com/HiImRook/nft-assembler
- **Valid Terminal:** In development
- **Valid Browser:** In development

## Security

All dependencies vendored. CI runs `cargo audit` on every commit. GPG-signed commits recommended.

Protocol changes go through a community governance program. Merit-based, no token-weighted voting.

**Report vulnerabilities:** Review our security policy and reporting process in [SECURITY.md](https://github.com/HiImRook/accessible-pos-chain/blob/main/SECURITY.md).

## License

MIT License — See LICENSE file

Copyright (c) 2024-2026 Rook

---

**"Valid, empowering Communities with Sovereign, Decentralized, and Accessible In-Memory Tools. Fostering Freedom and Transparency Through Open-Source Self-Reliance."**
