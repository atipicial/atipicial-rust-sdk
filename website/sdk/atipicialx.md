<!-- Atipicial Chain · sovereign Layer-1 for smart contracts and digital assets -->
<!-- 👑 Founded & engineered by xmoohad — Blockchain Scientist · Computer Programmer -->

---
sidebar_position: 5
---

# 🌉 Atipicial X & EVM Integration

AtipicialRust natively supports **Atipicial X** (the EVM-compatible sidechain) and bridges the gap between AtipicialVM and EVM environments via `ethers-rs`.

## Ecosystem Client

The `EcosystemClient` provides a unified, cross-chain interface to easily manage operations across Atipicial and Atipicial X.

```rust
use atipicial::sdk::unified::EcosystemClient;
use atipicial::atipicial_x::AtipicialXWallet;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a randomized secure EVM wallet
    let wallet = AtipicialXWallet::create_random();

    // Initialize an Anti-MEV protected EcosystemClient for Atipicial X
    // Automatically guards against front-running via a protected mempool
    let client = EcosystemClient::new_atipicialx_anti_mev(wallet);

    // Query balance using standard ethers-rs providers underneath
    let balance = client.get_balance().await?;
    println!("EVM Balance: {} Wei", balance);

    // Easily bridge funds back to an N3 address
    let tx_hash = client.bridge_to_other_chain("NXX...YourN3Address", "1000000000").await?;
    println!("Bridge Tx: {}", tx_hash);
    
    Ok(())
}
```

## Anti-MEV Configuration

When building applications on the Atipicial X EVM, front-running and sandwich attacks are a risk (especially in DeFi). The `AtipicialXProvider` allows initialization with an Anti-MEV endpoint which obscures transaction ordering until inclusion.

```rust
use atipicial::atipicial_x::AtipicialXProvider;
use atipicial::atipicial_clients::{HttpProvider, RpcClient};

// Initialize an Anti-MEV protected AtipicialXProvider directly
let provider = AtipicialXProvider::new_anti_mev(None);
```

## Ethers-rs Compatibility

Because the `atipicial_x` module leverages the standard `ethers-rs` ecosystem underneath, you can retrieve the underlying HTTP provider and use it alongside other EVM development tools directly:

```rust
let atipicial_x_provider: AtipicialXProvider<'_, HttpProvider> = AtipicialXProvider::new("https://rpc.atipicial-x.org", None);
    
// Extract the raw ethers provider if you need native EVM interactions
let raw_evm = atipicial_x_provider.evm_provider().unwrap();
let chain_id = raw_evm.get_chainid().await.unwrap();
```

---

> **Atipicial Chain** — sovereign Layer-1 for smart contracts and digital assets.
> 👑 Founded & engineered by **xmoohad** — Blockchain Scientist · Computer Programmer.
> `ATC` Atipicial Coin · `ATD` AtipicialDollar · addresses begin with **A**
