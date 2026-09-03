<!-- Atipicial Chain · sovereign Layer-1 for smart contracts and digital assets -->
<!-- 👑 Founded & engineered by xmoohad — Blockchain Scientist · Computer Programmer -->

# AtipicialRust Examples

This directory contains comprehensive, production-ready examples demonstrating how to use the AtipicialRust SDK for various Atipicial blockchain operations.

## 🎯 **Quick Start**

Choose an example category based on what you want to accomplish:

- **New to Atipicial?** → Start with `atipicial_nodes` for basic connectivity
- **Building a wallet?** → Check `atipicial_wallets` for account management  
- **Working with tokens?** → Explore `atipicial_aep17_tokens` for token operations
- **Smart contracts?** → See `atipicial_smart_contracts` for contract interaction
- **Deploying contracts?** → Look at `atipicial_contracts` for deployment guide

## 📚 **Example Categories**

### **Core Atipicial Examples** (✅ Production Ready)

| Category | Description | Key Features |
|----------|-------------|--------------|
| **[atipicial_nodes](atipicial_nodes/)** | Node connectivity and blockchain queries | Network health, block exploration, multi-endpoint testing |
| **[atipicial_wallets](atipicial_wallets/)** | Wallet and account management | Account creation, encryption, backup/recovery, multi-sig |
| **[atipicial_transactions](atipicial_transactions/)** | Transaction creation and broadcasting | GAS/ATC transfers, multi-call transactions, fee estimation |
| **[atipicial_smart_contracts](atipicial_smart_contracts/)** | Smart contract interaction | Read-only calls, state changes, best practices |
| **[atipicial_contracts](atipicial_contracts/)** | Contract deployment guide | AEF files, manifests, deployment workflow |
| **[atipicial_aep17_tokens](atipicial_aep17_tokens/)** | AEP-17 token operations | Token info, balance queries, transfer scripts |

### **Advanced Examples** 

| Category | Description | Status |
|----------|-------------|---------|
| **[atipicial_famous_contracts](atipicial_famous_contracts/)** | Well-known Atipicial contracts | ✅ Production Ready |
| **[atipicial_nns](atipicial_nns/)** | Atipicial Name Service | 🔄 Needs updating |
| **[atipicial_x](atipicial_x/)** | EVM compatibility layer | 🔄 Needs updating |

### **General Examples** (📝 Legacy)

| Category | Description | Status |
|----------|-------------|---------|
| **[wallets](wallets/)** | General wallet examples | 📝 Legacy |
| **[transactions](transactions/)** | General transaction examples | 📝 Legacy |
| **[providers](providers/)** | Provider examples | 📝 Legacy |
| **[contracts](contracts/)** | Contract examples | 📝 Legacy |

## 🚀 **Running Examples**

### **Method 1: Run Specific Example**
```bash
# Navigate to example category
cd examples/atipicial_nodes

# Run specific example
cargo run --example connect_to_node
```

### **Method 2: Run from Workspace Root**
```bash
# From project root
cargo run --example connect_to_node -p atipicial_nodes_examples
```

### **Method 3: Test Compilation**
```bash
# Test all examples compile
cd examples/atipicial_nodes && cargo check
```

## 🛠 **Example Features**

### **Production-Ready Code**
- ✅ Proper error handling and edge cases
- ✅ Comprehensive documentation and comments
- ✅ Real-world usage patterns
- ✅ Security best practices
- ✅ Network connectivity resilience

### **Educational Content**
- 📚 Step-by-step explanations
- 💡 Best practices and security tips
- 🔧 Common pitfalls and solutions
- 📊 Performance considerations
- 🎯 Real-world use cases

### **API Compatibility**
- ✅ Uses latest AtipicialRust production APIs
- ✅ Follows current Atipicial protocols
- ✅ Compatible with TestNet and MainNet
- ✅ Proper type safety and validation

## 📋 **Prerequisites**

### **Development Environment**
- **Rust**: 1.70+ (2021 edition)
- **Platform**: macOS, Linux, Windows
- **Network**: Internet connection for blockchain interaction

### **Atipicial Knowledge**
- Basic understanding of blockchain concepts
- Familiarity with Atipicial architecture
- Knowledge of smart contract principles (for contract examples)

### **Optional Tools**
- **Atipicial CLI**: For advanced blockchain operations
- **Atipicial-Express**: For local development blockchain
- **TestNet Wallet**: For testing with real network

## 🌐 **Network Configuration**

