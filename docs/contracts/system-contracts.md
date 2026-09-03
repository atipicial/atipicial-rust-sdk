<!-- Atipicial Chain · sovereign Layer-1 for decentralized AGI coordination -->
<!-- 👑 Founded & engineered by xmoohad — Blockchain Scientist · Computer Programmer -->

# Atipicial System Contracts

## Overview

Atipicial includes several native system contracts that manage core blockchain functionality. These contracts are special in that they're built directly into the Atipicial blockchain and don't need to be deployed by users. The AtipicialRust SDK provides dedicated interfaces for interacting with these system contracts.

## AtipicialCoin Contract

The AtipicialCoin contract manages the ATC governance token, which represents ownership in the Atipicial network and gives voting rights for consensus nodes.

### Key Functions

```rust,no_run
use atipicial::prelude::*;
use atipicial::atipicial_contract::AtipicialCoin;

async fn atipicial_token_example() -> Result<(), Box<dyn std::error::Error>> {
    // Connect to Atipicial
    let provider = HttpProvider::new("https://mainnet1.atipicial.com:443")?;
    let client = RpcClient::new(provider);
    
    // Create AtipicialCoin instance
    let atipicial_token = AtipicialCoin::new(&client);
    
    // Get basic token information
    let symbol = atipicial_token.symbol().await?;
    let decimals = atipicial_token.decimals().await?;
    let total_supply = atipicial_token.total_supply().await?;
    
    println!("Token: {} (Decimals: {})", symbol, decimals);
    println!("Total Supply: {}", total_supply);
    
    // Check ATC balance for an account
    let account_hash = ScriptHash::from_address("NUVPACTpQvd2HHmBgFjJJRWwVXJiR3uAEh")?;
    let balance = atipicial_token.balance_of(&account_hash).await?;
    println!("ATC Balance: {}", balance);
    
    // Get committee members (council)
    let committee = atipicial_token.get_committee().await?;
    println!("Committee Members: {}", committee.len());
    for member in committee {
        println!("  {}", member);
    }
    
    // Get all candidates for committee election
    let candidates = atipicial_token.get_all_candidates().await?;
    println!("Committee Candidates: {}", candidates.len());
    for (candidate, votes) in candidates {
        println!("  {} - {} votes", candidate, votes);
    }
    
    // Get unclaimed GAS
    let account_hash = ScriptHash::from_address("NUVPACTpQvd2HHmBgFjJJRWwVXJiR3uAEh")?;
    let block_height = client.get_block_count().await?;
    let unclaimed_gas = atipicial_token.get_unclaimed_gas(&account_hash, block_height).await?;
    println!("Unclaimed GAS: {}", unclaimed_gas);
    
    Ok(())
}
```

### Voting

ATC holders can vote for consensus nodes:

```rust,no_run
// Vote for a candidate (requires account with ATC)
let account = Account::from_wif("your-wif-here")?;
let candidate_pubkey = "03e47f3e9809da7c34a3f7a058f4b4bb833416d0d5b597dc81798895f1bd2f27bd";
let tx_hash = atipicial_token.vote(&account, candidate_pubkey).await?;
println!("Vote transaction: {}", tx_hash);

// Cancel voting
let tx_hash = atipicial_token.vote(&account, "").await?;
println!("Cancel vote transaction: {}", tx_hash);
```

## GasToken Contract

The GasToken contract manages the GAS utility token, which is used to pay for transaction fees and smart contract execution.

### Key Functions

```rust,no_run
use atipicial::prelude::*;
use atipicial::atipicial_contract::GasToken;

async fn gas_token_example() -> Result<(), Box<dyn std::error::Error>> {
    // Connect to Atipicial
    let provider = HttpProvider::new("https://mainnet1.atipicial.com:443")?;
    let client = RpcClient::new(provider);
    
    // Create GasToken instance
    let mut gas_token = GasToken::new(Some(&client));
    
    // Get basic token information
    let symbol = gas_token.get_symbol().await?;
    let decimals = gas_token.get_decimals().await?;
    let total_supply = gas_token.get_total_supply().await?;
    
    println!("Token: {} (Decimals: {})", symbol, decimals);
    println!("Total Supply: {}", total_supply);
    
    // Check GAS balance for an account
    let account_hash = ScriptHash::from_address("NUVPACTpQvd2HHmBgFjJJRWwVXJiR3uAEh")?;
    let balance = gas_token.get_balance_of(&account_hash).await?;
    println!("GAS Balance: {}", balance);
    
    // Transfer GAS (requires account with GAS)
    // let account = Account::from_wif("your-wif-here")?;
    // let recipient = ScriptHash::from_address("NZNos2WqTbu5oCgyfss9kUJgBXJqhuYAaj")?;
    // let amount = 1_0000_0000; // 1 GAS (with 8 decimals)
    // let tx_builder = gas_token.transfer_from_account(&account, &recipient, amount, None).await?;
    // println!("Transfer transaction builder prepared");
    
    Ok(())
}
```

