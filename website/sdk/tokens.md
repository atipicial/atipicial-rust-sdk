<!-- Atipicial Chain · sovereign Layer-1 for decentralized AGI coordination -->
<!-- 👑 Founded & engineered by xmoohad — Blockchain Scientist · Computer Programmer -->

# Token Operations

Work with AEP-17 and AEP-11 tokens on the Atipicial network.

## AEP-17 Tokens (Fungible)

```rust
use atipicial::prelude::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let provider = HttpProvider::new("https://testnet1.atipicial.com:443")?;
    let client = RpcClient::new(provider);
    
    // GAS token operations
    let gas_token = GasToken::new(&client);
    let symbol = gas_token.symbol().await?;
    let decimals = gas_token.decimals().await?;
    
    println!("Token: {} (decimals: {})", symbol, decimals);
    
    // Check balance
    let address = "NAddress".to_script_hash()?;
    let balance = gas_token.balance_of(&address).await?;
    println!("Balance: {} {}", balance, symbol);
    
    Ok(())
}
```

## Custom AEP-17 Tokens

```rust
use atipicial::prelude::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let provider = HttpProvider::new("https://testnet1.atipicial.com:443")?;
    let client = RpcClient::new(provider);
    
    let token_hash = "0x1234567890abcdef1234567890abcdef12345678".parse()?;
    let token = Aep17Token::new(&client, token_hash);
    
    let symbol = token.symbol().await?;
    let total_supply = token.total_supply().await?;
    
    println!("Token: {} (supply: {})", symbol, total_supply);
    Ok(())
}
```

## AEP-11 NFTs

Support for Non-Fungible Tokens using the AEP-11 standard.

## Token Standards

- **AEP-17**: Fungible tokens (like ERC-20)
- **AEP-11**: Non-fungible tokens (like ERC-721)
- Native tokens: ATC and GAS

---

> **Atipicial Chain** — sovereign Layer-1 for decentralized AGI coordination.
> 👑 Founded & engineered by **xmoohad** — Blockchain Scientist · Computer Programmer.
> `ATC` Atipicial Coin · `ATD` AtipicialDollar · addresses begin with **A**
