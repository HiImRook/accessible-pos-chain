# Valid Testnet

Live testnet for the Valid Blockchain network. This branch mirrors `valid-blockchain` exactly — same consensus, same minting, same tokenomics. Test VLid has no monetary value.

Join the Discord for real-time bug fixes, testnet discussion, support, reward tracking, and announcements.

**Discord:** https://discord.gg/2SP383cJs9

## What You Are Testing

- Validator connectivity and peer discovery
- TPI consensus under real network conditions
- Block production and reward minting
- Transaction submission and fee handling
- Network resilience across varied hardware and regions
- Snapshot persistance backup
- Racer conditions and edge cases

## Requirements

- Rust 1.70+ ([Install Rust](https://rustup.rs/))
- 2 GB RAM minimum
- Port 4000 open for P2P

## Quick Start
```bash
git clone -b valid-testnet https://github.com/HiImRook/accessible-pos-chain.git
cd accessible-pos-chain
cargo build --release
```

Bootstrap peer addresses are posted in Discord before each testnet launch.

## Getting Test VLid

**Coming Soon** Test VLid is earned through block production on the testnet — the same minting process as mainnet. A faucet bot is available in the `#faucet` channel on Discord to get you started. The initial testnet supply will start with the amount set aside for mainet genesis allocations(33,000).

## Reporting Issues

Use the testnet forum and channels on Discord. Include:
- What happened
- What you expected
- Your hardware specs
- Your region and connection type
- Any relevant logs
- Feel free to ding me or join me in voice chat if I'm online to discuss

## Known Limitations

- No state persistence(coming very soon) — node restart loses chain state(all restarts will be intentional and documented).
- No memory pruning yet — long-running nodes will grow in RAM over time(10-45 mb ram without pruning) 
- No TLS on P2P connections

These are active development items, not bugs to report.

## License

MIT License — See LICENSE file

Copyright (c) 2024-2026 Rook

---

**"Valid, empowering Communities with Sovereign, Decentralized, and Accessible In-Memory Tools, Fostering Freedom and Transparency Through Open-Source Self-Reliance."**
