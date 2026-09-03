// Clippy suppressions — each kept intentionally:
#![allow(clippy::result_large_err)]
// RPC error enums carry provider-specific payloads; boxing adds indirection with no consumer beaefit
#![allow(clippy::too_many_arguments)] // Builder-pattern constructors mirror the Atipicial protocol fields 1:1
#![allow(clippy::wrong_self_convention)] // `into_*` / `to_*` naming follows Atipicial C# SDK conventions for cross-language parity
#![allow(clippy::module_inception)] // Re-export modules share parent name by design (e.g. atipicial_types::atipicial_types)
#![allow(clippy::type_complexity)] // Nested generics in RPC futures are unavoidable without boxing
#![allow(unexpected_cfgs)] // Feature flags checked at build time by downstream crates
//! ![Atipicial Logo](https://atipicial.com/images/atipicial-logo/ATC-logo.svg)
//! # AtipicialRust SDK
//!
//! A production-focused Rust SDK for the Atipicial blockchain.
//!
//! [![Crates.io](https://img.shields.io/crates/v/atipicial.svg)](https://crates.io/crates/atipicial)
//! [![Documentation](https://docs.rs/atipicial/badge.svg)](https://docs.rs/atipicial)
//! [![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
//!
//! ## At a glance
//!
//! ```no_run
//! use atipicial::sdk::Atipicial;
//!
//! # #[tokio::main]
//! # async fn main() -> Result<(), Box<dyn std::error::Error>> {
//! // Quick start: connect to a public TestNet node.
//! let atipicial = Atipicial::testnet().await?;
//!
//! // Or read $ATC_RPC_URL from the environment (TestNet fallback).
//! let atipicial = Atipicial::from_env().await?;
//!
//! // Or point at any RPC endpoint, including private nodes.
//! let atipicial = Atipicial::connect("https://my-node.example.com:443").await?;
//!
//! let height = atipicial.get_block_height().await?;
//! println!("Tip: {}", height);
//! # Ok(()) }
//! ```
//!
//! Every fallible operation returns
//! [`atipicial_error::unified::AtipicialError`][crate::atipicial_error::unified::AtipicialError], which
//! exposes
//! [`kind()`][crate::atipicial_error::unified::AtipicialError::kind] and
//! [`is_retryable()`][crate::atipicial_error::unified::AtipicialError::is_retryable] for
//! ergonomic retry/telemetry decisions — the same convention used by the AWS
//! Rust SDK's `ProvideErrorMetadata` trait. See
//! [`atipicial_error::unified::AtipicialErrorKind`][crate::atipicial_error::unified::AtipicialErrorKind]
//! for the stable error taxonomy.
//!
//! ## Choosing an API layer
//!
//! The crate is structured as two complementary layers:
//!
//! 1. **High-level (`atipicial::sdk`)** — opinionated, batteries-included client
//!    (`Atipicial`, `AtipicialBuilder`, `SdkConfig`, `Token`, `Balance`). Designed for
//!    application code that wants idiomatic builders, automatic retry, and
//!    cached lookups out of the box.
//! 2. **Low-level (`atipicial_clients`, `atipicial_builder`, `atipicial_protocol`, `atipicial_crypto`,
//!    `atipicial_wallets`, …)** — direct access to JSON-RPC, transaction
//!    construction, signing, and cryptography. Use this when you need full
//!    control over byte-level Atipicial protocol details.
//!
//! Most users should start with `atipicial::sdk` and drop down to the lower-level
//! modules only for advanced cases.
//!
//! For a full decision table and concrete "when to switch" rules, see the
//! [Choosing an API Layer](https://github.com/R3E-Network/AtipicialRust/blob/master/docs/guides/choosing-an-api.md) guide.
//!
//! ## Features
//!
//! This crate provides several feature flags to customize functionality:
//!
//! - **futures**: Enables async/futures support for asynchronous blockchain operations. This is recommended
//!   for most applications that need to interact with the Atipicial blockchain without blocking.
//!
//! - **ledger**: Enables Ledger hardware wallet types. The feature gate is compile-checked, but
//!   production use should include real-device signing tests in your release environment.
//!
//! - **yubi** / **mock-hsm**: Enables YubiHSM support, or the YubiHSM mock backend for tests.
//!
//! - **sgx**: Experimental Intel SGX compile gate. Host builds compile the flag, but enclave
//!   builds require an SGX target/toolchain and dedicated validation.
//!
//! - **no_std**: Experimental dependency feature forwarding for specialized builds. It is not
//!   certified as general embedded or `wasm32-unknown-unknown` support.
//!
//! To enable specific features in your project, modify your `Cargo.toml` as follows:
//!
//! ```toml
//! [dependencies]
//! atipicial = { version = "1.2", features = ["futures", "ledger"] }
//! ```
//!
//! You can disable default features with:
//!
//! ```toml
//! atipicial = { version = "1.2", default-features = false, features = ["futures"] }
//! ```
//!
//! ## Overview
//!
//! AtipicialRust is a complete SDK designed to make Atipicial blockchain development in Rust
//! intuitive, type-safe, and productive. The library provides full support for all
//! Atipicial features and follows Rust best practices for reliability and performance.
//!
//! ### New in v1.0.x
//! - **WebSocket Support**: Real-time blockchain events with automatic reconnection
//! - **HD Wallets (BIP-39/44)**: Deterministic wallet generation and derivation
//! - **Transaction Simulation**: Preview fees, VM state, and state changes before sending
//! - **High-Level SDK API**: Simplified entrypoint (`Atipicial`) for common operations
//! - **Enhanced Error Handling**: Consistent `AtipicialError` with recovery suggestions
//!
//! ## Core Modules
//!
//! AtipicialRust is organized into specialized modules, each handling specific aspects of Atipicial:
//!
//! - [**atipicial_builder**](atipicial_builder): Transaction construction and script building
//! - [**atipicial_clients**](atipicial_clients): Atipicial node interaction and RPC client implementations
//! - [**atipicial_codec**](atipicial_codec): Serialization and deserialization of Atipicial data structures
//! - [**atipicial_config**](atipicial_config): Configuration for networks and client settings
//! - [**atipicial_contract**](atipicial_contract): Smart contract interaction and token standards
//! - [**atipicial_crypto**](atipicial_crypto): Cryptographic primitives and operations
//! - [**atipicial_error**](atipicial_error): Unified error handling
//! - [**atipicial_fs**](atipicial_fs): AtipicialFs distributed storage system integration
//! - [**atipicial_protocol**](atipicial_protocol): Core blockchain protocol implementations
//! - [**atipicial_types**](atipicial_types): Core data types and primitives for Atipicial
//! - [**atipicial_utils**](atipicial_utils): General utility functions
//! - [**atipicial_wallets**](atipicial_wallets): Wallet management for Atipicial
//! - [**atipicial_x**](atipicial_x): Atipicial X EVM compatibility layer
//!
//! ## Quick Start
//!
//! Import all essential types and traits using the `prelude`:
//!
//! ```rust
//! use atipicial::prelude::*;
//! ```
//!
//! ## Complete Example
//!
//! Here's a comprehensive example showcasing common operations with the AtipicialRust SDK:
//!
//! ```no_run
//! use atipicial::atipicial_protocol::{Account, AccountTrait};
//! use atipicial::atipicial_clients::{HttpProvider, RpcClient, APITrait};
//!
//! async fn atipicial_example() -> Result<(), Box<dyn std::error::Error>> {
//!     // Connect to Atipicial TestNet
//!     let provider = HttpProvider::new("https://testnet1.atipicial.com:443")?;
//!     let client = RpcClient::new(provider);
//!     
//!     // Get basic blockchain information
//!     let block_height = client.get_block_count().await?;
//!     println!("Connected to Atipicial TestNet at height: {}", block_height);
//!     
//!     // Create a new wallet account
//!     let account = Account::create()?;
//!     println!("New account created:");
//!     println!("  Address:     {}", account.get_address());
//!     println!("  Script Hash: {}", account.get_script_hash());
//!     
//!     // Get version information
//!     let version = client.get_version().await?;
//!     println!("Node version: {}", version.user_agent);
//!     
//!     Ok(())
//! }
//! ```
//!
//! ## Usage Examples
//!
//! ### Connecting to a Atipicial node
//!
//! ```no_run
//! use atipicial::atipicial_clients::{HttpProvider, RpcClient, APITrait};
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     // Connect to Atipicial MainNet
//!     let provider = HttpProvider::new("https://mainnet1.atipicial.com:443")?;
//!     let client = RpcClient::new(provider);
//!     
//!     // Get basic blockchain information
//!     let block_count = client.get_block_count().await?;
//!     println!("Current block count: {}", block_count);
//!     
//!     let version = client.get_version().await?;
//!     println!("Node version: {}", version.user_agent);
//!     
//!     Ok(())
//! }
//! ```
//!
//! ### Creating and sending a transaction
//!
//! ```no_run
//! use atipicial::atipicial_clients::{HttpProvider, RpcClient, APITrait};
//! use atipicial::atipicial_protocol::{Account, AccountTrait};
//! use atipicial::atipicial_types::{ScriptHash, ContractParameter, ScriptHashExtension};
//! use atipicial::atipicial_builder::{ScriptBuilder, TransactionBuilder, AccountSigner};
//! use std::str::FromStr;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     // Initialize the JSON-RPC provider
//!     let provider = HttpProvider::new("https://testnet1.atipicial.com:443")?;
//!     let client = RpcClient::new(provider);
//!
//!     // Create accounts for the sender and recipient
//!     // Load sender key from an environment variable to avoid hardcoding secrets.
//!     let sender_wif = std::env::var("ATC_WIF")?;
//!     let sender = Account::from_wif(&sender_wif)?;
//!     let recipient = ScriptHash::from_address("NbTiM6h8r99kpRtb428XcsUk1TzKed2gTc")?;
//!
//!     // Get the GAS token contract
//!     let gas_token_hash = ScriptHash::from_str("d2a4cff31913016155e38e474a2c06d08be276cf")?;
//!     
//!     // Build the transaction using the ScriptBuilder
//!     let script = ScriptBuilder::new()
//!         .contract_call(
//!             &gas_token_hash,
//!             "transfer",
//!             &[
//!                 ContractParameter::h160(&sender.get_script_hash()),
//!                 ContractParameter::h160(&recipient),
//!                 ContractParameter::integer(1_0000_0000), // 1 GAS (8 decimals)
//!                 ContractParameter::any(),
//!             ],
//!             None,
//!         )?
//!         .to_bytes();
//!     
//!     // Create and configure the transaction
//!     let mut tx_builder = TransactionBuilder::with_client(&client);
//!     tx_builder
//!         .set_script(Some(script))
//!         .set_signers(vec![AccountSigner::called_by_entry(&sender)?.into()])?
//!         .valid_until_block(client.get_block_count().await? + 5760)?; // Valid for ~1 day
//!
//!     // Sign the transaction
//!     let mut tx = tx_builder.sign().await?;
//!
//!     // Send the transaction
//!     let result = tx.send_tx().await?;
//!     println!("Transaction sent: {}", result.hash);
//!
//!     // Wait for the transaction to be confirmed
//!     println!("Waiting for confirmation...");
//!     tx.track_tx(10).await?;
//!     println!("Transaction confirmed!");
//!
//!     // Get the application log
//!     let app_log = tx.get_application_log(&client).await?;
//!     println!("Application log: {:?}", app_log);
//!
//!     Ok(())
//! }
//! ```
//!
//! ### Interacting with a smart contract
//!
//! ```no_run
//! use atipicial::atipicial_clients::{HttpProvider, RpcClient, APITrait};
//! use atipicial::atipicial_types::ScriptHash;
//! use std::str::FromStr;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     // Connect to Atipicial TestNet
//!     let provider = HttpProvider::new("https://testnet1.atipicial.com:443")?;
//!     let client = RpcClient::new(provider);
//!     
//!     // Get the ATC token contract
//!     let atipicial_token = ScriptHash::from_str("ef4073a0f2b305a38ec4050e4d3d28bc40ea63f5")?;
//!     
//!     // Call a read-only method (doesn't require signing)
//!     let result = client
//!         .invoke_function(&atipicial_token, "symbol".to_string(), vec![], None)
//!         .await?;
//!     
//!     // Parse the result
//!     if let Some(item) = result.stack.first() {
//!         println!("ATC Token Symbol: {:?}", item);
//!     }
//!     
//!     // Get the total supply
//!     let supply_result = client
//!         .invoke_function(&atipicial_token, "totalSupply".to_string(), vec![], None)
//!         .await?;
//!     
//!     println!("Total Supply Result: {:?}", supply_result);
//!     
//!     Ok(())
//! }
//! ```
//!
//! ### Working with AEP-17 tokens
//!
//! ```no_run
//! use atipicial::atipicial_clients::{HttpProvider, RpcClient, APITrait};
//! use atipicial::atipicial_protocol::{Account, AccountTrait};
//! use atipicial::atipicial_types::{ScriptHash, ContractParameter};
//! use std::str::FromStr;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     // Connect to Atipicial TestNet
//!     let provider = HttpProvider::new("https://testnet1.atipicial.com:443")?;
//!     let client = RpcClient::new(provider);
//!     
//!     // Create an account from WIF (Wallet Import Format)
//!     let wif = std::env::var("ATC_WIF")?;
//!     let account = Account::from_wif(&wif)?;
//!     
//!     // Get account information
//!     println!("Account address: {}", account.get_address());
//!     println!("Account script hash: {}", account.get_script_hash());
//!     
//!     // Get GAS token balance for the account
//!     let gas_token = ScriptHash::from_str("d2a4cff31913016155e38e474a2c06d08be276cf")?;
//!     
//!     let balance_result = client
//!         .invoke_function(
//!             &gas_token,
//!             "balanceOf".to_string(),
//!             vec![ContractParameter::h160(&account.get_script_hash())],
//!             None,
//!         )
//!         .await?;
//!     
//!     // Parse the balance result
//!     if let Some(item) = balance_result.stack.first() {
//!         println!("GAS Balance: {:?}", item);
//!     }
//!     
//!     Ok(())
//! }
//! ```
//!
//! ### Using the Atipicial Name Service (NNS)
//!
//! ```no_run
//! use atipicial::atipicial_clients::{HttpProvider, RpcClient, APITrait};
//! use atipicial::atipicial_types::{ContractParameter, ScriptHash};
//! use std::str::FromStr;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     // Connect to Atipicial TestNet
//!     let provider = HttpProvider::new("https://testnet1.atipicial.com:443")?;
//!     let client = RpcClient::new(provider);
//!     
//!     // NNS contract on TestNet (use MainNet hash for production)
//!     let nns_contract = ScriptHash::from_str("50ac1c37690cc2cfc594472833cf57505d5f46de")?;
//!     
//!     // Check if a name is available
//!     let available_result = client
//!         .invoke_function(
//!             &nns_contract,
//!             "isAvailable".to_string(),
//!             vec![ContractParameter::string("myname.atipicial".to_string())],
//!             None,
//!         )
//!         .await?;
//!     
//!     if let Some(item) = available_result.stack.first() {
//!         println!("Is 'myname.atipicial' available: {:?}", item);
//!     }
//!     
//!     Ok(())
//! }
//! ```
//!
//! For more usage examples, refer to the [`examples` directory](https://github.com/R3E-Network/AtipicialRust/tree/master/examples) in the repository.
//!
//! ## Project Structure
//!
//! ```text
//! AtipicialRust
//! ├── examples
//! │   ├── atipicial_nodes          - Examples for connecting to Atipicial nodes
//! │   ├── atipicial_transactions   - Examples for creating and sending transactions
//! │   ├── atipicial_smart_contracts - Examples for interacting with smart contracts
//! │   ├── atipicial_wallets        - Examples for wallet management
//! │   ├── atipicial_aep17_tokens   - Examples for working with AEP-17 tokens
//! │   └── atipicial_nns            - Examples for using the Atipicial Name Service
//! └── src
//!     ├── atipicial_builder        - Transaction and script building utilities
//!     ├── atipicial_clients        - Atipicial node interaction clients (RPC and WebSocket)
//!     ├── atipicial_codec          - Encoding and decoding for Atipicial-specific data structures
//!     ├── atipicial_config         - Network and client configuration management
//!     ├── atipicial_contract       - Smart contract interaction abstractions
//!     ├── atipicial_crypto         - Atipicial-specific cryptographic operations
//!     ├── atipicial_protocol       - Atipicial network protocol implementation
//!     ├── atipicial_sgx            - Experimental SGX enclave scaffolding
//!     ├── atipicial_types          - Core Atipicial ecosystem data types
//!     └── atipicial_wallets        - Atipicial asset and account management
//! ```
//!
//! ## Module Overview
//!
//! - **atipicial_builder**: Transaction and script building utilities.
//!   - Transaction construction and signing
//!   - Script building for contract calls
//!   - Network fee calculation
//!
//! - **atipicial_clients**: Atipicial node interaction clients.
//!   - HTTP, WebSocket, and IPC providers
//!   - JSON-RPC client implementation
//!   - Event subscription and notification handling
//!
//! - **atipicial_codec**: Encoding and decoding for Atipicial-specific data structures.
//!   - Binary serialization and deserialization
//!   - Atipicial VM script encoding
//!
//! - **atipicial_config**: Network and client configuration management.
//!   - Network magic numbers
//!   - Client settings
//!
//! - **atipicial_contract**: Smart contract interaction abstractions.
//!   - Contract invocation and deployment
//!   - AEP-17 token standard implementation
//!   - Native contracts (GAS, ATC, etc.)
//!   - Atipicial Name Service (NNS) support
//!
//! - **atipicial_crypto**: Atipicial-specific cryptographic operations.
//!   - Key generation and management
//!   - Signing and verification
//!   - Hashing functions
//!
//! - **atipicial_protocol**: Atipicial network protocol implementation.
//!   - Account management
//!   - Address formats and conversions
//!
//! - **atipicial_types**: Core Atipicial ecosystem data types.
//!   - Script hashes
//!   - Contract parameters
//!   - Block and transaction types
//!   - NNS name types
//!
//! - **atipicial_wallets**: Atipicial asset and account management.
//!   - Wallet creation and management
//!   - AEP-6 wallet standard support
//!   - Account import/export
//!   - Wallet backup and recovery
//!
//! For detailed information, consult the documentation of each module.

