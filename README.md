# Accessible PoS Chain — Valid Blockchain

A lightweight, decentralized-by-nature, Proof-of-Stake blockchain implementation written in async Rust. Under this framework, validators in developing regions can participate equally with those in major cities using modest hardware (4-8GB RAM, modest consumer CPUs, laptops). This is a fully functional blockchain node you can build, run, and deploy for testnet experimentation and real-world validator networks, now, with a few commands(see below) and light configuration. Bring your own protocol and tokenomics and you have your own blockchain, or join the Valid fork.

## Highlights

- **10-second block times** - Fast confirmation while maintaining global accessibility for validators on slower connections
- **Stake-weighted consensus** - Fair validator selection proportional to stake (no pooling required, solo validators welcome)
- **Automatic peer discovery** - Validators find each other through gossip protocol, no manual IP configuration needed after initial bootstrap
- **Randomized peer selection** - Protects against attackers trying to isolate validators from the network
- **Randomized transaction ordering** - Fair transaction processing, prevents frontrunning
- **Full signature verification** - Every transaction cryptographically verified using ed25519 before acceptance
- **Simple JSON-RPC API** - Easy integration for wallets, explorers, and applications
- **Minimal dependencies** - Clean codebase built on Tokio (async runtime), Serde (serialization), and ed25519-dalek (cryptography)
- **TOML configuration** - Human-readable config files for genesis state and validator setup

## Quick Start — Run a Two-Node LAN Test

### 1. Clone and Build

```bash
git clone https://github.com/HiImRook/accessible-pos-chain.git
cd accessible-pos-chain
cargo build --release
```

### 2. Create Config

```bash
cp config.example.toml config.toml
```

Edit `config.toml` to set:
- Genesis balances (initial token distribution)
- Validator addresses and stakes
- Network settings (ports, bootstrap nodes)

### 3. Run First Node (Bootstrap)

```bash
./target/release/pos-chain
```

### 4. Run Second Node (Peer)

Edit a second `config.toml`:
- Change `listen_addr` to different port (e.g., `0.0.0.0:8081`)
- Add first node's address to `bootstrap_nodes`

```bash
./target/release/pos-chain
```

You should see:
- Periodic slot/producer logs
- Blocks received from peers
- Peer discovery and connection logs

## Configuration (config.toml)

```toml
listen_addr = "0.0.0.0:8080"    # P2P TCP listener
rpc_addr    = "0.0.0.0:8333"     # JSON-RPC HTTP API

# Bootstrap nodes (leave empty for first node, add peer addresses for others)
bootstrap_nodes = [
  # "192.168.1.100:8080"   # Example peer address
]

[genesis]
# Initial token balances (address -> amount)
"valid1alice..." = 1000000
"valid1bob..." = 500000

[[validators]]
address = "valid1alice..."
stake = 1000000

[[validators]]
address = "valid1bob..."
stake = 500000
```

### Configuration Fields

- **listen_addr** - TCP address for P2P gossip protocol
- **rpc_addr** - JSON-RPC HTTP API listen address
- **bootstrap_nodes** - List of peer addresses to connect to on startup (empty for bootstrap node)
- **[genesis]** - Mapping of address -> initial token balance
- **[[validators]]** - Validator configurations (address, stake amount)

## JSON-RPC API

**Endpoint:** Configured `rpc_addr` (default: `http://0.0.0.0:8333`)

### Available Methods

#### get_balance
```bash
curl -X POST http://127.0.0.1:8333 \
  -H "Content-Type: application/json" \
  -d '{
    "jsonrpc": "2.0",
    "method": "get_balance",
    "params": {"address": "valid1alice..."},
    "id": 1
  }'
```

**Response:**
```json
{
  "jsonrpc": "2.0",
  "result": {"balance": 1000000},
  "id": 1
}
```

#### get_block
```bash
curl -X POST http://127.0.0.1:8333 \
  -H "Content-Type: application/json" \
  -d '{
    "jsonrpc": "2.0",
    "method": "get_block",
    "params": {"slot": 42},
    "id": 1
  }'
```

