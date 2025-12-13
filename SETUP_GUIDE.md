# SUPER SIMPLE SETUP GUIDE

Get validators running in minutes with copy/paste commands.

---

# SETUP (One Time)

## Install Ubuntu

**Download Ubuntu 22.04 LTS:**
https://ubuntu.com/download/desktop

**Install on:**
- **Physical computer (Linux OS)** - Dual boot or replace existing OS
- **Virtual machine (Windows OS)** - VirtualBox or VMware running Ubuntu

**After install, open terminal:** Ctrl+Alt+T

---

## Install Dependencies

```bash
# Update system
sudo apt update && sudo apt upgrade -y

# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env

# Install build tools
sudo apt install -y build-essential git screen

# Verify
rustc --version
git --version
```

**What is screen?** A Linux tool that keeps programs running in the background even if you close the terminal.

---

## Build Blockchain

```bash
# Clone repository
git clone https://github.com/HiImRook/accessible-pos-chain.git
cd accessible-pos-chain

# Build (takes 5-10 minutes)
cargo build --release
```

**Setup complete!** Now choose a test below.

---

# TEST 1: Single Validator

Run one validator to verify blockchain works.

### Run Validator

```bash
cd accessible-pos-chain

# Create config file
cp config.example.toml config.toml

# Run validator
./target/release/pos-chain
```

**Expected output:**
```
Listening on 0.0.0.0:8080
RPC server listening on 0.0.0.0:3000
[HH:MM:SS] Slot 1: Producer validator_1 (0 tx)
[HH:MM:SS] Slot 2: Producer validator_1 (0 tx)
```

**Success!** New block every 10 seconds.

Let validator run for a few minutes.

**Stop validator:** Press Ctrl+C

**Test complete.** You successfully ran a blockchain validator and produced blocks.

---

# TEST 2: Two Validators

Run two validators to test peer discovery and consensus.

## Step 1: Create Config Files

```bash
cd accessible-pos-chain

# Create 2 configs
cp config.example.toml config1.toml
cp config.example.toml config2.toml
```

---

## Step 2: Edit Configs

**Validator 1 (Bootstrap):**
```bash
nano config1.toml
```

Change these lines:
```toml
listen_addr = "0.0.0.0:8080"
rpc_addr = "0.0.0.0:3000"
bootstrap_nodes = []
```

Save: Ctrl+X, Y, Enter

---

**Validator 2:**
```bash
nano config2.toml
```

Change these lines:
```toml
listen_addr = "0.0.0.0:8081"
rpc_addr = "0.0.0.0:3001"
bootstrap_nodes = ["127.0.0.1:8080"]
```

Save: Ctrl+X, Y, Enter

---

## Step 3: Start Validators

**Start Validator 1:**
```bash
screen -S val1
./target/release/pos-chain --config config1.toml
```

Press Ctrl+A, then D (validator keeps running)

---

**Start Validator 2:**
```bash
screen -S val2
./target/release/pos-chain --config config2.toml
```

Press Ctrl+A, then D

---

## Step 4: Verify Connection

**Check logs:**
```bash
screen -r val1
```

Look for:
```
[HH:MM:SS] Handshake from peer-abc123 (1 peers)
```

Press Ctrl+A, then D to detach

**Success!** Validators discovered each other and are sharing blocks.

Let validators run for 10-15 minutes.

**Stop validators:**
```bash
killall pos-chain
```

**Test complete.** You ran a multi-validator network with peer discovery and consensus.

---

# TEST 3: Five or More Validators

Run 5+ validators to test network consensus at scale.

## Step 1: Create Config Files

```bash
cd accessible-pos-chain

# Create 5 configs (or more)
for i in {1..5}; do cp config.example.toml config$i.toml; done
```

---

## Step 2: Edit Configs

**Validator 1 (Bootstrap):**
```bash
nano config1.toml
```

Change these lines:
```toml
listen_addr = "0.0.0.0:8080"
rpc_addr = "0.0.0.0:3000"
bootstrap_nodes = []
```

Save: Ctrl+X, Y, Enter

---

**Validator 2:**
```bash
nano config2.toml
```

Change these lines:
```toml
listen_addr = "0.0.0.0:8081"
rpc_addr = "0.0.0.0:3001"
bootstrap_nodes = ["127.0.0.1:8080"]
```

Save: Ctrl+X, Y, Enter

---

**Validator 3:**
```bash
nano config3.toml
```

