<!-- Atipicial Chain · sovereign Layer-1 for smart contracts and digital assets -->
<!-- 👑 Founded & engineered by xmoohad — Blockchain Scientist · Computer Programmer -->

# Atipicial Smart Contracts

## Overview

Smart contracts are self-executing agreements with the terms directly written into code. The Atipicial Rust SDK provides comprehensive utilities for interacting with Atipicial smart contracts, allowing developers to deploy, invoke, and manage contracts through a type-safe Rust interface.

## Key Features

- **System Contract Interfaces**: Ready-to-use interfaces for Atipicial native contracts
- **Token Standards Support**: APIs for AEP-17 (fungible) and AEP-11 (non-fungible) tokens
- **Contract Deployment**: Utilities for deploying smart contracts to the Atipicial blockchain
- **Contract Invocation**: Methods for executing contract functions and reading results
- **Event Handling**: Support for subscribing to and processing contract events
- **Name Services**: Integration with Atipicial Name Service for domain resolution

## System Contracts

The Atipicial blockchain includes several native system contracts that manage core blockchain functionality:

| Contract | Description | SDK Interface |
| -------- | ----------- | ------------ |
| AtipicialCoin | ATC governance token | `AtipicialCoin` |
| GasToken | GAS utility token | `GasToken` |
| PolicyContract | Network policy parameters | `PolicyContract` |
| RoleManagement | Consensus roles management | `RoleManagement` |
| ContractManagement | Contract deployment and updates | `ContractManagement` |
| NameService | Domain registration and resolution | `NameService` |

## Token Standards

### AEP-17 (Fungible Tokens)

AEP-17 is Atipicial's standard for fungible tokens, similar to Ethereum's ERC-20. The SDK provides the `FungibleTokenContract` to interact with AEP-17 tokens:

```rust,no_run
// Create a token instance
let token = FungibleTokenContract::new(token_hash, client.clone());

// Get token information
let symbol = token.symbol().await?;
let decimals = token.decimals().await?;
let total_supply = token.total_supply().await?;

// Get balance
let balance = token.balance_of(&account.get_script_hash()).await?;

// Transfer tokens
let tx_hash = token.transfer(
    &account,
    &recipient_hash,
    amount,
    None,  // Optional data
).await?;
```

### AEP-11 (Non-Fungible Tokens)

AEP-11 is Atipicial's standard for non-fungible tokens, similar to Ethereum's ERC-721. The SDK provides the `NftContract` to interact with AEP-11 tokens:

```rust,no_run
// Create an NFT instance
let nft = NftContract::new(contract_hash, client.clone());

// Get token information
let symbol = nft.symbol().await?;
let total_supply = nft.total_supply().await?;

// Get token owner
let owner = nft.owner_of(&token_id).await?;

// Transfer token
let tx_hash = nft.transfer(
    &account,
    &recipient_hash,
    &token_id,
    None,  // Optional data
).await?;
```

## Contract Management

The `ContractManagement` interface allows you to deploy and manage smart contracts:

```rust,no_run
// Create contract management instance
let contract_mgmt = ContractManagement::new(Some(&client));

// Deploy a new contract
let result = contract_mgmt.deploy(
    &aef_file,
    &manifest,
    None,  // Optional data
    &account,
).await?;

// Get contract information
let contract_state = contract_mgmt.get_contract(&contract_hash).await?;

// Update an existing contract
let update_result = contract_mgmt.update(
    &contract_hash,
    &new_aef_file,
    &new_manifest,
    None,  // Optional data
    &account,
).await?;
```

## Atipicial Name Service

The `NameService` interface provides access to Atipicial's domain name system:

```rust,no_run
// Create name service instance
let nns = NameService::new(client.clone());

// Resolve a domain to its address
let address = nns.resolve("example.atipicial").await?;

// Get domain owner
let owner = nns.get_owner("example.atipicial").await?;

// Register a new domain (if you're the root domain owner)
let tx_hash = nns.register("subdomain.example.atipicial", &recipient_hash, &account).await?;
```

## Raw Contract Invocation

For contracts without specific SDK interfaces, you can use raw invocation:

```rust,no_run
// Build a script to call the contract
let script = ScriptBuilder::build_contract_call(
    &contract_hash,
    "methodName",
    &[
        ContractParameter::string("parameter1"),
        ContractParameter::integer(42),
        ContractParameter::boolean(true),
    ],
)?;

// Create a transaction using the script
let transaction = TransactionBuilder::new()
    .script(script)
    .signers(vec![Signer::calledByEntry(account.get_script_hash())])
    .build()?;

// Sign and send the transaction
let signed_tx = transaction.sign(&client, &account).await?;
let tx_hash = client.send_raw_transaction(&signed_tx).await?;
```

## Test Invocation

You can test a contract call without sending a transaction:

```rust,no_run
// Test invoke a method
let result = client.invoke_function(
    &contract_hash,
    "methodName",
    &[
        ContractParameter::string("parameter1"),
        ContractParameter::integer(42),
    ],
    Some(&[Signer::calledByEntry(account.get_script_hash())]),
).await?;

// Process the result
if let Some(stack) = result.stack.first() {
    match stack {
        StackItem::Integer(value) => println!("Result: {}", value),
        StackItem::ByteString(bytes) => println!("Result: {:?}", bytes),
        // Handle other types...
        _ => println!("Unknown result type"),
    }
}
```

## Contract Events

Smart contracts can emit events that applications can listen for:

```rust,no_run
// Subscribe to contract events (requires WebSocket provider)
let mut events = client.subscribe_contract_event(&contract_hash).await?;

// Process events as they arrive
while let Some(event) = events.next().await {
    println!("Event: {} - {:?}", event.event_name, event.state);
    
    // Process specific events
    if event.event_name == "Transfer" {
        // Extract event parameters...
    }
}
```

## Related Documentation

- [Token Standards](token-standards.md): Detailed guide to AEP-17 and AEP-11 token standards
- [Contract Deployment](contract-deployment.md): Step-by-step guide to deploying smart contracts
- [Contract Invocation](contract-invocation.md): Guide to invoking contract methods
- [System Contracts](system-contracts.md): Documentation for Atipicial system contracts

---

> **Atipicial Chain** — sovereign Layer-1 for smart contracts and digital assets.
> 👑 Founded & engineered by **xmoohad** — Blockchain Scientist · Computer Programmer.
> `ATC` Atipicial Coin · `ATD` AtipicialDollar · addresses begin with **A**