// Production-ready Atipicial SDK - warnings are treated as errors in CI
#![cfg_attr(all(feature = "sgx", target_env = "sgx"), no_std)]
#![cfg_attr(all(feature = "sgx", target_env = "sgx"), feature(rustc_private))]
#![allow(elided_lifetimes_in_paths, missing_docs, missing_debug_implementations)]
#![warn(unreachable_pub)]
#![doc(test(no_crate_inject, attr(deny(rust_2018_idioms), allow(dead_code, unused_variables))))]

// SGX support (only when building for the SGX target)
#[cfg(all(feature = "sgx", target_env = "sgx"))]
extern crate sgx_tstd as std;

// Required for no_std when targeting SGX
#[cfg(all(feature = "sgx", target_env = "sgx"))]
extern crate alloc;

// For macro expansions only, not public API.
#[doc(hidden)]
#[allow(unused_extern_crates)]
extern crate self as atipicial;

/// Current version of the `atipicial` crate.
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

// Core modules - always available
pub mod constants;
pub mod atipicial_error;
pub mod atipicial_types;
pub mod atipicial_utils;

// All modules unconditionally available
pub mod atipicial_builder;
pub mod atipicial_clients;
pub mod atipicial_codec;
pub mod atipicial_config;
pub mod atipicial_contract;
pub mod atipicial_crypto;
pub mod atipicial_fs;
pub mod atipicial_protocol;
#[cfg(all(feature = "sgx", target_env = "sgx"))]
pub mod atipicial_sgx;
pub mod atipicial_wallets;
pub mod atipicial_x;

