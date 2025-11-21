# Accessible PoS Chain — (Valid pos-chain)

A lightweight, practical, Proof‑of‑Stake node implementation written in async Rust. This project intentionally favors clarity and runnability over academic maximalism — it’s a small, self‑contained node you can build, run, and use as a private or public testnet for development and experimentation.

Highlights
- 10‑second slot cadence (configurable)
- Stake‑weighted per‑slot producer selection (simple, auditable algorithm)
- Gossip P2P with handshake, peer exchange and automatic reconnect
- Mempool with basic deduplication and size limits
- JSON‑RPC for submission and inspection (get_balance, get_block, submit_transaction, get_peers)
- Genesis and initial validators configured from TOML
- Minimal dependency surface (Tokio, Serde, toml, ed25519 libraries, etc.)

Important disclaimer
This repository is a work‑in‑progress research / engineering project. It is useful for testing, learning, and small private networks. Do not run this on mainnet with real funds until the crypto, storage, and networking primitives have been audited and hardened (see Security section and Roadmap).

Quick start — run a two‑node LAN test
1. Clone and build
   git clone https://github.com/HiImRook/accessible-pos-chain.git
   cd accessible-pos-chain
   cargo build --release

2. Create config and edit for your environment
   cp config.example.toml config.toml
   # edit config.toml to set genesis balances and validators

3. Run a node
   ./target/release/pos-chain

4. Run a second node (change listen_addr and add first node to bootstrap_nodes)
   ./target/release/pos-chain

You should see periodic slot/producer logs and blocks received from peers.

Configuration (config.toml)
- listen_addr — TCP address for P2P
- rpc_addr — JSON‑RPC listen address
- bootstrap_nodes — addresses to bootstrap peer list
- [genesis] — mapping of address -> initial balance
- [validators] — mapping of address -> staked amount (controls selection weight)

Example excerpt:
```toml
listen_addr = "0.0.0.0:8080"
rpc_addr    = "0.0.0.0:8333"

# Leave bootstrap_nodes empty when you start the first node.
# To join peers, add their <host:port> entries here.
bootstrap_nodes = [
  # "198.51.100.42:8080"   # example address (TEST/DOCS ONLY — replace with your peer IPs)
]
```

JSON‑RPC (example)
- Endpoint: configured rpc_addr (default in examples: 0.0.0.0:8333)
- Methods:
  - get_balance { "address": "..." } → { "balance": u64 }
  - get_block { "slot": u64 } → Block (full)
  - submit_transaction signed tx → acceptance response
  - get_peers → list of connected peers

curl example:
curl -X POST http://127.0.0.1:8333 \
  -H "Content-Type: application/json" \
  -d '{"jsonrpc":"2.0","method":"get_balance","params":{"address":"valid1..."},"id":1}'

Architecture overview
- main.rs
  - Spawns Tokio tasks: P2P listener, outbound connector, RPC server, block production loop.
- src/consensus.rs
  - Per‑slot leader selection and block production logic.
- src/network.rs + src/peer_manager.rs
  - TCP-based gossip, handshake, peer list exchange, inbound/outbound connection management.
- src/mempool.rs (or equivalent)
  - Transaction queueing, deduplication, and mempool limits.
- src/rpc.rs
  - JSON‑RPC request parsing and handlers.
- src/crypto.rs
  - Key and signature helpers (ed25519-based today).
- src/types.rs
  - Block, Transaction, and ChainState definitions.

Consensus model — plainspoken detail
This project uses a slot‑based, single‑leader per‑slot model with stake‑weighted selection. Important points:
- Time is divided into fixed slots (default 10s). Each slot has at most one scheduled producer.
- Producer selection is proportional to validator stake. The node deterministically derives the scheduled producer for a slot from the validator set and a deterministic pseudo‑random function (implemented in Rust). This is intentionally simple and auditable; a VRF or full entropy beacon can replace it later.
- Finality model: current design offers probabilistic finality like classic chain‑selection PoS (finality emerges from extending the longest/heaviest chain and fork resolution rules). It is not BFT finality (no multi‑round commit votes).
- Safety / liveness constraints: with a largely honest stake majority and typical network conditions, the chain progresses and conflicts resolve quickly. Under adversarial or highly partitioned conditions, forks can happen and manual reconciliation or more sophisticated consensus (e.g., commit voting, dynamic validator set) is required.