## PolicyContract

The PolicyContract manages network policy parameters, which control aspects of the Atipicial blockchain like fees and transaction validation.

### Key Functions

```rust,no_run
use atipicial::prelude::*;
use atipicial::atipicial_contract::PolicyContract;

async fn policy_contract_example() -> Result<(), Box<dyn std::error::Error>> {
    // Connect to Atipicial
    let provider = HttpProvider::new("https://mainnet1.atipicial.com:443")?;
    let client = RpcClient::new(provider);
    
    // Create PolicyContract instance
    let policy = PolicyContract::new(Some(&client));
    
    // Get blockchain fees
    let exec_fee_factor = policy.get_exec_fee_factor().await?;
    let storage_price = policy.get_storage_price().await?;
    
    println!("Execution Fee Factor: {}", exec_fee_factor);
    println!("Storage Price: {}", storage_price);
    
    // Get system attributes
    let block_account = policy.get_block_account().await?;
    let max_block_size = policy.get_max_block_size().await?;
    let max_block_system_fee = policy.get_max_block_system_fee().await?;
    
    println!("Block Account: {}", block_account);
    println!("Max Block Size: {}", max_block_size);
    println!("Max Block System Fee: {}", max_block_system_fee);
    
    // Check if script is allowed
    let script_hash = ScriptHash::from_address("NUVPACTpQvd2HHmBgFjJJRWwVXJiR3uAEh")?;
    let is_blocked = policy.is_blocked(&script_hash).await?;
    println!("Is script blocked: {}", is_blocked);
    
    Ok(())
}
```

## ContractManagement

The ContractManagement contract manages the deployment, update, and destruction of smart contracts on the Atipicial blockchain.

### Key Functions

```rust,no_run
use atipicial::prelude::*;
use atipicial::atipicial_contract::ContractManagement;
use atipicial::atipicial_types::{ContractManifest, AefFile};
use std::fs;

async fn contract_management_example() -> Result<(), Box<dyn std::error::Error>> {
    // Connect to Atipicial
    let provider = HttpProvider::new("https://mainnet1.atipicial.com:443")?;
    let client = RpcClient::new(provider);
    
    // Create ContractManagement instance
    let contract_mgmt = ContractManagement::new(Some(&client));
    
    // Get contract details
    let contract_hash = ScriptHash::from_address("NUVPACTpQvd2HHmBgFjJJRWwVXJiR3uAEh")?;
    let contract = contract_mgmt.get_contract(&contract_hash).await?;
    
    println!("Contract ID: {}", contract.id);
    println!("Contract Hash: {}", contract.hash);
    println!("Contract Update Counter: {}", contract.update_counter);
    
    // Deploy a contract (requires account with GAS)
    // let account = Account::from_wif("your-wif-here")?;
    // 
    // // Load contract files
    // let aef_bytes = fs::read("path/to/contract.aef")?;
    // let manifest_json = fs::read_to_string("path/to/contract.manifest.json")?;
    // 
    // // Parse contract files
    // let aef = AefFile::from_bytes(&aef_bytes)?;
    // let manifest = ContractManifest::from_json(&manifest_json)?;
    // 
    // // Deploy the contract
    // let result = contract_mgmt.deploy(
    //     &aef,
    //     &manifest,
    //     None, // No data
    //     &account,
    // ).await?;
    // 
    // println!("Contract deployed: {}", result.script_hash);
    
    Ok(())
}
```

## RoleManagement

The RoleManagement contract manages consensus roles on the Atipicial blockchain, including Oracle nodes and state validators.

### Key Functions

```rust,no_run
use atipicial::prelude::*;
use atipicial::atipicial_contract::RoleManagement;

async fn role_management_example() -> Result<(), Box<dyn std::error::Error>> {
    // Connect to Atipicial
    let provider = HttpProvider::new("https://mainnet1.atipicial.com:443")?;
    let client = RpcClient::new(provider);
    
    // Create RoleManagement instance
    let role_mgmt = RoleManagement::new(Some(&client));
    
    // Get nodes by role
    let oracle_nodes = role_mgmt.get_designated_by_role(Role::Oracle, 0).await?;
    let state_validators = role_mgmt.get_designated_by_role(Role::StateValidator, 0).await?;
    
    println!("Oracle Nodes: {}", oracle_nodes.len());
    for node in oracle_nodes {
        println!("  {}", node);
    }
    
    println!("State Validators: {}", state_validators.len());
    for validator in state_validators {
        println!("  {}", validator);
    }
    
    Ok(())
}
```

