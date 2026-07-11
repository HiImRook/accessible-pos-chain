# Valid Blockchain

Live `valid-blockchain` branch of the Accessible TPI Chain.

A sovereign **TPI (Three-Party Integrity)** blockchain written from scratch in Rust. No frameworks, pallets, or inherited consensus primitives.

Designed as a reaction to unnecessarily heavy blockchain consensus and PC requirements.

The `main` branch holds the forkable protocol base.

---

> ✅ **Network Identity Notice — v0.7.5**
>
> Raw IP addresses are no longer stored as peer identity. Peer identity is epoch-salted and hashed from canonicalized addresses. Malformed handshake identities are dropped before hashing. Inbound connections and peer messages are rate limited. All P2P connections are encrypted with TLS 1.3. Certificate fingerprint pinning is now configurable. Certificates are ephemeral — generated in memory at startup and never persisted as identity artifacts. Bootstrap remains a private ceremony between trusted partners. See [NETWORKING.md](https://github.com/HiImRook/accessible-pos-chain/blob/main/NETWORKING.md) for full details.

---

## Core Principles

- Single Rust binary: one `cargo build --release` executable
- Under 2,000 lines of code total
- Entire chain state lives in in-memory HashMaps. Future developers take note.
- All dependencies vendored
- Merit valued over capital: no token-weighted mechanics, no SPOs, period

## Consensus: TPI (Three-Party Integrity)

TPI is an original consensus mechanism. It is not a variant of Proof of Stake, Proof of Work, or Delegated PoS. It was designed here as a counter-reaction to unnecessarily heavy blockchain consensus.

- Exactly 3 validators are randomly selected from the eligible pool for each block slot
- Each independently computes a candidate block hash and compares for authenticity
- The highest-merit validator among those in agreement becomes the producer
- The other two act as verifiers. Finality requires 2/3 agreement (sub-second finality on 10-second blocks)
- No capital at stake. No computational race. Merit is gained through participation
- Merit penalizes bad behavior. Mismatches trigger quarantine, which strengthens the validator set over time
- Racer backup system provides automatic failover if any selected validator fails to participate
- Validator legitimacy is proven through block production, not handshake declarations

## Token Economics (VLid)

- Hard cap: 33 million VLid over exactly 21 years (3 epochs of 7 years each)
- Tokens mint only when validated work is proven (when a block is produced)
- No VC allocations, no traditional treasury. Completely non-custodial.
- Epoch 0 block reward: 0.0808 VLid
- Fees: 100% to block producer (temporary — will go to maintenance, legal, and ecosystem grants decided by community governance)
- Genesis bootstrap: 33,000 VLid (0.1%)

Future governance will be merit-based (participation + wallet age).

## Current Status: v0.7.5

**Completed**
- Full TPI consensus (random trio + merit producer + 2/3 finality) — original mechanism
- validator_id removed from peer handshake entirely
- Peer connections are identity-free at the transport layer
- SPO delegation dropped — TPI chain, not PoS
- Startup quorum gating replaced by sync-complete readiness
- Raw IP addresses never stored as peer identity
- Epoch-salted peer address hashing — 24-hour rotation window derived from genesis hash
- Peer identity/transport separated — hashed identity layer, raw address transport layer
- Inbound peers registered only after handshake — no ephemeral source port hashing
- Inbound identity derived from canonicalized advertised peer_addr — stable across reconnects
- Address canonicalization module — wildcard, localhost, IPv6, hostname normalization
- Malformed handshake identities dropped before hashing — never hash a non-address string
- bind_canonical_dial_target() — stale provisional dial targets upgraded on every handshake
- normalize_peer_address() overwrites inherited dial target on canonical migration
- Outbound provisional identity reconciled via handshake normalization
- Broadcast dials raw transport targets — logs hashes only
- Gossip stays dialable — known peers distributed as raw addresses
- Gossiped peer addresses validated before entering PeerManager
- RPC addresses validated after canonicalization — invalid normalized RPC not bound
- Invalid their_addr rejects all handshake data including gossip and RPC
- PeerManager::apply_handshake_metadata() — handshake policy in testable helper
- Per-IP inbound connection rate limiting — 5 attempts per 60 seconds, keyed by source IP only
- Per-peer inbound message rate limiting — 100 messages per 10 seconds
- Handshake counts against peer message budget — rate-limited peers disconnected immediately
- Rate check before update_seen() — abusive messages do not mutate peer liveness
- message_timestamps migrated during normalize_peer_address() with stale entry pruning
- TLS 1.3 on all P2P connections — inbound and outbound
- Ephemeral self-signed certificates generated in memory at startup — never persisted
- Peer certificate fingerprints logged for observability
- Shared TLS configs via Arc — generated once at startup, not per connection
- trusted_peer_fingerprints config field — optional SHA-256 fingerprint allowlist
- tls_trust_mode config field — validated at startup, unsupported modes exit immediately
- validate_peer_certificate() — shared cert extraction and trust check for all outbound paths
- is_trusted_fingerprint() — case-insensitive, whitespace-tolerant matching
- Outbound connections and broadcasts enforce fingerprint allowlist
- LoggingOnlyVerifier — naming reflects actual behavior at rustls layer
- Merit scoring, penalization, and quarantine logic
- Racer backup system
- In-memory ChainState using HashMaps
- Custom P2P with discovery, one-per-IP enforcement, and gossip
- Mempool with fee priority and duplicate protection
- Ed25519 signature verification
- Wallet CLI
- WebSocket RPC and metrics dashboard
- 119 tests covering TPI, mempool, minting, tokenomics, ChainState, archive segments, address canonicalization, peer manager reconciliation, handshake validation, rate limiting, and TLS trust
- Snapshot primitives with deterministic checksums and atomic writes
- Recovery RPC endpoints (GET /head, GET /block/:slot)
- Archive segment module — 6-hour durable chain persistence unit
- Archive segment generation wired into node — triggers every 2,160 blocks
- Genesis identity fixed at startup — peer adoption removed
- Genesis mismatch logging on handshake
- Canonical peer address normalization
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
- Backend-neutral archive publication contract (manifest/receipt/status)
- Arweave uploader — JWK wallet loading, deep hash, RSA-PSS signing, inline upload
- Background publisher loop — 5-minute scan, retry on failure, skip terminal statuses
- Oversize guard — segments over 8MB deferred, configurable via ARWEAVE_INLINE_MAX_BYTES
- Chain-native tag schema embedded in every Arweave archive upload
- Prune correctness never gated on remote upload success

**Next**
- Arweave Merkle data_root validation — requires active testnet network submission with funded wallet (deferred from v0.6.x)
- v0.7.6 Arweave publication validation

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
- 6-hour archive segments for durable chain persistence and historical record. This method replaces the planned "memory pruning" update.
- Archive generation runs without holding the chain lock or blocking the async runtime — file I/O isolated via spawn_blocking, duplicate concurrent archive attempts prevented
- Arweave publication sidecar — verified archive segments queued and uploaded to Arweave as permanent off-chain record; prune never depends on upload success as local durability always gates prune; VIPFS replaces Arweave as the backend when ready
- TLS 1.3 encrypted P2P transport — ephemeral self-signed certificates generated at startup, never persisted, not part of peer identity model
- Configurable TLS fingerprint pinning — trusted_peer_fingerprints allowlist enforced on all outbound connections and broadcasts; empty list means trust all
- Zero Footprint network layer — raw IPs never stored as peer identity; epoch-salted hashed identity separated from raw transport addresses; peer addresses canonicalized before hashing; malformed identities dropped before they become identity material; you cannot leak what you never kept
- Network abuse hardening — inbound connections rate limited per source IP before TLS handshake; message floods disconnected immediately; gossiped peer and RPC addresses validated before ingestion; invalid sender identity rejects all associated handshake data
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
