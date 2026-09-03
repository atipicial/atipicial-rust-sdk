//! # AtipicialRust prelude
//!
//! A curated re-export of the most commonly used types, traits, and helpers
//! from across the crate. Pull it in once and skip the import boilerplate:
//!
//! ```rust
//! use atipicial::prelude::*;
//! ```
//!
//! ## What you get
//!
//! | Category | Highlights |
//! |----------|------------|
//! | **High-level SDK** | [`Atipicial`], [`AtipicialBuilder`], [`SdkConfig`], [`Token`], [`Balance`], [`Network`] |
//! | Primitives | [`Address`], [`ScriptHash`], [`Bytes`], `H160`, `H256`, `U256` |
//! | Contracts | [`ContractManifest`], [`ContractParameter`], [`InvocationResult`], [`AefFile`] |
//! | VM | [`OpCode`], [`StackItem`], [`VMState`] |
//! | NNS | [`NNSName`] |
//! | Errors | [`AtipicialError`] — unified error with `kind()` / `is_retryable()` / recovery hints |
//! | Module aliases | `builder`, `providers`, `codec`, `config`, `crypto`, `protocol`, `wallets`, `x` |
//! | Encoding helpers | [`Base64Encode`], [`StringExt`], `ToBase58`, `ToHexString`, … |
//!
//! ## Quick start via the prelude
//!
//! The high-level entry point is available straight from the prelude, so a
//! working client is a one-liner:
//!
//! ```no_run
//! use atipicial::prelude::*;
//!
//! # #[tokio::main]
//! # async fn main() -> Result<(), AtipicialError> {
//! let atipicial = Atipicial::testnet().await?;
//! let height = atipicial.get_block_height().await?;
//! println!("tip = {height}");
//! # Ok(())
//! # }
//! ```
//!
//! ## When to import the prelude vs explicit paths
//!
//! - **Use the prelude** when writing application code that touches many parts
//!   of the SDK (transactions, contracts, wallets, RPC).
//! - **Use explicit imports** in library code, or when you only need one or
//!   two types — it keeps namespaces clean and helps incremental compilation.
//!
//! [`AtipicialError`]: crate::atipicial_error::unified::AtipicialError
//! [`Atipicial`]: crate::sdk::Atipicial
//! [`AtipicialBuilder`]: crate::sdk::AtipicialBuilder
//! [`SdkConfig`]: crate::sdk::SdkConfig
//! [`Token`]: crate::sdk::Token
//! [`Balance`]: crate::sdk::Balance
//! [`Network`]: crate::sdk::Network
// Unified error type (modern). Carries kind()/is_retryable()/recovery hints,
// and has From impls for every domain error (ProviderError, BuilderError,
// ContractError, WalletError, CryptoError, AtipicialError, io, serde, ...), so a
// single `fn() -> Result<T, AtipicialError>` boundary composes with `?` across the
// whole SDK. This is the same type returned by the high-level `sdk::Atipicial`.
pub use crate::atipicial_error::unified::AtipicialError;

// SDK version
pub use crate::VERSION;

// === Core Types ===
// Basic blockchain types
pub use crate::atipicial_types::{
	Address, AddressOrScriptHash, Base64Encode, Bytes, NameOrAddress, ScriptHash,
	ScriptHashExtension, StringExt, ToBase58, TryBase64Encode, TryStringExt,
};

// Contract-related types
pub use crate::atipicial_types::{
	ContractManifest, ContractParameter, ContractParameterType, ContractState, InvocationResult,
	AefFile,
};

// VM and runtime types
pub use crate::atipicial_types::{OpCode, StackItem, VMState};

// NNS-related types
pub use crate::atipicial_types::NNSName;

// Common external types
pub use primitive_types::{H160, H256, U256};
pub use serde_json::Value as ParameterValue;
pub use url::Url;

// === Serialization Helpers ===
pub use crate::atipicial_types::{
	// H160/H256 serialization
	deserialize_h160,
	deserialize_h256,
	// Other serialization helpers
	deserialize_script_hash,
	// U256 serialization
	deserialize_u256,
	deserialize_u64,
	deserialize_vec_h256,
	deserialize_vec_u256,
	deserialize_wildcard,
	serialize_h160,
	serialize_h256,
	serialize_script_hash,
	serialize_u256,
	serialize_u64,

	serialize_vec_h256,

	serialize_vec_u256,
	serialize_wildcard,
};

// === Core Functionality Modules ===
// These are aliased module names for user convenience
pub use crate::{
	atipicial_builder as builder, atipicial_clients as providers, atipicial_codec as codec, atipicial_config as config,
	atipicial_crypto as crypto, atipicial_protocol as protocol, atipicial_wallets as wallets, atipicial_x as x,
};

// === Extension modules ===
// These are full modules that provide specialized functionality
pub use crate::atipicial_fs; // AtipicialFs distributed storage

// Re-export ValueExtension
pub use crate::atipicial_types::ValueExtension;

// === Utility Traits ===
// Hex and base64 encoding/decoding utilities
pub use crate::atipicial_crypto::utils::{FromBase64String, FromHexString, ToHexString};

// === High-level SDK API ===
// The opinionated, batteries-included entry point. `Atipicial::testnet()`,
// `Atipicial::from_env()`, `Atipicial::connect(url)` — see the crate-level docs for
// guidance on when to use this layer versus the lower-level `providers::RpcClient`.
pub use crate::sdk::{Balance, Atipicial, AtipicialBuilder, Network, SdkConfig, Token};