Change these lines:
```toml
listen_addr = "0.0.0.0:8082"
rpc_addr = "0.0.0.0:3002"
bootstrap_nodes = ["127.0.0.1:8080"]
```

Save: Ctrl+X, Y, Enter

---

**Validator 4:**
```bash
nano config4.toml
```

Change these lines:
```toml
listen_addr = "0.0.0.0:8083"
rpc_addr = "0.0.0.0:3003"
bootstrap_nodes = ["127.0.0.1:8080"]
```

Save: Ctrl+X, Y, Enter

---

**Validator 5:**
```bash
nano config5.toml
```

Change these lines:
```toml
listen_addr = "0.0.0.0:8084"
rpc_addr = "0.0.0.0:3004"
bootstrap_nodes = ["127.0.0.1:8080"]
```

Save: Ctrl+X, Y, Enter

---

## Step 3: Start Validators

**Start Validator 1:**
```bash
screen -S val1
./target/release/pos-chain --config config1.toml
```

Press Ctrl+A, then D

---

**Start Validator 2:**
```bash
screen -S val2
./target/release/pos-chain --config config2.toml
```

Press Ctrl+A, then D

---

**Start Validator 3:**
```bash
screen -S val3
./target/release/pos-chain --config config3.toml
```

Press Ctrl+A, then D

---

**Start Validator 4:**
```bash
screen -S val4
./target/release/pos-chain --config config4.toml
```

Press Ctrl+A, then D

---

**Start Validator 5:**
```bash
screen -S val5
./target/release/pos-chain --config config5.toml
```

Press Ctrl+A, then D

---

## Step 4: Verify Network

**Check all validators running:**
```bash
screen -ls
```

Should show: val1, val2, val3, val4, val5

**Check a validator's logs:**
```bash
screen -r val1
```

Look for:
```
[HH:MM:SS] Handshake from peer-abc123 (4 peers)
[HH:MM:SS] Block from peer-def456: slot 15
```

Press Ctrl+A, then D to detach

**Success!** Test complete.

Let validators run for 24-48 hours.

**Stop validators:**
```bash
killall pos-chain
```

**Test complete.** You ran a distributed blockchain network with multiple validators achieving consensus.

---

# MONITORING

## Check Blockchain Status

**Check peers:**
```bash
curl -X POST http://localhost:3000 \
  -H "Content-Type: application/json" \
  -d '{"jsonrpc":"2.0","method":"get_peers","params":{},"id":1}'
```

**Check balance:**
```bash
curl -X POST http://localhost:3000 \
  -H "Content-Type: application/json" \
  -d '{
    "jsonrpc":"2.0",
    "method":"get_balance",
    "params":{"address":"validator_1"},
    "id":1
  }'
```

**Check block:**
```bash
curl -X POST http://localhost:3000 \
  -H "Content-Type: application/json" \
  -d '{
    "jsonrpc":"2.0",
    "method":"get_block",
    "params":{"slot":100},
    "id":1
  }'
```

---

# TROUBLESHOOTING

## Port Already in Use

**Error:** `Address already in use`

**Fix:**
```bash
sudo lsof -i :8080
kill -9 <PID>
```

Or change port in config file.

---

## Validators Not Connecting

**Check validator is running:**
```bash
screen -ls
```

**Check logs:**
```bash
screen -r val1
```

Look for: `Handshake from peer-abc123`

If no handshakes, check bootstrap_nodes in config files.

---

## Screen Commands

```bash
# List all screens
screen -ls

# Attach to screen
screen -r val1

# Detach (while inside screen)
# Press: Ctrl+A then D

# Kill a screen
screen -X -S val1 quit

# Kill all screens
killall screen
```

---

# QUICK REFERENCE

## Port Allocation

| Validator | P2P Port | RPC Port |
|-----------|----------|----------|
| 1 | 8080 | 3000 |
| 2 | 8081 | 3001 |
| 3 | 8082 | 3002 |
| 4 | 8083 | 3003 |
| 5 | 8084 | 3004 |

## Common Commands

```bash
# Build
cargo build --release

# Run single validator
./target/release/pos-chain

# Run with config
./target/release/pos-chain --config config2.toml

# Check validators
screen -ls

# View logs
screen -r val1

# Stop all
killall pos-chain
```

---

# WHAT'S NEXT

After running Test 3 for 24-48 hours:

1. **Test transactions** - Use CLI wallet to send test transactions
2. **Scale up** - Add more validators (10, 20, 50+)
3. **Report results** - Open an issue on GitHub with your findings

---

**Need help?** https://github.com/HiImRook/accessible-pos-chain/issues
