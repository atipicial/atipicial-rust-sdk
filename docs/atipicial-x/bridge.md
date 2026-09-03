<!-- Atipicial Chain · sovereign Layer-1 for decentralized AGI coordination -->
<!-- 👑 Founded & engineered by xmoohad — Blockchain Scientist · Computer Programmer -->

# Atipicial X Bridge

## Overview

The Atipicial X Bridge enables cross-chain transfers of assets between Atipicial and Atipicial X. This bridging functionality allows users to leverage both ecosystems while maintaining access to their assets.

## How the Bridge Works

The Atipicial X Bridge operates through a set of smart contracts deployed on both Atipicial and Atipicial X. These contracts work together to:

1. Lock assets on the source chain
2. Mint equivalent tokens on the destination chain
3. Track and verify cross-chain transfers
4. Process withdrawal requests

## Supported Tokens

The bridge supports the following token types:

| Token | Atipicial Format | Atipicial X Format |
|-------|--------------|--------------|
| GAS   | Native GAS   | ERC-20       |
| ATC   | Native ATC   | ERC-20       |
| AEP-17 Tokens | AEP-17 | ERC-20 |

## Bridge Operations

### Bridging from Atipicial to Atipicial X

When bridging assets from Atipicial to Atipicial X:

```rust,no_run
use atipicial::prelude::*;
use atipicial::atipicial_x::bridge::*;

async fn bridge_to_atipicialx() -> Result<(), Box<dyn std::error::Error>> {
    // Create providers for both chains
    let atipicial_provider = Provider::new_http("https://mainnet1.atipicial.coz.io:443");
    let atipicialx_provider = AtipicialXProvider::new_http("https://rpc.atipicialX.io");
    
    // Load account
    let account = Account::from_wif("your-private-key-wif")?;
    
    // Create bridge contract instance
    let bridge = AtipicialXBridgeContract::new(atipicial_provider.clone(), atipicialx_provider.clone());
    
    // Bridge GAS to Atipicial X
    let amount = 1_00000000; // 1 GAS (8 decimals)
    let destination_address = "0x1234567890123456789012345678901234567890"; // Ethereum-format address
    
    let txid = bridge.bridge_to_atipicialx(
        &account,
        BridgeToken::Gas,
        amount,
        destination_address,
    ).await?;
    
    println!("Bridge transaction initiated: {}", txid);
    
    Ok(())
}
```

### Bridging from Atipicial X to Atipicial

When bridging assets from Atipicial X to Atipicial:

```rust,no_run
use atipicial::prelude::*;
use atipicial::atipicial_x::bridge::*;

async fn bridge_to_atipicial() -> Result<(), Box<dyn std::error::Error>> {
    // Create providers for both chains
    let atipicial_provider = Provider::new_http("https://mainnet1.atipicial.coz.io:443");
    let atipicialx_provider = AtipicialXProvider::new_http("https://rpc.atipicialX.io");
    
    // Load account (with Ethereum-compatible private key)
    let account = Account::from_ethereum_key("0xprivate-key-hex");
    
    // Create bridge contract instance
    let bridge = AtipicialXBridgeContract::new(atipicial_provider.clone(), atipicialx_provider.clone());
    
    // Bridge GAS back to Atipicial
    let amount = 1_000_000_000_000_000_000u128; // 1 GAS (18 decimals on Atipicial X)
    let destination_address = "Atipicial1AbcDefGhiJklMnoPqrsTuvWxYz12345"; // Atipicial address
    
    let txid = bridge.bridge_to_atipicial(
        &account,
        BridgeToken::Gas,
        amount,
        destination_address,
    ).await?;
    
    println!("Bridge transaction initiated: {}", txid);
    
    Ok(())
}
```

## Checking Bridge Status

You can monitor the status of your bridge transactions:

