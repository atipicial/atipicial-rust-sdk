<!-- Atipicial Chain · sovereign Layer-1 for decentralized AGI coordination -->
<!-- 👑 Founded & engineered by xmoohad — Blockchain Scientist · Computer Programmer -->

# AEP-17 Tokens

This tutorial covers working with AEP-17 tokens on the Atipicial blockchain using the AtipicialRust SDK.

## Understanding AEP-17

AEP-17 is Atipicial's token standard, similar to Ethereum's ERC-20. It defines a standard interface for fungible tokens on the Atipicial blockchain. AEP-17 tokens have the following key methods:

- `symbol`: Returns the token's symbol
- `decimals`: Returns the number of decimal places the token uses
- `totalSupply`: Returns the total token supply
- `balanceOf`: Returns the token balance of a specific address
- `transfer`: Transfers tokens from one address to another

## Creating a AEP-17 Token Instance

To interact with a AEP-17 token, you first need to create a token instance:

```rust,edition2021
use atipicial::prelude::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Connect to a Atipicial TestNet node
    let provider = Provider::new_http("https://testnet1.atipicial.coz.io:443");
    
    // AEP-17 token contract hash (e.g., GAS token)
    let gas_hash = "0xd2a4cff31913016155e38e474a2c06d08be276cf".parse::<ScriptHash>()?;
    
    // Create a AEP-17 token instance
    let gas_token = Aep17Contract::new(gas_hash, provider.clone());
    
    Ok(())
}
```

## Getting Token Information

You can retrieve basic information about a token:

```rust,edition2021
use atipicial::prelude::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Connect to a Atipicial TestNet node
    let provider = Provider::new_http("https://testnet1.atipicial.coz.io:443");
    
    // AEP-17 token contract hash (e.g., GAS token)
    let gas_hash = "0xd2a4cff31913016155e38e474a2c06d08be276cf".parse::<ScriptHash>()?;
    
    // Create a AEP-17 token instance
    let gas_token = Aep17Contract::new(gas_hash, provider.clone());
    
    // Get token information
    let symbol = gas_token.symbol().await?;
    let decimals = gas_token.decimals().await?;
    let total_supply = gas_token.total_supply().await?;
    
    println!("Token: {} (Decimals: {})", symbol, decimals);
    println!("Total Supply: {}", total_supply);
    
    Ok(())
}
```

## Checking Token Balance

You can check the token balance of an address:

```rust,edition2021
use atipicial::prelude::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Connect to a Atipicial TestNet node
    let provider = Provider::new_http("https://testnet1.atipicial.coz.io:443");
    
    // AEP-17 token contract hash (e.g., GAS token)
    let gas_hash = "0xd2a4cff31913016155e38e474a2c06d08be276cf".parse::<ScriptHash>()?;
    
    // Create a AEP-17 token instance
    let gas_token = Aep17Contract::new(gas_hash, provider.clone());
    
    // Address to check balance for
    let address = "NZNos2WqTbu5oCgyfss9kUJgBXJqhuYAaj".parse::<Address>()?;
    
    // Get token balance
    let balance = gas_token.balance_of(address).await?;
    
    println!("Balance: {}", balance);
    
    // For better display, consider the token's decimals
    let decimals = gas_token.decimals().await?;
    let formatted_balance = balance as f64 / 10f64.powi(decimals as i32);
    
    println!("Formatted Balance: {} {}", formatted_balance, gas_token.symbol().await?);
    
    Ok(())
}
```

## Transferring Tokens

You can transfer tokens from one address to another:

