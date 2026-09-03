<!-- Atipicial Chain · sovereign Layer-1 for smart contracts and digital assets -->
<!-- 👑 Founded & engineered by xmoohad — Blockchain Scientist · Computer Programmer -->

# Atipicial CLI Usage Examples

This document provides practical examples for using Atipicial CLI commands in different scenarios.

## Wallet Operations

### Creating and Managing Wallets

Create a new wallet:
```bash
atipicial-cli wallet create --name my-wallet
```

Create a wallet with a specific password:
```bash
atipicial-cli wallet create --name my-wallet --password mysecurepassword
```

Open an existing wallet:
```bash
atipicial-cli wallet open --name my-wallet
```

List all addresses in a wallet:
```bash
atipicial-cli wallet addresses --name my-wallet
```

Create a new address in the wallet:
```bash
atipicial-cli wallet address-new --name my-wallet
```

Export wallet in AEP-6 format:
```bash
atipicial-cli wallet export --name my-wallet --format aep6 --output my-wallet-backup.json
```

Import a wallet from AEP-6 format:
```bash
atipicial-cli wallet import --input my-wallet-backup.json --format aep6 --name restored-wallet
```

### Working with Assets

Check wallet balance:
```bash
atipicial-cli wallet balance --name my-wallet
```

Check balance for a specific address:
```bash
atipicial-cli wallet balance --name my-wallet --address NXV7ZhHiyME9ipZzXELiWXiVX1W5B7o2Lz
```

Transfer ATC to another address:
```bash
atipicial-cli wallet transfer --name my-wallet --to NXV7ZhHiyME9ipZzXELiWXiVX1W5B7o2Lz --asset ATC --amount 5
```

Transfer GAS:
```bash
atipicial-cli wallet transfer --name my-wallet --to NXV7ZhHiyME9ipZzXELiWXiVX1W5B7o2Lz --asset GAS --amount 2.5
```

Transfer a specific AEP-17 token:
```bash
atipicial-cli wallet transfer --name my-wallet --to NXV7ZhHiyME9ipZzXELiWXiVX1W5B7o2Lz --asset 0xde5f57d430d3dece511cf975a8d37703f19f90c7 --amount 100
```

### Multi-signature Wallets

Create a multi-signature wallet:
```bash
atipicial-cli wallet create-multisig --name multi-wallet --min-signatures 2 --pubkeys 03a301d7384a0124e3737ff7c9e3377628f3f1a7c9df549d9585c21bc7cfd2a4c9,02a7834be9b32e2981d157cb5bbd3acb42cfd11ea5c3b10224d7a44e98c5910f1b,0214baf0ceea3a66f17e7e1e839ea25fd8bed6cd82e6bb6e68250189065f44ff01
```

Sign a multi-signature transaction:
```bash
atipicial-cli wallet multi-sign --name my-wallet --tx-hex 800000012da7d8bab9b5aeaf8e0c8b6e41e35c82047c37e7d11b7eabe2f00000000000000000001ccbd889c8e4e16af7f941dc29520a41f65be3461e2cc9ed532d51a5f7f6cd8250000029b7cffdaa674beae0f930ebe6085af9093e5fe56b34a5c220ccdcf6efc336fc500c2eb0b00000000b4e2105c9ce97d59cc0a84bdcd3b38c53e20b43b9b7cffdaa674beae0f930ebe6085af9093e5fe56b34a5c220ccdcf6efc336fc500743ba40b00000000e79f2cf0f19f70373d8a975c51ce0d43d57f5de0141408810240c54faf3f58b226886d33fc197b6eb73d35882f5557e1097f1a40c2c1aa9b09a8a81f56cb22d729fa68dc3d0e5a1c780f5f2e0c72ec1221c6e42b9aa232102a7834be9b32e2981d157cb5bbd3acb42cfd11ea5c3b10224d7a44e98c5910f1bac
```

