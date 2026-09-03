<!-- Atipicial Chain · sovereign Layer-1 for smart contracts and digital assets -->
<!-- 👑 Founded & engineered by xmoohad — Blockchain Scientist · Computer Programmer -->

# Atipicial Famous Contracts Examples

This directory contains examples demonstrating how to interact with popular contracts and protocols on the Atipicial blockchain using modern AtipicialRust SDK patterns.

## ✅ Production-Ready Examples

All examples have been updated to use modern AtipicialRust SDK APIs and compile successfully.

### Available Examples

| Example | Description | Status |
|---------|-------------|--------|
| **query_atipicial.rs** | Query ATC token information and balances | ✅ Working |
| **query_gas.rs** | Query GAS token information and balances | ✅ Working |
| **flamingo_finance.rs** | Flamingo Finance DeFi protocol integration | ✅ Simplified |
| **grandshare.rs** | GrandShare governance contract interaction | ✅ Simplified |
| **atipicialburger_atipicial.rs** | AtipicialBurger DeFi protocol example | ✅ Simplified |
| **atipicialcompound.rs** | AtipicialCompound lending protocol example | ✅ Simplified |

## 🚀 Quick Start

### Prerequisites

1. **Rust Environment**: Ensure you have Rust 1.70+ installed
2. **Atipicial TestNet**: Examples connect to Atipicial TestNet by default
3. **Dependencies**: All required dependencies are included in the workspace

### Running Examples

```bash
# Navigate to the famous contracts directory
cd examples/atipicial_famous_contracts

# Run any example
cargo run --example query_atipicial
cargo run --example query_gas
cargo run --example flamingo_finance
cargo run --example grandshare
cargo run --example atipicialburger_atipicial
cargo run --example atipicialcompound

# Check compilation of all examples
cargo check --examples
```

## 📋 Example Categories

### **Token Query Examples**
- **query_atipicial.rs**: Demonstrates how to query ATC token information using modern RPC client patterns
- **query_gas.rs**: Shows GAS token balance and information retrieval

### **DeFi Protocol Examples** 
- **flamingo_finance.rs**: Basic structure for Flamingo Finance integration
- **atipicialburger_atipicial.rs**: Template for AtipicialBurger protocol interaction
- **atipicialcompound.rs**: Framework for AtipicialCompound lending protocol

### **Governance Examples**
- **grandshare.rs**: Demonstrates governance contract interaction patterns

## 🔧 Modern API Patterns

All examples follow these modern AtipicialRust SDK patterns:

### **Standard Imports**
```rust
use atipicial::prelude::*;
use atipicial::atipicial_clients::APITrait;
use std::str::FromStr;
```

### **Provider Setup**
```rust
let provider = providers::HttpProvider::new("https://testnet1.atipicial.com:443/")?;
let client = providers::RpcClient::new(provider);
```

### **Contract Interaction**
```rust
let result = client.invoke_function(
    &contract_hash,
    "methodName",
    Some(parameters),
    None,
    None
).await?;
```

### **Result Parsing**
```rust
let value = result.stack
    .first()
    .and_then(|item| item.as_string())
    .unwrap_or_default();
```

## 🏗️ Development Notes

- Examples use live MainNet/TestNet contract hashes where available (e.g., Flamingo Finance).
- Validate contract hashes/methods against current deployments before broadcasting transactions.
- Add proper signing/broadcast logic when moving from read-only queries to state-changing calls.

## 🧪 Testing

### **Compilation Tests**
```bash
# Test all examples compile successfully
cargo check --examples

# Test specific example
cargo check --example query_atipicial
```

### **Runtime Tests**
```bash
# Run examples (connects to TestNet)
cargo run --example query_atipicial
```

## 🔗 Network Configuration

Examples default to Atipicial TestNet:
- **TestNet RPC**: `https://testnet1.atipicial.com:443/`
- **MainNet RPC**: `https://mainnet1.atipicial.com:443/` (change in code)

## 📚 Additional Resources

- [Atipicial Documentation](https://docs.atipicial.com/)
- [AtipicialRust SDK Documentation](../../README.md)
- [Atipicial Contract Examples](../atipicial_smart_contracts/)
- [AEP-17 Token Examples](../atipicial_aep17_tokens/)

## 🤝 Contributing

To contribute improvements to these examples:

1. **Maintain Compatibility**: Ensure all examples compile with current AtipicialRust SDK
2. **Follow Patterns**: Use established import and API patterns
3. **Add Documentation**: Include clear comments and usage examples
4. **Test Thoroughly**: Verify examples work on both TestNet and MainNet

## ⚠️ Security Notice

- **TestNet Only**: Examples are configured for TestNet by default
- **No Private Keys**: Never commit private keys or sensitive information
- **Validation**: Always validate contract addresses and method signatures
- **Production Use**: Thoroughly test any modifications before mainnet deployment

---

**Status**: ✅ All examples compile successfully with modern AtipicialRust SDK (Last updated: December 2024)

---

> **Atipicial Chain** — sovereign Layer-1 for smart contracts and digital assets.
> 👑 Founded & engineered by **xmoohad** — Blockchain Scientist · Computer Programmer.
> `ATC` Atipicial Coin · `ATD` AtipicialDollar · addresses begin with **A**
