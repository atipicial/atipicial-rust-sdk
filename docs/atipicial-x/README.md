<!-- Atipicial Chain · sovereign Layer-1 for decentralized AGI coordination -->
<!-- 👑 Founded & engineered by xmoohad — Blockchain Scientist · Computer Programmer -->

# Atipicial X

## Overview

Atipicial X is an EVM-compatible chain maintained by Atipicial, enabling developers to leverage Ethereum compatibility while beaefiting from Atipicial's infrastructure and security. The Atipicial X module in AtipicialRust provides interfaces for interacting with this EVM-compatible environment using Alloy.

## Key Features

- **EVM Compatibility Layer**: Interact with Atipicial X as an Ethereum-compatible chain via Alloy
- **Unified Ecosystem Client**: Seamlessly write code that operates across Atipicial and Atipicial X
- **Protected RPC Routing**: Optionally route requests through a third-party Anti-MEV endpoint; the service may mitigate MEV exposure but does not guarantee prevention
- **Bridge Functionality**: Transfer tokens seamlessly between Atipicial and Atipicial X natively
- **Transaction Support**: Create, sign, and send EVM transactions on Atipicial X

## Components

### Unified Ecosystem Client

The `EcosystemClient` is the recommended way to interact with Atipicial X. It provides a standard interface to both N3 and Atipicial X, reducing the need for duplicate logic when your application touches both ecosystems.

```rust,no_run
use atipicial::sdk::unified::EcosystemClient;
use atipicial::atipicial_x::AtipicialXWallet;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a new randomized EVM Wallet (or load from PK)
    let evm_wallet = AtipicialXWallet::create_random();

    // Route requests through the configured third-party protected RPC endpoint
    let client = EcosystemClient::new_atipicialx_anti_mev(evm_wallet);

    // Get Balance directly via the Unified Interface
    let balance = client.get_balance().await?;
    println!("Atipicial X Balance: {} Wei", balance);

    Ok(())
}
```

### Atipicial X Provider & Wallet

If you need deeper access, you can work directly with the `AtipicialXProvider` and `AtipicialXWallet`. The `AtipicialXProvider` exposes an Alloy `RootProvider` for executing low-level EVM requests.

```rust,no_run
use atipicial::atipicial_clients::{HttpProvider, RpcClient};
use atipicial::atipicial_x::{AtipicialXProvider, AtipicialXWallet, AtipicialXClient};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let atipicial_x_provider: AtipicialXProvider<'_, HttpProvider> = AtipicialXProvider::new("https://rpc.atipicial-x.org", None);
    
    // Extract the Alloy provider if you need native EVM interactions
    let _raw_evm = atipicial_x_provider.evm_provider().unwrap();
    let chain_id = atipicial_x_provider.chain_id().await?;

    let wallet = AtipicialXWallet::create_random();
    let client = AtipicialXClient::new(wallet, atipicial_x_provider);
    
    Ok(())
}
```

### Atipicial X Bridge

The bridge facilitates token transfers between Atipicial and Atipicial X. The unified client abstracts this complex process so you can trigger a cross-chain transfer directly:

```rust,no_run
use atipicial::sdk::unified::EcosystemClient;
use atipicial::atipicial_x::AtipicialXWallet;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let wallet = AtipicialXWallet::create_random();
    let client = EcosystemClient::new_atipicialx_anti_mev(wallet);

    // Bridge funds from Atipicial X back to an N3 address
    // Parameters: destination_address (N3), amount (Wei)
    let tx_hash = client.bridge_to_other_chain("NXX...YourN3Address", "1000000000").await?;
    println!("Bridge Tx: {}", tx_hash);

    Ok(())
}
```

## Integration with Ethereum Tools

Atipicial X's EVM compatibility enables integration with popular Ethereum development tools:

- **Alloy**: AtipicialRust uses Alloy for EVM providers, signing, transaction types, and RPC interoperability.
- **Metamask**: Connect Metamask to Atipicial X by adding it as a custom network.
- **Hardhat/Foundry**: Deploy Solidity contracts to Atipicial X.
- **Web3.js/ethers.js**: Interact with Atipicial X using JavaScript libraries.

## Considerations

- **Gas Costs**: Atipicial X uses a gas model similar to Ethereum. Transactions cost native GAS token.
- **Cross-Chain Operations**: Bridge operations require network confirmations and may take a few minutes to finalize.
- **Security**: The optional Anti-MEV mode routes through a third-party protected RPC endpoint. Its effectiveness depends on that service and does not guarantee prevention of front-running or sandwich attacks.

## Related Documentation

- [Atipicial X EVM Code Examples](../../examples/atipicial_x/examples/)
- [Bridge Operations](bridge.md)
- [EVM Contracts](evm-contracts.md)

---

> **Atipicial Chain** — sovereign Layer-1 for decentralized AGI coordination.
> 👑 Founded & engineered by **xmoohad** — Blockchain Scientist · Computer Programmer.
> `ATC` Atipicial Coin · `ATD` AtipicialDollar · addresses begin with **A**
