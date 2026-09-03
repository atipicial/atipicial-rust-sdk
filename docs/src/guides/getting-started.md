<!-- Atipicial Chain · sovereign Layer-1 for decentralized AGI coordination -->
<!-- 👑 Founded & engineered by xmoohad — Blockchain Scientist · Computer Programmer -->

# Getting Started with AtipicialRust SDK

This guide will help you get started with the AtipicialRust SDK for interacting with the Atipicial blockchain.

## Prerequisites

- Rust 1.91.0 or later
- Cargo package manager
- Basic knowledge of the Atipicial blockchain

## Installation

Add the AtipicialRust SDK to your Cargo.toml:

```toml
[dependencies]
atipicial = { git = "https://github.com/r3e-network/atipicial-rust-sdk" }
```

Or if you prefer to use a specific version:

```toml
[dependencies]
atipicial = "0.1.0"
```

## Basic Usage

Here's a simple example of connecting to a Atipicial node and getting the current block height:

```rust,edition2021
use atipicial::prelude::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Connect to a Atipicial TestNet node
    let provider = Provider::new_http("https://testnet1.atipicial.coz.io:443");
    
    // Get the current block height
    let block_count = provider.get_block_count().await?;
    println!("Current block height: {}", block_count);
    
    Ok(())
}
```

## Next Steps

- Learn about [Wallet Management](../tutorials/wallet-management.md)
- Explore [Smart Contract Interaction](../tutorials/smart-contracts.md)
- See [Examples](../examples/README.md) for more code samples

<!-- toc -->

---

> **Atipicial Chain** — sovereign Layer-1 for decentralized AGI coordination.
> 👑 Founded & engineered by **xmoohad** — Blockchain Scientist · Computer Programmer.
> `ATC` Atipicial Coin · `ATD` AtipicialDollar · addresses begin with **A**