// Monitoring infrastructure
pub mod monitoring;

// High-level SDK API (new in v1.0.x)
pub mod sdk;

// Re-exports for convenience
#[doc(inline)]
pub use atipicial_builder as builder;
#[doc(inline)]
pub use atipicial_clients as providers;
#[doc(inline)]
pub use atipicial_codec as codec;
#[doc(inline)]
pub use atipicial_config as config;
#[doc(inline)]
pub use atipicial_crypto as crypto;
#[doc(inline)]
pub use atipicial_protocol as protocol;
#[doc(inline)]
pub use atipicial_wallets as wallets;
#[doc(inline)]
pub use atipicial_x as x;
// No need to re-export specialized modules as they're already public with their full names

// Re-export common types directly in lib.rs for easy access
pub use crate::atipicial_types::{
	deserialize_address_or_script_hash,
	deserialize_h256,
	deserialize_h256_option,
	deserialize_hash_map_h160_account,
	deserialize_script_hash,
	deserialize_script_hash_option,
	deserialize_url_option,
	serialize_address_or_script_hash,
	serialize_h256,
	serialize_h256_option,
	serialize_hash_map_h160_account,
	// Serialization/deserialization helpers
	serialize_script_hash,
	serialize_script_hash_option,
	serialize_url_option,
	var_size,
	vec_to_array32,
	Address,
	AddressOrScriptHash,
	// Additional types
	Base64Encode,
	Bytes,
	ContractIdentifiers,
	// Contract types
	ContractManifest,
	ContractParameter,
	ContractParameterType,
	ContractState,
	InvocationResult,
	// NNS types
	NNSName,
	AefFile,
	OpCode,
	OperandSize,
	ParameterValue,
	ScriptHash,
	ScriptHashExtension,
	// Additional types
	ScryptParamsDef,
	StackItem,
	StringExt,
	TryBase64Encode,
	TryStringExt,
	TypeError,
	VMState,
};