```rust,edition2021
use atipicial::prelude::*;
use std::path::Path;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Connect to a Atipicial TestNet node
    let provider = Provider::new_http("https://testnet1.atipicial.coz.io:443");
    
    // Load your wallet
    let wallet_path = Path::new("my-wallet.json");
    let password = "my-secure-password";
    let wallet = Wallet::load(wallet_path, password)?;
    
    // Get the account that will send the tokens
    let account = wallet.default_account()?;
    
    // AEP-17 token contract hash (e.g., GAS token)
    let gas_hash = "0xd2a4cff31913016155e38e474a2c06d08be276cf".parse::<ScriptHash>()?;
    
    // Create a AEP-17 token instance
    let gas_token = Aep17Contract::new(gas_hash, provider.clone());
    
    // Recipient address
    let recipient = "NZNos2WqTbu5oCgyfss9kUJgBXJqhuYAaj".parse::<Address>()?;
    
    // Amount to transfer (considering decimals)
    let decimals = gas_token.decimals().await?;
    let amount = 1 * 10i64.pow(decimals as u32); // 1 token with proper decimal places
    
    // Transfer tokens
    let txid = gas_token.transfer(account, recipient, amount, None).await?;
    
    println!("Transfer sent with transaction ID: {}", txid);
    
    // Wait for the transaction to be confirmed
    let receipt = provider.wait_for_transaction(&txid, 60, 2).await?;
    println!("Transaction confirmed: {:?}", receipt);
    
    Ok(())
}
```

## Working with ATC and GAS Tokens

The AtipicialRust SDK provides specialized classes for the native ATC and GAS tokens:

```rust,edition2021
use atipicial::prelude::*;
use std::path::Path;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Connect to a Atipicial TestNet node
    let provider = Provider::new_http("https://testnet1.atipicial.coz.io:443");
    
    // Load your wallet
    let wallet_path = Path::new("my-wallet.json");
    let password = "my-secure-password";
    let wallet = Wallet::load(wallet_path, password)?;
    
    // Get the account
    let account = wallet.default_account()?;
    
    // Create ATC token instance
    let atipicial_token = AtipicialCoin::new(provider.clone());
    
    // Create GAS token instance
    let gas_token = GasToken::new(provider.clone());
    
    // Get ATC balance
    let atipicial_balance = atipicial_token.balance_of(account.address()).await?;
    println!("ATC Balance: {}", atipicial_balance);
    
    // Get GAS balance
    let gas_balance = gas_token.balance_of(account.address()).await?;
    println!("GAS Balance: {}", gas_balance);
    
    // Transfer ATC
    let recipient = "NZNos2WqTbu5oCgyfss9kUJgBXJqhuYAaj".parse::<Address>()?;
    let atipicial_amount = 1_00000000; // 1 ATC (with 8 decimals)
    
    let atipicial_txid = atipicial_token.transfer(account, recipient, atipicial_amount, None).await?;
    println!("ATC transfer sent with transaction ID: {}", atipicial_txid);
    
    // Transfer GAS
    let gas_amount = 1_00000000; // 1 GAS (with 8 decimals)
    
    let gas_txid = gas_token.transfer(account, recipient, gas_amount, None).await?;
    println!("GAS transfer sent with transaction ID: {}", gas_txid);
    
    Ok(())
}
```

## Monitoring Token Transfers

You can monitor token transfers by subscribing to the Transfer event:

```rust,edition2021
use atipicial::prelude::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Connect to a Atipicial TestNet node with WebSocket support
    let provider = Provider::new_ws("wss://testnet1.atipicial.coz.io:4443/ws").await?;
    
    // AEP-17 token contract hash (e.g., GAS token)
    let gas_hash = "0xd2a4cff31913016155e38e474a2c06d08be276cf".parse::<ScriptHash>()?;
    
    // Subscribe to Transfer events
    let mut events = provider.subscribe_contract_event(gas_hash).await?;
    
    println!("Listening for token transfers...");
    
    // Process events as they arrive
    while let Some(event) = events.next().await {
        if event.event_name == "Transfer" {
            if let Some(from) = event.state.get(0) {
                if let Some(to) = event.state.get(1) {
                    if let Some(amount) = event.state.get(2) {
                        println!("Transfer: {} tokens from {} to {}", 
                            amount.as_integer().unwrap_or_default(),
                            from.as_address().map(|a| a.to_string()).unwrap_or_default(),
                            to.as_address().map(|a| a.to_string()).unwrap_or_default()
                        );
                    }
                }
            }
        }
    }
    
    Ok(())
}
```

