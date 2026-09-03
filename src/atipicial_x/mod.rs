//! # Atipicial X
//!
//! Support for Atipicial X, an EVM-compatible chain maintained by Atipicial.
//!
//! ## Overview
//!
//! The atipicial_x module provides interfaces for interacting with Atipicial X, an EVM-compatible
//! chain maintained by Atipicial. It includes:
//!
//! - EVM compatibility layer for interacting with Atipicial X as an Ethereum-compatible chain
//! - Bridge functionality for transferring tokens between Atipicial and Atipicial X
//! - EVM wallet and client wrappers powered by Alloy
//! - Optional routing through a third-party Anti-MEV RPC endpoint; mitigation is
//!   service-dependent and does not guarantee prevention of front-running or sandwich attacks
//! - Transaction creation and signing for Atipicial X
//! - Provider interfaces for connecting to Atipicial X nodes
//!
//! This module enables seamless integration between Atipicial and EVM-compatible ecosystems,
//! allowing developers to leverage both blockchain environments natively.
//!
//! ## Examples
//!
//! ### Connecting to Atipicial X and getting chain information
//!
//! ```no_run
//! use atipicial::atipicial_clients::{HttpProvider, RpcClient};
//! use atipicial::atipicial_x::AtipicialXProvider;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     // Connect to Atipicial (optional, for cross-chain ops)
//!     let atipicial_provider = HttpProvider::new("https://mainnet1.atipicial.com:443")?;
//!     let atipicial_client = RpcClient::new(atipicial_provider);
//!
//!     // Initialize the Atipicial X EVM provider with the third-party protected RPC endpoint
//!     let atipicial_x_provider = AtipicialXProvider::new_anti_mev(Some(&atipicial_client));
//!
//!     // Get the chain ID for Atipicial X via Alloy or fall back to the N3 node
//!     let chain_id = atipicial_x_provider.chain_id().await?;
//!     println!("Atipicial X Chain ID: {}", chain_id);
//!
//!     Ok(())
//! }
//! ```
//!
//! ### Using AtipicialXWallet to check EVM balance
//!
//! ```no_run
//! use atipicial::atipicial_clients::{HttpProvider, RpcClient};
//! use atipicial::atipicial_x::{AtipicialXProvider, AtipicialXWallet, AtipicialXClient};
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let atipicial_x_provider: AtipicialXProvider<'_, HttpProvider> = AtipicialXProvider::new("https://rpc.atipicial-x.org", None);
//!     
//!     // Create a random wallet or load from private key
//!     let wallet = AtipicialXWallet::create_random();
//!     println!("Generated EVM Address: {:?}", wallet.address());
//!
//!     let client = AtipicialXClient::new(wallet, atipicial_x_provider);
//!     let balance = client.get_balance().await.unwrap();
//!     println!("Balance: {}", balance);
//!
//!     Ok(())
//! }
//! ```
//!
//! ### Using the bridge to transfer tokens between Atipicial and Atipicial X
//!
//! ```no_run
//! use atipicial::atipicial_clients::{HttpProvider, RpcClient};
//! use atipicial::atipicial_protocol::{Account, AccountTrait};
//! use atipicial::atipicial_types::ScriptHash;
//! use atipicial::atipicial_x::AtipicialXBridgeContract;
//! use std::str::FromStr;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     // Connect to Atipicial
//!     let atipicial_provider = HttpProvider::new("https://mainnet1.atipicial.com:443")?;
//!     let atipicial_client = RpcClient::new(atipicial_provider);
//!
//!     // Initialize the bridge contract
//!     let bridge = AtipicialXBridgeContract::new(Some(&atipicial_client))?;
//!
//!     // Get the GAS token script hash
//!     let gas_token = ScriptHash::from_str("d2a4cff31913016155e38e474a2c06d08be276cf")?;
//!
//!     // Query bridge fee for the token
//!     let fee = bridge.get_fee(&gas_token).await?;
//!     println!("Bridge fee: {}", fee);
//!
//!     Ok(())
//! }
//! ```

/// Bridge operations between Atipicial and Atipicial X
pub mod bridge;
/// EVM compatibility helpers for Atipicial X
pub mod evm;

pub use bridge::*;
pub use evm::*;
