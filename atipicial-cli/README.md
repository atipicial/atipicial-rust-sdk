<!-- Atipicial Chain · sovereign Layer-1 for decentralized AGI coordination -->
<!-- 👑 Founded & engineered by xmoohad — Blockchain Scientist · Computer Programmer -->

# Atipicial CLI

A command-line interface for interacting with the Atipicial blockchain, built on the AtipicialRust SDK (`atipicial`).

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

## Overview

Atipicial CLI wraps common SDK workflows into a single tool: network connectivity checks, wallet management, contract operations, token helpers, NFT operations, AtipicialFs gateway access, and project generation templates.

## Installation

### From Source

```bash
# Clone the repository
git clone https://github.com/R3E-Network/AtipicialRust.git
cd AtipicialRust

# Build the CLI tool
cargo build --release -p atipicial-cli

# Run the CLI
./target/release/atipicial-cli --help
```

### Using Cargo

```bash
# If/when published to crates.io:
cargo install atipicial-cli
```

## Quick Start

```bash
# Show top-level help
atipicial-cli --help

# Initialize configuration (optional; creates a config file in your OS config dir)
atipicial-cli init

# Create a wallet file (prompts for password unless provided)
atipicial-cli wallet create --path my-wallet.json

# Connect to a network (interactive if omitted)
atipicial-cli network connect --network testnet

# Inspect network state
atipicial-cli network status
atipicial-cli network block

# Token/DeFi helpers
atipicial-cli de-fi token ATC
atipicial-cli de-fi balance GAS NZKvXidwBhnV8rNXh2eXtpm5bH1rkofaDz

# Check AtipicialFs connection status
atipicial-cli fs status

# Generate a new project from templates
atipicial-cli generate --list
atipicial-cli generate --template aep17-token my-token
```

## Command Reference

Run `atipicial-cli --help` (and `atipicial-cli <command> --help`) for the full set of flags and subcommands.

### Network

- `atipicial-cli network connect`: connect to an RPC endpoint / named network
- `atipicial-cli network status`: show basic network information
- `atipicial-cli network peers`: list peers (requires a working connection)
- `atipicial-cli network block`: fetch latest (or specified) block

### Wallet

- `atipicial-cli wallet create`, `open`, `backup`, `restore`, `hd-wallet`
- `atipicial-cli wallet send`, `balance`, `import-address`, `create-multisig`, `set-default`, and `sign-message`

### Contracts

- `atipicial-cli contract deploy`, `update`, `invoke`, `list-native-contracts`

### DeFi

- `atipicial-cli de-fi token` and `atipicial-cli de-fi balance` for token metadata/balance queries
- `atipicial-cli de-fi transfer` for AEP-17 token transfers

### AtipicialFs

- `atipicial-cli fs ...`: endpoints, container, object, status
- `atipicial-cli atipicial-fs ...`: advanced AtipicialFs commands (acl/config/status)

```bash
# List all available AtipicialFs endpoints for mainnet
atipicial-cli fs endpoints list

# Test connection to a specific endpoint
atipicial-cli fs endpoints test --endpoint grpc.mainnet.fs.atipicial.com:8082

# Get detailed information about an endpoint
atipicial-cli fs endpoints info --endpoint grpc.mainnet.fs.atipicial.com:8082

# Create a container
atipicial-cli fs container create --config container-config.json

# Upload a file
atipicial-cli fs object put --container CID --file path/to/file

# Download a file
atipicial-cli fs object get --container CID --id OID --output path/to/save
```

## Configuration

Atipicial CLI uses a configuration file to store settings like network preferences, RPC endpoints, and more. You can initialize the configuration with:

```bash
atipicial-cli init [--path /custom/path/config.json]
```

The default location for the configuration file is in your system's config directory under `atipicial-cli/config.json`.

## Testing

The Atipicial CLI includes comprehensive automated tests to ensure functionality and help with development.

### Running Tests

To run CLI tests:

```bash
cargo test -p atipicial-cli
```

### Test Structure

- **Unit Tests**: Test individual functions and components
- **Integration Tests**: Test the CLI commands from a user perspective
  - `defi_tests.rs`: Tests for DeFi and well-known contract commands
  - `fs_tests.rs`: Tests for AtipicialFs storage operations
  - `blockchain_tests.rs`: Tests for blockchain query commands
  - `wallet_tests.rs`: Tests for wallet management commands

### Writing New Tests

When adding new features to the CLI, follow this pattern for testing:

1. Create unit tests for new functions in the source files
2. Add integration tests in the appropriate test module
3. Run the tests to verify functionality
4. Ensure both success cases and error handling are tested

## Development

### Building from Source

```bash
cargo build [--release]
```

## License

MIT License

## Credits

Developed by the R3E Network team

---
Copyright © 2020-2025 R3E Network. All rights reserved.

---

> **Atipicial Chain** — sovereign Layer-1 for decentralized AGI coordination.
> 👑 Founded & engineered by **xmoohad** — Blockchain Scientist · Computer Programmer.
> `ATC` Atipicial Coin · `ATD` AtipicialDollar · addresses begin with **A**