Examples are configured for **Atipicial TestNet** by default:
- **TestNet RPC**: `https://testnet1.atipicial.com:443/`
- **Explorer**: [TestNet Atipicialtube](https://testnet.atipicialtube.io/)
- **Faucet**: Get test GAS/ATC from community faucets

To use **MainNet**, update RPC endpoints in examples:
- **MainNet RPC**: `https://mainnet1.atipicial.com:443/`
- **Explorer**: [MainNet Atipicialtube](https://atipicialtube.io/)

Environment toggles:
- Set `ATC_RPC_URL` to point examples at a specific node (otherwise they use TestNet defaults or skip live calls when possible).
- Some examples include mock/offline paths; they will print a hint when a live RPC URL is required.
- Feature flags: enable `ws` for WebSocket transport and `ipc` for IPC transport when running examples that use those clients.

## 🔧 **Troubleshooting**

### **Common Issues**

| Issue | Solution |
|-------|----------|
| **Compilation errors** | Run `cargo update` and ensure Rust 1.70+ |
| **Network timeouts** | Check internet connection and try different RPC endpoint |
| **Missing dependencies** | Run `cargo clean && cargo build` |
| **Type errors** | Ensure using latest AtipicialRust version |

### **Debug Mode**
Enable detailed logging for troubleshooting:
```bash
RUST_LOG=debug cargo run --example connect_to_node
```

### **Example-Specific Issues**

| Example | Common Issues | Solutions |
|---------|---------------|-----------|
| **atipicial_nodes** | Network connectivity | Try multiple endpoints |
| **atipicial_wallets** | Key management | Use proper secure storage |
| **atipicial_contracts** | Deployment costs | Ensure sufficient GAS balance |
| **atipicial_aep17_tokens** | Token queries | Verify contract addresses |

## 📖 **Learning Path**

### **Beginner Path**
1. **[connect_to_node](atipicial_nodes/examples/connect_to_node.rs)** - Basic connectivity
2. **[wallet_management](atipicial_wallets/examples/wallet_management.rs)** - Account creation
3. **[aep17_token_operations](atipicial_aep17_tokens/examples/aep17_token_operations.rs)** - Token basics

### **Intermediate Path**  
1. **[create_and_send_transaction](atipicial_transactions/examples/create_and_send_transaction.rs)** - Transaction creation
2. **[interact_with_contract](atipicial_smart_contracts/examples/interact_with_contract.rs)** - Contract interaction
3. **[deploy_atipicial_contract](atipicial_contracts/examples/deploy_atipicial_contract.rs)** - Contract deployment

### **Advanced Path**
1. **[famous_contracts](atipicial_famous_contracts/)** - Production contract interaction
2. **[atipicial_nns](atipicial_nns/)** - Name service integration
3. **[atipicial_x](atipicial_x/)** - EVM compatibility

## 💡 **Best Practices**

### **Development Workflow**
1. **Start with TestNet** - Always test on TestNet first
2. **Use proper error handling** - Implement comprehensive error management
3. **Validate inputs** - Check all user inputs and contract parameters
4. **Monitor transactions** - Track transaction status and confirmations
5. **Security first** - Never log private keys or sensitive data

### **Production Deployment**
1. **Audit thoroughly** - Review all code before MainNet deployment
2. **Use hardware wallets** - For signing valuable transactions
3. **Implement monitoring** - Track contract health and performance
4. **Plan for upgrades** - Design contracts with upgrade mechanisms
5. **Have recovery plans** - Prepare for emergency scenarios

## 🤝 **Contributing**

### **Adding New Examples**
1. Create example in appropriate category directory
2. Follow the established code structure and documentation style
3. Include comprehensive comments and error handling
4. Add example to category's `Cargo.toml`
5. Update this README with new example information
6. Test thoroughly on TestNet

### **Improving Existing Examples**
1. Ensure compatibility with latest AtipicialRust APIs
2. Add more comprehensive error handling
3. Improve documentation and comments
4. Add performance optimizations
5. Include additional security considerations

### **Documentation Standards**
- Start with clear overview and purpose
- Include step-by-step explanations
- Provide best practices and security tips
- Add troubleshooting information
- Include links to relevant resources

## 🔗 **Additional Resources**

### **Atipicial Documentation**
- [Atipicial Developer Guide](https://docs.atipicial.com/)
- [Atipicial RPC API](https://docs.atipicial.com/docs/en-us/reference/rpc/latest-version/api.html)
- [Smart Contract Development](https://docs.atipicial.com/docs/en-us/develop/write/basics.html)

### **AtipicialRust SDK**
- [SDK Documentation](https://docs.rs/atipicial)
- [GitHub Repository](https://github.com/R3E-Network/AtipicialRust)
- [Release Notes](https://github.com/R3E-Network/AtipicialRust/releases)

### **Community Resources**  
- [Atipicial Discord](https://discord.gg/atipicial)
- [Atipicial Reddit](https://reddit.com/r/ATC)
- [Atipicial Developer Community](https://atipicial.com/dev)

---

**Happy coding with AtipicialRust! 🦀⚡**

> 💡 **Pro Tip**: Start with the `atipicial_nodes` examples to understand basic connectivity, then progress through the categories based on your specific use case.

---

> **Atipicial Chain** — sovereign Layer-1 for smart contracts and digital assets.
> 👑 Founded & engineered by **xmoohad** — Blockchain Scientist · Computer Programmer.
> `ATC` Atipicial Coin · `ATD` AtipicialDollar · addresses begin with **A**
