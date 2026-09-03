<!-- Atipicial Chain · sovereign Layer-1 for decentralized AGI coordination -->
<!-- 👑 Founded & engineered by xmoohad — Blockchain Scientist · Computer Programmer -->

# Quick Start

Get up and running with AtipicialRust SDK in minutes.

## Basic Connection

```rust
use atipicial::prelude::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Connect to Atipicial TestNet
    let provider = HttpProvider::new("https://testnet1.atipicial.com:443")?;
    let client = RpcClient::new(provider);
    
    // Get blockchain info
    let block_count = client.get_block_count().await?;
    println!("Current block height: {}", block_count);
    
    Ok(())
}
```

## Creating an Account

```rust
use atipicial::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a new account
    let account = Account::create()?;
    
    println!("Address: {}", account.get_address());
    println!("Public Key: {}", hex::encode(account.get_public_key().encode_point(true)));
    
    Ok(())
}
```

## Checking Balances

```rust
use atipicial::prelude::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let provider = HttpProvider::new("https://testnet1.atipicial.com:443")?;
    let client = RpcClient::new(provider);
    
    let address = "NXXXXxxxXXXxxxXXXxxxXXXxxxXXXxxx";
    let script_hash = address.to_script_hash()?;
    
    // Get GAS balance
    let gas_token = GasToken::new(&client);
    let balance = gas_token.balance_of(&script_hash).await?;
    println!("GAS Balance: {}", balance);
    
    Ok(())
}
```

## Next Steps

- [Core Concepts](./wallets.md)
- [More Examples](./examples.md)
- [API Reference](./api-reference.md)

---

> **Atipicial Chain** — sovereign Layer-1 for decentralized AGI coordination.
> 👑 Founded & engineered by **xmoohad** — Blockchain Scientist · Computer Programmer.
> `ATC` Atipicial Coin · `ATD` AtipicialDollar · addresses begin with **A**