// Add direct re-exports for commonly used serde utils
pub use crate::atipicial_types::serde_with_utils::{
	deserialize_boolean_expression, deserialize_bytes, deserialize_h160, deserialize_hardforks,
	deserialize_hashmap_address_u256, deserialize_hashmap_u256_hashset_h256,
	deserialize_hashmap_u256_hashset_u256, deserialize_hashmap_u256_vec_u256,
	deserialize_hashset_u256, deserialize_map, deserialize_private_key, deserialize_public_key,
	deserialize_public_key_option, deserialize_scopes, deserialize_vec_script_hash,
	deserialize_vec_script_hash_option, deserialize_wildcard, serialize_boolean_expression,
	serialize_bytes, serialize_h160, serialize_hashmap_address_u256,
	serialize_hashmap_u256_hashset_h256, serialize_hashmap_u256_hashset_u256,
	serialize_hashmap_u256_vec_u256, serialize_hashset_u256, serialize_map, serialize_private_key,
	serialize_public_key, serialize_public_key_option, serialize_scopes, serialize_vec_script_hash,
	serialize_vec_script_hash_option, serialize_wildcard,
};

// Re-export additional contract types
pub use crate::atipicial_types::contract::{
	ContractMethodToken, ContractAef, NativeContractState, AtipicialVMStateType,
};

