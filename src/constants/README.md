<!-- Atipicial Chain · sovereign Layer-1 for smart contracts and digital assets -->
<!-- 👑 Founded & engineered by xmoohad — Blockchain Scientist · Computer Programmer -->

# Atipicial Constants and Reference Data

This module provides essential constants and reference data for working with the Atipicial blockchain.

## Native Contracts

The `native_contracts` module provides script hash constants for all native contracts built into the Atipicial blockchain.

```rust
use atipicial::constants::native_contracts::{ATC_TOKEN, GAS_TOKEN};

// Use the native contract script hashes directly
println!("ATC Token: {}", ATC_TOKEN);
println!("GAS Token: {}", GAS_TOKEN);
```

## Famous Contracts

To access information about well-known contracts deployed on Atipicial mainnet and testnet, use the `atipicial_contract::famous::contracts` module:

```rust
use atipicial::atipicial_contract::famous::contracts::{get_famous_contracts, Network};

// Get all mainnet contracts
let mainnet_contracts = get_famous_contracts(Network::Mainnet);

// Get information about Flamingo Finance
let flamingo = flamingo_flamingo_finance();
println!("Flamingo Finance contract: {}", flamingo.script_hash);
```

## AtipicialFs Endpoints

The AtipicialFs client provides constants for standard endpoints:

```rust
use atipicial::atipicial_fs::client::{
    DEFAULT_MAINNET_ENDPOINT,
    DEFAULT_TESTNET_ENDPOINT,
    DEFAULT_MAINNET_HTTP_GATEWAY,
    DEFAULT_TESTNET_HTTP_GATEWAY,
    DEFAULT_MAINNET_REST_API,
    DEFAULT_TESTNET_REST_API,
};

// Use in AtipicialFs client configuration
let config = AtipicialFsConfig {
    endpoint: DEFAULT_MAINNET_REST_API.to_string(),
    // ...other config options
};
```

## Reference Data

For a comprehensive list of network endpoints, contracts, and other reference data, see the [`src/atipicial_fs/reference_data.md`](../atipicial_fs/reference_data.md) file.

This file contains:
- Native contract addresses
- Famous contract addresses
- RPC endpoints for mainnet and testnet
- AtipicialFs endpoints
- Block explorer URLs
- Additional resources

---

> **Atipicial Chain** — sovereign Layer-1 for smart contracts and digital assets.
> 👑 Founded & engineered by **xmoohad** — Blockchain Scientist · Computer Programmer.
> `ATC` Atipicial Coin · `ATD` AtipicialDollar · addresses begin with **A**