Complete a multi-signature transaction:
```bash
atipicial-cli wallet multi-complete --tx-hex 800000012da7d8bab9b5aeaf8e0c8b6e41e35c82047c37e7d11b7eabe2f00000000000000000001ccbd889c8e4e16af7f941dc29520a41f65be3461e2cc9ed532d51a5f7f6cd8250000029b7cffdaa674beae0f930ebe6085af9093e5fe56b34a5c220ccdcf6efc336fc500c2eb0b00000000b4e2105c9ce97d59cc0a84bdcd3b38c53e20b43b9b7cffdaa674beae0f930ebe6085af9093e5fe56b34a5c220ccdcf6efc336fc500743ba40b00000000e79f2cf0f19f70373d8a975c51ce0d43d57f5de0141408810240c54faf3f58b226886d33fc197b6eb73d35882f5557e1097f1a40c2c1aa9b09a8a81f56cb22d729fa68dc3d0e5a1c780f5f2e0c72ec1221c6e42b9aa232102a7834be9b32e2981d157cb5bbd3acb42cfd11ea5c3b10224d7a44e98c5910f1bac --signatures 4140bd01edc0c5d43b0fb68d5a39bd7878a8e0a17b75e82a5a2a62f3c48f0a968f7f405c78c7c7adc54b45ea1c83431d14b2ee496a6301c95238ea5d1628aad777,410aa92fa4a3557bc1e37bc20fb49d61697d2efeb87ce97dc5c2f581ef65bedd37e8dfd8e2bd4c3efadfd4ceea2d49c69c0e35fa85c4e8ab2827b5e868db00c94
```

## Blockchain Operations

### Querying Blockchain Information

Get general blockchain information:
```bash
atipicial-cli blockchain info
```

Get current block height:
```bash
atipicial-cli blockchain height
```

Get block by index:
```bash
atipicial-cli blockchain block --index 12345
```

Get block by hash:
```bash
atipicial-cli blockchain block --hash 0x6b4b9ecd57ca4a17bd5f7a6c25be98d678b347c271839be5db47bca98a5b2fbb
```

Get transaction details:
```bash
atipicial-cli blockchain tx --hash 0x9e3bdb1d7df922c2db7c9ef90edb0086d8243ba7e3f740d0f0984cd25c577b91
```

Get asset information:
```bash
atipicial-cli blockchain asset --id 0xd2a4cff31913016155e38e474a2c06d08be276cf
```

### Blockchain Monitoring

Monitor new blocks:
```bash
atipicial-cli blockchain monitor-blocks
```

Monitor new transactions:
```bash
atipicial-cli blockchain monitor-transactions
```

Monitor mempool:
```bash
atipicial-cli blockchain monitor-mempool
```

## Network Operations

### Network Management

Get network status:
```bash
atipicial-cli network status
```

List connected nodes:
```bash
atipicial-cli network nodes
```

Switch to TestNet:
```bash
atipicial-cli network switch --network testnet
```

Switch to MainNet:
```bash
atipicial-cli network switch --network mainnet
```

Add a custom RPC node:
```bash
atipicial-cli network add-node --url http://seed1.example.org:10332 --name custom-node
```

Set a node as default:
```bash
atipicial-cli network set-default --name custom-node
```

Ping a specific node:
```bash
atipicial-cli network ping --url http://seed1.atipicial.com:10332
```

## Smart Contract Operations

### Contract Deployment and Invocation

Deploy a smart contract:
```bash
atipicial-cli contract deploy --path ./contract.aef --manifest ./contract.manifest.json --wallet my-wallet
```

Invoke a contract method:
```bash
atipicial-cli contract invoke --contract 0x1a70eac53f5882e40dd90f55463cce31a9f72cd4 --method balanceOf --params '[{"type":"Hash160","value":"0x7f35401c25b21d99b160c2c22f6c75c9f25574e4"}]' --wallet my-wallet
```

Invoke a contract method with multiple parameters:
```bash
atipicial-cli contract invoke --contract 0x1a70eac53f5882e40dd90f55463cce31a9f72cd4 --method transfer --params '[{"type":"Hash160","value":"0x7f35401c25b21d99b160c2c22f6c75c9f25574e4"},{"type":"Hash160","value":"0x9f82866c82783d79c1d7c1b25e4b50d7f5ab67d1"},{"type":"Integer","value":"100"},{"type":"Any","value":null}]' --wallet my-wallet
```

### Contract Inspection

Get contract information:
```bash
atipicial-cli contract info --contract 0x1a70eac53f5882e40dd90f55463cce31a9f72cd4
```

Get contract manifest:
```bash
atipicial-cli contract manifest --contract 0x1a70eac53f5882e40dd90f55463cce31a9f72cd4
```

List all methods in a contract:
```bash
atipicial-cli contract methods --contract 0x1a70eac53f5882e40dd90f55463cce31a9f72cd4
```

Query contract storage:
```bash
atipicial-cli contract storage --contract 0x1a70eac53f5882e40dd90f55463cce31a9f72cd4 --key 74657374
```

Query contract storage with a prefix:
```bash
atipicial-cli contract storage --contract 0x1a70eac53f5882e40dd90f55463cce31a9f72cd4 --prefix 7465
```

### Contract Testing

Test a contract method (without broadcasting):
```bash
atipicial-cli contract test-invoke --contract 0x1a70eac53f5882e40dd90f55463cce31a9f72cd4 --method balanceOf --params '[{"type":"Hash160","value":"0x7f35401c25b21d99b160c2c22f6c75c9f25574e4"}]'
```