## NameService

The NameService contract manages domain name registration and resolution on the Atipicial blockchain, similar to DNS.

### Key Functions

```rust,no_run
use atipicial::prelude::*;
use atipicial::atipicial_contract::NameService;

async fn name_service_example() -> Result<(), Box<dyn std::error::Error>> {
    // Connect to Atipicial
    let provider = HttpProvider::new("https://mainnet1.atipicial.com:443")?;
    let client = RpcClient::new(provider);
    
    // Create NameService instance
    let nns = NameService::new(&client);
    
    // Resolve a domain name
    let domain = "atipicial.atipicial";
    if let Ok(address) = nns.resolve(domain).await {
        println!("{} resolves to: {}", domain, address);
        
        // Get domain owner
        let owner = nns.get_owner(domain).await?;
        println!("{} is owned by: {}", domain, owner);
        
        // Get domain expiration
        let expiration = nns.get_expiration(domain).await?;
        println!("{} expires at block: {}", domain, expiration);
    } else {
        println!("{} is not registered", domain);
    }
    
    // Register a domain (requires account with GAS and ownership of parent domain)
    // let account = Account::from_wif("your-wif-here")?;
    // let subdomain = "mydomain.atipicial";
    // let recipient = ScriptHash::from_address("NZNos2WqTbu5oCgyfss9kUJgBXJqhuYAaj")?;
    // let tx_hash = nns.register(subdomain, &recipient, &account).await?;
    // println!("Domain registration transaction: {}", tx_hash);
    
    Ok(())
}
```

## Oracle Contract

The Oracle contract provides external data to smart contracts on the Atipicial blockchain, allowing them to access information from outside the blockchain environment.

### Key Functions

```rust,no_run
use atipicial::prelude::*;
use atipicial::atipicial_contract::OracleContract;

async fn oracle_contract_example() -> Result<(), Box<dyn std::error::Error>> {
    // Connect to Atipicial
    let provider = HttpProvider::new("https://mainnet1.atipicial.com:443")?;
    let client = RpcClient::new(provider);
    
    // Create OracleContract instance
    let oracle = OracleContract::new(&client);
    
    // Get oracle request price
    let request_price = oracle.get_price().await?;
    println!("Oracle request price: {} GAS", request_price as f64 / 100_000_000.0);
    
    // Request external data (requires account with GAS)
    // let account = Account::from_wif("your-wif-here")?;
    // let url = "https://api.example.com/data";
    // let filter = "$.price";
    // let callback_contract = ScriptHash::from_address("NZNos2WqTbu5oCgyfss9kUJgBXJqhuYAaj")?;
    // let callback_method = "onOracleResponse";
    // let user_data = "custom_data".as_bytes().to_vec();
    // 
    // let tx_hash = oracle.request(
    //     &account,
    //     url,
    //     filter,
    //     &callback_contract,
    //     callback_method,
    //     Some(user_data),
    // ).await?;
    // 
    // println!("Oracle request transaction: {}", tx_hash);
    
    Ok(())
}
```

## Interoperability

System contracts can be used together for more complex operations:

```rust,no_run
async fn complex_example() -> Result<(), Box<dyn std::error::Error>> {
    // Connect to Atipicial
    let provider = HttpProvider::new("https://mainnet1.atipicial.com:443")?;
    let client = RpcClient::new(provider);
    
    // Create instances
    let atipicial_token = AtipicialCoin::new(&client);
    let gas_token = GasToken::new(&client);
    let nns = NameService::new(&client);
    
    // Resolve a domain and check balances
    let domain = "example.atipicial";
    if let Ok(address) = nns.resolve(domain).await {
        let script_hash = ScriptHash::from_address(&address)?;
        
        // Get ATC and GAS balances
        let atipicial_balance = atipicial_token.balance_of(&script_hash).await?;
        let gas_balance = gas_token.balance_of(&script_hash).await?;
        
        println!("{} ({})", domain, address);
        println!("ATC Balance: {}", atipicial_balance);
        println!("GAS Balance: {}", gas_balance);
        
        // Check if this account is voting
        if let Ok(candidate) = atipicial_token.get_candidate_vote_by_account(&script_hash).await {
            println!("Voting for: {}", candidate);
        } else {
            println!("Not voting for any candidate");
        }
    }
    
    Ok(())
}
```

---

> **Atipicial Chain** — sovereign Layer-1 for decentralized AGI coordination.
> 👑 Founded & engineered by **xmoohad** — Blockchain Scientist · Computer Programmer.
> `ATC` Atipicial Coin · `ATD` AtipicialDollar · addresses begin with **A**