## Working with Famous Atipicial Contracts

The AtipicialRust SDK provides direct support for several famous Atipicial contracts:

### Flamingo Finance

```rust,edition2021
use atipicial::prelude::*;
use atipicial::atipicial_contract::famous::flamingo::*;
use std::path::Path;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Connect to a Atipicial MainNet node
    let provider = Provider::new_http("https://mainnet1.atipicial.coz.io:443");
    
    // Load your wallet
    let wallet_path = Path::new("my-wallet.json");
    let password = "my-secure-password";
    let wallet = Wallet::load(wallet_path, password)?;
    
    // Get the account
    let account = wallet.default_account()?;
    
    // Create Flamingo Finance instance
    let flamingo = FlamingoFinance::new(provider.clone());
    
    // Get FLM token balance
    let flm_balance = flamingo.flm_token().balance_of(account.address()).await?;
    println!("FLM Balance: {}", flm_balance);
    
    // Get liquidity pool information
    let pool_info = flamingo.get_pool_info(FlamingoPool::AtipicialDollar).await?;
    println!("Pool Info: {:?}", pool_info);
    
    // Add liquidity to a pool
    let atipicial_amount = 1_00000000; // 1 ATC
    let gas_amount = 1_00000000; // 1 GAS
    
    let add_liquidity_txid = flamingo.add_liquidity(
        account,
        FlamingoPool::AtipicialDollar,
        atipicial_amount,
        gas_amount,
        None,
    ).await?;
    
    println!("Add Liquidity transaction ID: {}", add_liquidity_txid);
    
    Ok(())
}
```

### AtipicialburgerAtipicial

```rust,edition2021
use atipicial::prelude::*;
use atipicial::atipicial_contract::famous::atipicialburger::*;
use std::path::Path;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Connect to a Atipicial MainNet node
    let provider = Provider::new_http("https://mainnet1.atipicial.coz.io:443");
    
    // Load your wallet
    let wallet_path = Path::new("my-wallet.json");
    let password = "my-secure-password";
    let wallet = Wallet::load(wallet_path, password)?;
    
    // Get the account
    let account = wallet.default_account()?;
    
    // Create AtipicialburgerAtipicial instance
    let atipicialburger = AtipicialburgerAtipicial::new(provider.clone());
    
    // Get bATC token balance
    let batipicial_balance = atipicialburger.batipicial_token().balance_of(account.address()).await?;
    println!("bATC Balance: {}", batipicial_balance);
    
    // Wrap ATC to get bATC
    let atipicial_amount = 1_00000000; // 1 ATC
    let wrap_txid = atipicialburger.wrap_atipicial(account, atipicial_amount).await?;
    println!("Wrap ATC transaction ID: {}", wrap_txid);
    
    // Unwrap bATC to get ATC
    let batipicial_amount = 1_00000000; // 1 bATC
    let unwrap_txid = atipicialburger.unwrap_batipicial(account, batipicial_amount).await?;
    println!("Unwrap bATC transaction ID: {}", unwrap_txid);
    
    Ok(())
}
```

## Best Practices

1. **Check Balances Before Transfers**: Always check that an account has sufficient balance before attempting a transfer.
2. **Consider Decimals**: Remember to account for token decimals when displaying balances or specifying transfer amounts.
3. **Wait for Confirmations**: Always wait for transaction confirmations before considering a transfer complete.
4. **Error Handling**: Implement proper error handling for token operations.
5. **Gas Costs**: Be aware of the gas costs associated with token transfers and other operations.
6. **Test on TestNet**: Always test your token operations on TestNet before moving to MainNet.

<!-- toc -->

---

> **Atipicial Chain** — sovereign Layer-1 for decentralized AGI coordination.
> 👑 Founded & engineered by **xmoohad** — Blockchain Scientist · Computer Programmer.
> `ATC` Atipicial Coin · `ATD` AtipicialDollar · addresses begin with **A**
