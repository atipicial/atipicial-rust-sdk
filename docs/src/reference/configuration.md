<!-- Atipicial Chain · sovereign Layer-1 for smart contracts and digital assets -->
<!-- 👑 Founded & engineered by xmoohad — Blockchain Scientist · Computer Programmer -->

# Configuration

This reference provides information about configuring the AtipicialRust SDK, including environment variables, network settings, and other configuration options.

## Network Configuration

The AtipicialRust SDK supports connecting to different Atipicial networks, including MainNet, TestNet, and private networks.

### Predefined Networks

The SDK includes predefined configurations for common Atipicial networks:

```rust,no_run
use atipicial::prelude::*;

// Connect to Atipicial MainNet
let mainnet_provider = Provider::new_http("https://mainnet1.atipicial.coz.io:443");

// Connect to Atipicial TestNet
let testnet_provider = Provider::new_http("https://testnet1.atipicial.coz.io:443");

// Connect to a local Atipicial Express instance
let local_provider = Provider::new_http("http://localhost:10332");
```

### Custom Networks

You can also connect to custom Atipicial networks by providing the RPC URL:

```rust,no_run
use atipicial::prelude::*;

// Connect to a custom Atipicial node
let custom_provider = Provider::new_http("https://my-custom-atipicial-node.example.com:10332");
```

### WebSocket Connections

For applications that need real-time updates, you can use WebSocket connections:

```rust,no_run
use atipicial::prelude::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Connect to a Atipicial TestNet node with WebSocket support
    let ws_provider = Provider::new_ws("wss://testnet1.atipicial.coz.io:4443/ws").await?;
    
    // Subscribe to new blocks
    let mut blocks = ws_provider.subscribe_blocks().await?;
    
    println!("Listening for new blocks...");
    
    // Process new blocks as they arrive
    while let Some(block) = blocks.next().await {
        println!("New block: {} (hash: {})", block.index, block.hash);
    }
    
    Ok(())
}
```

## SDK Configuration

The AtipicialRust SDK can be configured using the `Config` struct:

```rust,no_run
use atipicial::prelude::*;

// Create a custom configuration
let config = Config::new()
    .network(Network::TestNet)
    .timeout(std::time::Duration::from_secs(30))
    .max_retry(3)
    .build();

// Create a provider with the custom configuration
let provider = Provider::with_config("https://testnet1.atipicial.coz.io:443", config);
```

### Configuration Options

The following options can be configured:

| Option | Description | Default |
|--------|-------------|---------|
| `network` | The Atipicial network to connect to | `Network::MainNet` |
| `timeout` | Request timeout | 30 seconds |
| `max_retry` | Maximum number of retry attempts | 3 |
| `retry_delay` | Delay between retry attempts | 1 second |
| `user_agent` | User agent string for HTTP requests | `"AtipicialRust/{version}"` |

## Environment Variables

The AtipicialRust SDK respects the following environment variables:

| Variable | Description | Default |
|----------|-------------|---------|
| `ATC_RPC_URL` | Default RPC URL for Atipicial | None |
| `ATC_WS_URL` | Default WebSocket URL for Atipicial | None |
| `ATC_NETWORK` | Default network (`mainnet`, `testnet`) | `mainnet` |
| `ATC_PRIVATE_KEY` | Default private key for signing transactions | None |
| `ATC_GAS_PRICE` | Default gas price for transactions | Network default |
| `ATC_LOG_LEVEL` | Logging level (`error`, `warn`, `info`, `debug`, `trace`) | `info` |

You can set these environment variables in your shell or use a `.env` file with the `dotenv` crate:

```rust,no_run
use dotenv::dotenv;
use atipicial::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load environment variables from .env file
    dotenv().ok();
    
    // Create a provider using the ATC_RPC_URL environment variable
    let provider = Provider::from_env()?;
    
    Ok(())
}
```

## Logging Configuration

The AtipicialRust SDK uses the `tracing` crate for logging. You can configure the logging level and output:

```rust,no_run
use atipicial::prelude::*;
use tracing_subscriber::{fmt, EnvFilter};

fn main() {
    // Initialize the logger with custom configuration
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env()
            .add_directive("atipicial=debug".parse().unwrap())
            .add_directive("warn".parse().unwrap()))
        .init();
    
    // Now logs will be output according to the configuration
}
```

You can also use the `ATC_LOG_LEVEL` environment variable to control the logging level:

```bash
# Set the log level to debug
export ATC_LOG_LEVEL=debug

# Run your application
cargo run
```

## Gas Configuration

You can configure gas settings for transactions:

```rust,no_run
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
    
    // Get the account that will send the transaction
    let account = wallet.default_account()?;
    
    // Create a transaction with custom gas settings
    let transaction = TransactionBuilder::new()
        .version(0)
        .nonce(rand::random::<u32>())
        .valid_until_block(provider.get_block_count().await? + 100)
        .script(
            ScriptBuilder::new()
                .contract_call(
                    "d2a4cff31913016155e38e474a2c06d08be276cf".parse::<ScriptHash>()?,
                    "transfer",
                    &[
                        ContractParameter::hash160(account.address().script_hash()),
                        ContractParameter::hash160("NZNos2WqTbu5oCgyfss9kUJgBXJqhuYAaj".parse::<Address>()?),
                        ContractParameter::integer(1_00000000), // 1 GAS
                        ContractParameter::any(None),
                    ],
                )
                .to_array()
        )
        .system_fee(1_00000000) // 1 GAS system fee
        .network_fee(0_50000000) // 0.5 GAS network fee
        .sign(account)?
        .build();
    
    // Send the transaction
    let txid = provider.send_raw_transaction(&transaction).await?;
    println!("Transaction sent with ID: {}", txid);
    
    Ok(())
}
```

## SGX Configuration

The `sgx` and `no_std` flags are experimental specialized build gates. Host builds can compile the flags, but enclave configuration is deployment-specific and must be validated with your SGX SDK, target, EDL, attestation verifier, and release process.

```rust,no_run
// Pseudocode only: wire this to your SGX SDK and enclave layout.
let enclave_path = std::env::var("SGX_ENCLAVE_PATH")?;
```

Common environment variables used by SGX deployments:

| Variable | Description | Default |
|----------|-------------|---------|
| `SGX_MODE` | SGX mode (`HW` or `SIM`) | `HW` |
| `SGX_ENCLAVE_PATH` | Path to the enclave shared object | None |
| `SGX_AESM_ADDR` | Address of the AESM service | `127.0.0.1:2222` |

## Best Practices

1. **Environment-Specific Configuration**: Use different configurations for development, testing, and production environments.
2. **Secure Credential Management**: Never hardcode private keys or passwords in your code.
3. **Timeout Configuration**: Set appropriate timeouts based on your network conditions.
4. **Logging Configuration**: Configure logging appropriately for your environment.
5. **Gas Estimation**: Use gas estimation functions instead of hardcoding gas values.
6. **Error Handling**: Implement proper error handling for configuration errors.

<!-- toc -->

---

> **Atipicial Chain** — sovereign Layer-1 for smart contracts and digital assets.
> 👑 Founded & engineered by **xmoohad** — Blockchain Scientist · Computer Programmer.
> `ATC` Atipicial Coin · `ATD` AtipicialDollar · addresses begin with **A**