How this differs from Tendermint
- Tendermint: classical BFT consensus with multi‑round voting and deterministic finality (commits require 2/3 votes).
- Accessible PoS (this repo): single‑leader slot game with probabilistic finality based on chain growth and stake weighting. Simpler and easier to implement but relies on eventual chain growth for finality; faster to prototype and run with fewer protocol rounds.

Transaction format and signing
- Transactions are minimal: { from, to, amount, signature } (ed25519 expected).
- Currently the repository contains transaction/verification primitives; full replay protection (nonces) and canonical transaction serialization are on the roadmap.
- Wallets should sign transactions with ed25519 keys and submit via RPC.

Security notes (read this before running validators or wallets)
- Do not use the provided JS “house wallet” for real funds — it currently implements a prototype seed flow and insecure localStorage storage. Treat it as a UX prototype only. See: https://github.com/HiImRook/Valid-Blockchain-Wallet
- Keep validator private keys offline where possible. Do not store keys in plain files on exposed machines.
- The node listens on TCP for P2P — run behind a firewall or in a private LAN for early testing.
- Input validation: RPC and network handlers perform basic checks, but rejecting malformed or malicious input requires further hardening and fuzz testing.
- Dependency & cryptography: ed25519 primitives are in use; cryptographic code paths should be audited before any production use.

Developer notes and best practices (Rust)
- Fully async: prefer Tokio tasks and non‑blocking I/O; avoid blocking calls inside async contexts.
- Shared state: currently the code uses Arc<Mutex<...>> for simplicity. For high concurrency and production load consider:
  - More granular locks (RwLock) or lock sharding.
  - Offloading state mutations to a single writer task and applying commands via channels (actor pattern).
- Determinism:
  - Block serialization and hashing must be deterministic for reliable consensus and syncing; move to Blake3(content) and canonical CBOR/bytes soon.
- Testing:
  - Unit tests for consensus invariants are critical. Add deterministic multi‑node integration tests (spawn nodes within the test harness).
  - Use fixed seeds and deterministic clocks in tests to exercise edge cases.
- Reproducible builds: Cargo.lock is included — keep it up to date for reproducible CI runs.

Roadmap
- Short term
  - Proper transaction signing + replay protection (nonces)
  - Blake3 block hashing and Merkle roots for transactions
  - Genesis generator CLI (avoid manual toml edits)
  - Harden RPC input validation
- Medium term
  - VRF or verifiable randomness for leader selection
  - Light client / explorer (Sled + Axum)
  - Public testnet with multiple stable bootstrap nodes
- Long term
  - Pluggable consensus abstractions (swap in Tendermint‑like or hybrid BFT)
  - Hardware wallet / extension integrations
  - Formal audits and fuzzing

Contributing
- This project is intentionally compact and readable. Contributions that:
  - Reduce contention and improve concurrency
  - Harden cryptography and serialization
  - Improve network robustness and tests
  are especially welcome.
- Large changes: open an issue first describing approach and tests.
- Tests and CI: include tests and a short description of how to reproduce locally.

Repository layout
- Cargo.toml, Cargo.lock
- config.example.toml
- src/
  - main.rs
  - lib.rs
  - consensus.rs
  - crypto.rs
  - network.rs
  - peer_manager.rs
  - rpc.rs
  - types.rs
  - bin/ (additional binaries)

Related projects
- Wallet (prototype, UX): https://github.com/HiImRook/Valid-Blockchain-Wallet
- K.E.V.I.N. (A.I. agent): https://github.com/HiImRook/K.E.V.I.N.
- NFT Assembler (local tool visual NFT builder): https://github.com/HiImRook/nft-assembler

License
- MIT — see LICENSE in this repo.

Acknowledgements
- Built and maintained by the solo author Rook. Contact me directly with questions and inquiries. If you run this successfully on your machines and it behaves, open an issue and tell me.