#### submit_transaction
```bash
curl -X POST http://127.0.0.1:8333 \
  -H "Content-Type: application/json" \
  -d '{
    "jsonrpc": "2.0",
    "method": "submit_transaction",
    "params": {
      "from": "valid1alice...",
      "from_pubkey": "02abc...",
      "to": "valid1bob...",
      "amount": 1000,
      "signature": "a1b2c3..."
    },
    "id": 1
  }'
```

#### get_peers
```bash
curl -X POST http://127.0.0.1:8333 \
  -H "Content-Type: application/json" \
  -d '{
    "jsonrpc": "2.0",
    "method": "get_peers",
    "params": {},
    "id": 1
  }'
```

## Architecture Overview

### Core Components

**main.rs**  
Spawns concurrent Tokio tasks:
- P2P listener (incoming connections)
- Outbound connector (peer discovery)
- RPC server (JSON-RPC API)
- Block production loop (validator task)

**src/consensus.rs**  
Per-slot leader selection and block production logic using stake-weighted deterministic algorithm.

**src/network.rs + src/peer_manager.rs**  
TCP-based gossip protocol with:
- Handshake on connection
- Peer list exchange (automatic discovery)
- Randomized peer selection (Phase 1 security)
- Automatic reconnection on disconnect

**src/types.rs**  
Core data structures: Block, Transaction, ChainState, Mempool, PeerInfo

**src/mempool.rs**  
Transaction queueing with:
- Deduplication
- Size limits
- Randomized transaction ordering (Phase 1 security)

**src/rpc.rs**  
JSON-RPC request parsing and handlers (Axum-based HTTP server)

**src/crypto.rs**  
ed25519 key generation, signing, and verification helpers

**src/config.rs**  
TOML configuration parsing and validation

## Consensus Model

This project uses a **slot-based, single-leader-per-slot Proof-of-Stake** model with stake-weighted selection.

### Key Characteristics

**Fixed time slots:** Time is divided into fixed 10-second slots (configurable). Each slot has at most one scheduled producer.

**Stake-weighted selection:** Producer selection is proportional to validator stake. The node deterministically derives the scheduled producer from the validator set using a pseudo-random function based on `(slot ^ epoch) % total_stake`.

**Finality model:** Probabilistic finality similar to classic chain-selection PoS. The longest valid chain with the most stake behind it becomes canonical. Validators converge on the canonical chain through the gossip protocol.

**Safety/liveness:** With an honest stake majority and typical network conditions, the chain progresses smoothly with minimal forks. The deterministic producer selection and rapid block times ensure consistent forward progress.

### Future Enhancements

**VRF (Verifiable Random Function) for leader selection**  
*Technical:* Cryptographically secure randomness for selecting block producers  
*What it means:* Makes it impossible for attackers to predict which validator will produce the next block, eliminating targeted attacks

**BFT-style finality with validator voting**  
*Technical:* Byzantine Fault Tolerant consensus with multi-round commit votes  
*What it means:* Blocks become absolutely final within seconds with no possibility of reversal

**Dynamic validator set with stake delegation**  
*Technical:* Validators can accept delegated stake from token holders who don't run nodes  
*What it means:* Professional validators can accept stake from others, allowing anyone to earn rewards from securing the network without technical expertise or hardware

### Merit-Based Validator Selection (Future Roadmap)

**Phase 3: 3-Validator Snapshot Cross-Check**  
*Technical:* Three randomly selected validators verify blockchain state snapshots match  
*What it means:* Detects validators with corrupted or manipulated state. Divergent validators are flagged and potentially removed from the network, preventing eclipse attacks and state corruption

**Phase 3: Arweave Snapshot Storage**  
*Technical:* Periodic state snapshots stored permanently on Arweave decentralized storage  
*What it means:* Complete blockchain state can be recovered from permanent storage. New validators can sync quickly from verified snapshots instead of replaying entire history

**Phase 4: Peer Quality Scoring**  
*Technical:* Track peer latency, uptime, and block relay performance  
*What it means:* Validators automatically prefer connections to high-quality, reliable peers

**Phase 4: Geographic Diversity Preference**  
*Technical:* Balance peer connections across different geographic regions  
*What it means:* Prevents regional network failures from isolating validators, fair global participation