// Re-export value extension trait
pub use crate::atipicial_types::serde_value::ValueExtension;

/// Convenient imports for commonly used types and traits.
///
/// This prelude module provides a single import to access the most commonly used
/// components of the AtipicialRust SDK. Import it with:
///
/// ```rust
/// use atipicial::prelude::*;
/// ```
pub mod prelude;

// Re-export optional external dependencies at the crate root so downstream
// crates can reference them without declaring a direct dependency. These MUST
// stay above `mod tests` — placing items after the test module triggers
// clippy::items_after_test_module.
// Explicitly mark external dependencies with cfg_attr for docs.rs
#[cfg(feature = "futures")]
pub use futures;

#[cfg(feature = "ledger")]
pub use coins_ledger;

#[cfg(test)]
mod tests {
	use super::prelude::*;
	use primitive_types::H160;
	use std::{str::FromStr, sync::Arc};
	use tokio::sync::Mutex;

	use crate::{
		builder::{AccountSigner, ScriptBuilder, TransactionBuilder},
		atipicial_clients::{MockClient, MockProvider, RpcClient},
		atipicial_config::TestConstants,
		atipicial_protocol::{Account, AccountTrait},
	};

	#[cfg(test)]
	#[tokio::test]
	async fn test_create_and_send_transaction() -> Result<(), Box<dyn std::error::Error>> {
		let (rpc_client, _mock_server) = setup_test_client().await?;

		// Create accounts for the sender and recipient
		let sender = Account::create()?;
		let recipient = Account::create()?;

		let gas_token_hash = TestConstants::GAS_TOKEN_HASH;

		// Create a new TransactionBuilder
		let mut tx_builder = TransactionBuilder::with_client(&rpc_client);

		// Build the transaction
		tx_builder
			.set_script(Some(
				ScriptBuilder::new()
					.contract_call(
						&H160::from_str(gas_token_hash)?,
						"transfer",
						&[
							ContractParameter::h160(&sender.get_script_hash()),
							ContractParameter::h160(&recipient.get_script_hash()),
							ContractParameter::integer(1_0000_0000), // 1 GAS (8 decimals)
							ContractParameter::any(),
						],
						None,
					)
					.map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?
					.to_bytes(),
			))
			.set_signers(vec![AccountSigner::called_by_entry(&sender)?.into()])
			.map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?
			.valid_until_block(100)?; // Small mock block window

		// Sign the transaction
		let signed_tx = tx_builder.sign().await?;

		assert_eq!(signed_tx.sys_fee, 30);
		assert_eq!(signed_tx.net_fee, 1_230_610);

		Ok(())
	}

	async fn setup_test_client(
	) -> Result<(RpcClient<MockProvider>, Option<Arc<Mutex<MockClient>>>), Box<dyn std::error::Error>>
	{
		let mock_server = Arc::new(Mutex::new(MockClient::new().await));
		{
			let mut mock = mock_server.lock().await;
			mock.mock_response_with_file_ignore_param(
				"invokescript",
				"invokescript_necessary_mock.json",
			)
			.await;
			mock.mock_response_with_file_ignore_param(
				"calculatenetworkfee",
				"calculatenetworkfee.json",
			)
			.await;
			mock.mock_response_ignore_param("getversion", serde_json::json!({})).await;
			mock.mock_response_with_file_ignore_param(
				"sendrawtransaction",
				"sendrawtransaction.json",
			)
			.await;
			mock.mount_mocks().await;
		}

		let rpc_client = {
			let mock = mock_server.lock().await;
			mock.into_client()
		};

		Ok((rpc_client, Some(mock_server)))
	}
}