## DeFi Operations

### Token Swaps and Liquidity

Perform a token swap:
```bash
atipicial-cli defi swap --from ATC --to GAS --amount 10 --max-slippage 1.0 --wallet my-wallet
```

Check available liquidity pools:
```bash
atipicial-cli defi pools --platform flamingo
```

Add liquidity to a pool:
```bash
atipicial-cli defi add-liquidity --token-a ATC --token-b GAS --amount-a 5 --amount-b 10 --platform flamingo --wallet my-wallet
```

Remove liquidity:
```bash
atipicial-cli defi remove-liquidity --pool-id 1 --amount 100 --platform flamingo --wallet my-wallet
```

### Staking and Farming

Stake tokens:
```bash
atipicial-cli defi stake --token FLM --amount 1000 --platform flamingo --duration 30 --wallet my-wallet
```

Check staking positions:
```bash
atipicial-cli defi staking-positions --wallet my-wallet
```

Claim rewards:
```bash
atipicial-cli defi claim-rewards --platform flamingo --wallet my-wallet
```

Check available farming pools:
```bash
atipicial-cli defi farming-pools --platform flamingo
```

Start yield farming:
```bash
atipicial-cli defi farm --pool-id 2 --amount 500 --platform flamingo --wallet my-wallet
```

## Configuration Management

Initialize configuration:
```bash
atipicial-cli init
```

Initialize configuration at a specific path:
```bash
atipicial-cli init --path ~/.config/atipicial-cli/custom-config.json
```

Update RPC endpoint:
```bash
atipicial-cli config set --key network.rpc.mainnet --value http://custom-seed.atipicial.com:10332
```

Change default network:
```bash
atipicial-cli config set --key network.default --value testnet
```

Get a specific configuration value:
```bash
atipicial-cli config get --key network.default
```

Export configuration:
```bash
atipicial-cli config export --output ~/atipicial-config-backup.json
```

Import configuration:
```bash
atipicial-cli config import --input ~/atipicial-config-backup.json
```

## Scripting Examples

### Bash Script for Daily Token Transfer

```bash
#!/bin/bash
# daily_transfer.sh - Script to transfer tokens daily
# Usage: ./daily_transfer.sh <wallet_name> <recipient_address> <amount>

WALLET=$1
RECIPIENT=$2
AMOUNT=$3

# Validate inputs
if [ -z "$WALLET" ] || [ -z "$RECIPIENT" ] || [ -z "$AMOUNT" ]; then
    echo "Usage: ./daily_transfer.sh <wallet_name> <recipient_address> <amount>"
    exit 1
fi

echo "Performing daily transfer..."
atipicial-cli wallet transfer --name "$WALLET" --to "$RECIPIENT" --asset GAS --amount "$AMOUNT"

if [ $? -eq 0 ]; then
    echo "Transfer successful!"
    exit 0
else
    echo "Transfer failed!"
    exit 1
fi
```

### Python Script for Contract Monitoring

```python
#!/usr/bin/env python3
# monitor_contract.py - Script to monitor contract storage changes
# Requires: subprocess module

import subprocess
import json
import time
import sys

CONTRACT_HASH = sys.argv[1] if len(sys.argv) > 1 else "0x1a70eac53f5882e40dd90f55463cce31a9f72cd4"
CHECK_INTERVAL = 60  # seconds

def get_contract_storage():
    result = subprocess.run(
        ["atipicial-cli", "contract", "storage", "--contract", CONTRACT_HASH, "--output", "json"],
        capture_output=True, text=True
    )
    return json.loads(result.stdout)

previous_storage = get_contract_storage()
print(f"Starting monitoring for contract {CONTRACT_HASH}...")

while True:
    time.sleep(CHECK_INTERVAL)
    current_storage = get_contract_storage()
    
    # Compare storage states
    for key in current_storage:
        if key not in previous_storage:
            print(f"New key added: {key} = {current_storage[key]}")
        elif previous_storage[key] != current_storage[key]:
            print(f"Key changed: {key} = {current_storage[key]} (was {previous_storage[key]})")
    
    for key in previous_storage:
        if key not in current_storage:
            print(f"Key removed: {key}")
    
    previous_storage = current_storage
```

---

> **Atipicial Chain** — sovereign Layer-1 for smart contracts and digital assets.
> 👑 Founded & engineered by **xmoohad** — Blockchain Scientist · Computer Programmer.
> `ATC` Atipicial Coin · `ATD` AtipicialDollar · addresses begin with **A**