**Phase 5: Reputation-Based Peer Prioritization**  
*Technical:* Long-term tracking of validator behavior and reliability  
*What it means:* Honest, reliable validators get preferred treatment in peer selection, making network attacks more expensive. Im other words, protoritization for well-performers, kick metrics for bad performers

## Transaction Format and Signing

Transactions are minimal ed25519-signed messages:

```rust
{
  "from": "valid1alice...",        // Base58 address (derived from pubkey)
  "from_pubkey": "02abc...",        // Hex-encoded ed25519 public key
  "to": "valid1bob...",             // Recipient address
  "amount": 1000,                   // Amount in smallest unit
  "signature": "a1b2c3..."          // Hex-encoded ed25519 signature
}
```

**Signature generation:** Sign `from||to||amount` using ed25519 private key.

**Verification:** Node verifies signature against `from_pubkey` before accepting transaction into mempool.

**Coming Soon:**
- Transaction nonces for replay protection (Phase 3)
- Transaction fee model and validator rewards (Phase 4)
- CBOR serialization with Blake3 hashing (Phase 3)

## Security Architecture

### Current Security Features ✅

**Cryptographic signatures (ed25519)**  
Every transaction is signed with industry-standard elliptic curve cryptography. Signatures are verified before transactions enter the mempool, ensuring only authorized transfers occur.

**Randomized peer selection**  
Validators randomly select which peers to connect to from their known peer list. This prevents attackers from predictably isolating validators or controlling their view of the network.

**Randomized transaction ordering**  
Transactions are shuffled before being packaged into blocks. This eliminates frontrunning opportunities and ensures fair transaction processing regardless of submission timing.

**Gossip protocol with automatic peer discovery**  
Validators share their peer lists with each other, creating a self-healing network that automatically routes around failures and discovers new participants.

### Security Roadmap

**Phase 3: Transaction Replay Protection**  
Adding nonce-based replay protection to prevent reuse of valid transaction signatures

**Phase 3: VRF Leader Selection**  
Implementing Verifiable Random Functions for cryptographically secure, unpredictable block producer selection

**Phase 4: Enhanced Input Validation**  
Comprehensive validation and sanitization of all RPC inputs and network messages

**Phase 5: Formal Security Audit**  
Professional third-party security audit of all cryptographic implementations and consensus logic

**Phase 5: Fuzz Testing**  
Automated fuzzing of network protocol and RPC endpoints to discover edge cases

### Validator Best Practices

**Key management:** Keep validator private keys private! Consider hardware security modules (HSM) for production deployments.

**Network security:** Run validators behind firewalls with quality antivirus protection. Isolate validator machines from general-purpose computing.

**Monitoring:** Track validator uptime, block production, and peer connectivity to ensure healthy network participation.

## Contributing

This project intentionally maintains a compact, readable codebase. Contributions welcome in these areas:

**High Priority:**
- Reducing lock contention and improving concurrency
- Hardening cryptography and serialization
- Improving network robustness and peer management
- Adding comprehensive tests

**Process:**
1. **Large changes:** Open an issue first describing approach and testing plan
2. **Tests required:** Include unit/integration tests and reproduction instructions
3. **Code style:** Follow existing patterns, no code bloat (zero comments, self-documenting names)

## Related Projects

- **Wallet (prototype UX):** https://github.com/HiImRook/Valid-Blockchain-Wallet
- **K.E.V.I.N. (AI agent):** https://github.com/HiImRook/K.E.V.I.N.
- **NFT Assembler (visual builder):** https://github.com/HiImRook/nft-assembler
- **Valid Browser (Brave fork):** In development

## Token Economics (Vlid)

**Name:** Vlid (Valid Blockchain native token)  
**Supply:** 33 billion (fixed, no minting, no burning)  
**Decimals:** 9 (standard precision for microtransactions)  
**Distribution:** No planned ICO. Vested allocations, validator tester rewards, validator rewards pool, ecosystem grants, P2P filesharing protocol rewards  
**Status:** In active development

## License

MIT License - See LICENSE file for full text.

Copyright (c) 2024 Rook

## Acknowledgements

Built and maintained solo by developer, Rook. Contact me directly with questions or inquiries. If you run this successfully on your machines and it behaves, open an issue.