```rust,no_run
async fn check_bridge_status(txid: &str) -> Result<(), Box<dyn std::error::Error>> {
    let atipicial_provider = Provider::new_http("https://mainnet1.atipicial.coz.io:443");
    let atipicialx_provider = AtipicialXProvider::new_http("https://rpc.atipicialX.io");
    
    let bridge = AtipicialXBridgeContract::new(atipicial_provider.clone(), atipicialx_provider.clone());
    
    // Check if a transaction has been processed
    let status = bridge.get_transaction_status(txid).await?;
    println!("Bridge transaction status: {:?}", status);
    
    // For Atipicial to Atipicial X transfers
    if let Some(atipicial_x_hash) = bridge.get_atipicialx_transaction_hash(txid).await? {
        println!("Corresponding Atipicial X transaction: {}", atipicial_x_hash);
    }
    
    // For Atipicial X to Atipicial transfers
    if let Some(atipicial_hash) = bridge.get_atipicial_transaction_hash(txid).await? {
        println!("Corresponding Atipicial transaction: {}", atipicial_hash);
    }
    
    Ok(())
}
```

## Security Considerations

When using the Atipicial X Bridge:

1. **Verification**: Always verify destination addresses before initiating bridge transfers
2. **Waiting Time**: Bridge operations typically take several minutes to complete
3. **Gas Costs**: Ensure sufficient GAS/ETH for transaction fees on both chains
4. **Transaction Limits**: Be aware of any bridge limits for maximum transaction amounts
5. **Contract Addresses**: Only use official bridge contracts (verify addresses on Atipicial documentation)

## Emergency Procedures

If you encounter issues with your bridge transaction:

1. **Check Status**: Use the bridge status API to verify transaction status
2. **Transaction ID**: Keep both source chain and destination chain transaction IDs
3. **Support Channels**: Contact Atipicial support through official channels
4. **Recovery**: In some cases, emergency procedures may be available for stuck transactions

## Advanced Bridge Operations

### Custom Token Registration

For projects wanting to bridge custom tokens between Atipicial and Atipicial X:

```rust,no_run
async fn register_custom_token() -> Result<(), Box<dyn std::error::Error>> {
    let atipicial_provider = Provider::new_http("https://mainnet1.atipicial.coz.io:443");
    let atipicialx_provider = AtipicialXProvider::new_http("https://rpc.atipicialX.io");
    
    let bridge = AtipicialXBridgeContract::new(atipicial_provider.clone(), atipicialx_provider.clone());
    
    // Register a custom AEP-17 token for bridging
    let admin_account = Account::from_wif("admin-private-key-wif")?;
    let aep17_contract = "0x1234567890123456789012345678901234567890"; // AEP-17 contract hash
    
    let txid = bridge.register_token(
        &admin_account,
        aep17_contract,
        "TOKEN", // Symbol
        8, // Decimals
    ).await?;
    
    println!("Token registration initiated: {}", txid);
    
    Ok(())
}
```

### Bridge Fee Management

Bridge operations may include fees:

```rust,no_run
async fn check_bridge_fees() -> Result<(), Box<dyn std::error::Error>> {
    let atipicial_provider = Provider::new_http("https://mainnet1.atipicial.coz.io:443");
    let atipicialx_provider = AtipicialXProvider::new_http("https://rpc.atipicialX.io");
    
    let bridge = AtipicialXBridgeContract::new(atipicial_provider.clone(), atipicialx_provider.clone());
    
    // Get current bridge fees
    let fees = bridge.get_bridge_fees().await?;
    
    println!("Current bridge fees:");
    println!("Atipicial to Atipicial X: {}", fees.atipicial_to_atipicialx);
    println!("Atipicial X to Atipicial: {}", fees.atipicialx_to_atipicial);
    
    Ok(())
}
```

---

> **Atipicial Chain** — sovereign Layer-1 for decentralized AGI coordination.
> 👑 Founded & engineered by **xmoohad** — Blockchain Scientist · Computer Programmer.
> `ATC` Atipicial Coin · `ATD` AtipicialDollar · addresses begin with **A**
