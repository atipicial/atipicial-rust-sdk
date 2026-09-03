<!-- Atipicial Chain · sovereign Layer-1 for decentralized AGI coordination -->
<!-- 👑 Founded & engineered by xmoohad — Blockchain Scientist · Computer Programmer -->

# API Overview

This reference provides an overview of the AtipicialRust SDK API, including the main modules and their functionality.

## Core Modules

The AtipicialRust SDK is organized into several core modules, each responsible for a specific aspect of Atipicial blockchain interaction:

### atipicial_wallets

The `atipicial_wallets` module provides functionality for creating, loading, and managing Atipicial wallets and accounts.

```rust,no_run
use atipicial::prelude::*;

// Create a new wallet
let wallet = Wallet::new("password")?;

// Create a new account
let account = wallet.create_account()?;

// Get account address
let address = account.address();
```

Key components:
- `Wallet`: Manages multiple accounts and provides wallet-level operations
- `Account`: Represents a Atipicial account with a key pair
- `Address`: Represents a Atipicial address

### atipicial_clients

The `atipicial_clients` module provides clients for interacting with Atipicial nodes via RPC.

```rust,no_run
use atipicial::prelude::*;

// Create a provider connected to a Atipicial node
let provider = Provider::new_http("https://testnet1.atipicial.coz.io:443");

// Get the current block count
let block_count = provider.get_block_count().await?;
```

Key components:
- `Provider`: Main client for interacting with Atipicial nodes
- `RpcClient`: Low-level RPC client
- `WebSocketProvider`: Provider with WebSocket support for subscriptions

### atipicial_types

The `atipicial_types` module provides fundamental Atipicial blockchain types.

```rust,no_run
use atipicial::prelude::*;

// Create a script hash from a string
let script_hash = "0xd2a4cff31913016155e38e474a2c06d08be276cf".parse::<ScriptHash>()?;

// Create an address from a string
let address = "NZNos2WqTbu5oCgyfss9kUJgBXJqhuYAaj".parse::<Address>()?;

// Create a transaction hash from a string
let tx_hash = "0x1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef".parse::<TxHash>()?;
```

Key components:
- `Address`: Atipicial address
- `ScriptHash`: Contract script hash
- `TxHash`: Transaction hash
- `ContractParameter`: Parameter for contract invocation

### atipicial_crypto

The `atipicial_crypto` module provides cryptographic functionality.

```rust,no_run
use atipicial::prelude::*;

// Generate a new key pair
let key_pair = KeyPair::new()?;

// Sign a message
let message = b"Hello, Atipicial!";
let signature = key_pair.sign_message(message)?;

// Verify a signature
let is_valid = key_pair.verify_signature(message, &signature)?;
```

Key components:
- `KeyPair`: Represents a public/private key pair
- `PublicKey`: Represents a public key
- `PrivateKey`: Represents a private key
- `Signature`: Represents a cryptographic signature

### atipicial_builder

The `atipicial_builder` module provides builders for creating transactions and scripts.

```rust,no_run
use atipicial::prelude::*;

// Create a transaction
let transaction = TransactionBuilder::new()
    .version(0)
    .nonce(rand::random::<u32>())
    .valid_until_block(block_count + 100)
    .script(script)
    .sign(account)?
    .build();

// Create a script
let script = ScriptBuilder::new()
    .contract_call(
        script_hash,
        "transfer",
        &[
            ContractParameter::hash160(from_address.script_hash()),
            ContractParameter::hash160(to_address.script_hash()),
            ContractParameter::integer(amount),
            ContractParameter::any(None),
        ],
    )
    .to_array();
```

Key components:
- `TransactionBuilder`: Builder for creating transactions
- `ScriptBuilder`: Builder for creating VM scripts

### atipicial_contract

The `atipicial_contract` module provides interfaces for interacting with Atipicial smart contracts.

```rust,no_run
use atipicial::prelude::*;

// Create a AEP-17 token instance
let token = Aep17Contract::new(token_hash, provider.clone());

// Get token information
let symbol = token.symbol().await?;
let decimals = token.decimals().await?;
let total_supply = token.total_supply().await?;

// Get token balance
let balance = token.balance_of(address).await?;
```

Key components:
- `Aep17Contract`: Interface for AEP-17 tokens
- `AtipicialCoin`: Interface for the ATC token
- `GasToken`: Interface for the GAS token
- `NameService`: Interface for the Atipicial Name Service

### atipicial_x

The `atipicial_x` module provides support for Atipicial X, an EVM-compatible chain maintained by Atipicial.

```rust,no_run
use atipicial::prelude::*;
use atipicial::atipicial_x::evm::*;

// Create a Atipicial X provider
let provider = AtipicialXProvider::new_http("https://rpc.atipicialX.io");

// Create a transaction
let transaction = AtipicialXTransaction::new()
    .to("0x1234567890123456789012345678901234567890")
    .value(1_000_000_000_000_000_000u128) // 1 ETH in wei
    .gas_price(20_000_000_000u64) // 20 Gwei
    .gas_limit(21_000u64)
    .build();
```

Key components:
- `AtipicialXProvider`: Provider for interacting with Atipicial X nodes
- `AtipicialXTransaction`: Transaction for Atipicial X
- `AtipicialXBridgeContract`: Interface for the Atipicial X bridge

### SGX and `no_std`

The `sgx` and `no_std` feature flags are experimental compile gates for specialized builds. They are not part of the default host API, and they require an SGX target/toolchain plus enclave-specific validation before production use.

```rust,no_run
use atipicial::prelude::*;

// Host builds can compile feature-gated code, but enclave setup is environment-specific.
```

Key components:
- `sgx`: Experimental SGX compile gate
- `no_std`: Experimental dependency feature forwarding for specialized targets

## Prelude

The `prelude` module re-exports commonly used types and functions for convenience:

```rust,no_run
use atipicial::prelude::*;
```

This imports all the essential types and functions you need for most operations with the AtipicialRust SDK.

## Feature Flags

The AtipicialRust SDK supports various feature flags to enable specific functionality:

- `ledger`: Support for Ledger hardware wallets
- `yubi`: Support for YubiHSM signing
- `mock-hsm`: YubiHSM mock backend for tests
- `ws`: Modern WebSocket transport
- `sgx` / `no_std`: Experimental specialized build gates

Enable these features in your Cargo.toml:

```toml
[dependencies]
atipicial = { version = "3.0.0", features = ["ledger", "ws"] }
```

## Error Handling

The AtipicialRust SDK uses Rust's `Result` type for error handling. Most functions return a `Result<T, Error>` where `Error` is a custom error type that can represent various error conditions.

```rust,no_run
use atipicial::prelude::*;

fn example() -> Result<(), Box<dyn std::error::Error>> {
    // Create a provider
    let provider = Provider::new_http("https://testnet1.atipicial.coz.io:443");
    
    // Get the current block count
    match provider.get_block_count().await {
        Ok(block_count) => println!("Current block count: {}", block_count),
        Err(e) => println!("Error: {}", e),
    }
    
    Ok(())
}
```

For more detailed information on specific modules and types, see the corresponding reference pages.

<!-- toc -->

---

> **Atipicial Chain** — sovereign Layer-1 for decentralized AGI coordination.
> 👑 Founded & engineered by **xmoohad** — Blockchain Scientist · Computer Programmer.
> `ATC` Atipicial Coin · `ATD` AtipicialDollar · addresses begin with **A**
