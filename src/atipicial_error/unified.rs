#![deny(missing_docs)]
//! Unified error handling system for improved developer experience
//!
//! This module provides a hierarchical error system with context,
//! recovery suggestions, and better error messages.

use std::fmt;
use thiserror::Error;

/// Unified error type for the entire Atipicial SDK
///
/// Provides consistent error handling with context and recovery suggestions.
#[derive(Debug, Error)]
#[non_exhaustive]
pub enum AtipicialError {
	/// Network / RPC transport failure.
	#[error("Network error: {message}")]
	Network {
		/// Human-readable description of the failure (sans source error).
		message: String,
		/// Underlying transport error, if any (`reqwest`, JSON-RPC, etc.).
		#[source]
		source: Option<Box<dyn std::error::Error + Send + Sync>>,
		/// Recovery hints (retryable flag, suggested actions, doc links).
		recovery: ErrorRecovery,
	},

	/// Wallet / account / key-material problem.
	#[error("Wallet error: {message}")]
	Wallet {
		/// Human-readable description.
		message: String,
		/// Underlying wallet/crypto error, if any.
		#[source]
		source: Option<Box<dyn std::error::Error + Send + Sync>>,
		/// Recovery hints.
		recovery: ErrorRecovery,
	},

	/// Smart-contract invocation problem (missing method, VM fault, …).
	#[error("Contract error: {message}")]
	Contract {
		/// Human-readable description.
		message: String,
		/// Contract hash (hex) the failure was associated with, if known.
		contract: Option<String>,
		/// Contract method name the failure was associated with, if known.
		method: Option<String>,
		/// Underlying error, if any.
		#[source]
		source: Option<Box<dyn std::error::Error + Send + Sync>>,
		/// Recovery hints.
		recovery: ErrorRecovery,
	},

	/// Transaction construction, signing, or submission failure.
	#[error("Transaction failed: {message}")]
	Transaction {
		/// Human-readable description.
		message: String,
		/// Transaction hash (hex) involved in the failure, if known.
		tx_hash: Option<String>,
		/// Underlying error, if any.
		#[source]
		source: Option<Box<dyn std::error::Error + Send + Sync>>,
		/// Recovery hints.
		recovery: ErrorRecovery,
	},

	/// Bad SDK/client configuration.
	#[error("Configuration error: {message}")]
	Configuration {
		/// Human-readable description.
		message: String,
		/// Name of the offending configuration field, if applicable.
		field: Option<String>,
		/// Recovery hints.
		recovery: ErrorRecovery,
	},

	/// User-supplied input validation failure.
	#[error("Validation error: {message}")]
	Validation {
		/// Human-readable description.
		message: String,
		/// Name of the offending input field.
		field: String,
		/// Offending value (when safe to echo back).
		value: Option<String>,
		/// Recovery hints.
		recovery: ErrorRecovery,
	},

	/// Account balance is too low for the requested operation.
	#[error("Insufficient funds: need {required} but have {available}")]
	InsufficientFunds {
		/// Required amount, formatted for display.
		required: String,
		/// Available amount, formatted for display.
		available: String,
		/// Token symbol or contract identifier.
		token: String,
		/// Recovery hints.
		recovery: ErrorRecovery,
	},

	/// Operation exceeded its deadline.
	#[error("Operation timed out after {duration:?}")]
	Timeout {
		/// How long the operation was waited on before giving up.
		duration: std::time::Duration,
		/// Name of the operation that timed out.
		operation: String,
		/// Recovery hints.
		recovery: ErrorRecovery,
	},

	/// Provider applied a rate limit.
	#[error("Rate limit exceeded: {message}")]
	RateLimit {
		/// Human-readable description.
		message: String,
		/// Suggested wait duration before retrying, if the provider returned one.
		retry_after: Option<std::time::Duration>,
		/// Recovery hints.
		recovery: ErrorRecovery,
	},

	/// Uncategorised / future-compatible bucket.
	#[error("{message}")]
	Other {
		/// Human-readable description.
		message: String,
		/// Underlying error, if any.
		#[source]
		source: Option<Box<dyn std::error::Error + Send + Sync>>,
		/// Recovery hints.
		recovery: ErrorRecovery,
	},
}

/// Error recovery suggestions
#[derive(Debug, Clone, Default)]
pub struct ErrorRecovery {
	/// Suggested actions to recover from the error
	pub suggestions: Vec<String>,
	/// Whether the operation can be retried
	pub retryable: bool,
	/// Recommended retry delay if retryable
	pub retry_after: Option<std::time::Duration>,
	/// Links to documentation
	pub docs: Vec<String>,
}

impl ErrorRecovery {
	/// Create a new recovery suggestion
	pub fn new() -> Self {
		Self::default()
	}

	/// Add a suggestion
	#[must_use]
	pub fn suggest(mut self, suggestion: impl Into<String>) -> Self {
		self.suggestions.push(suggestion.into());
		self
	}

	/// Mark as retryable
	#[must_use]
	pub fn retryable(mut self, retryable: bool) -> Self {
		self.retryable = retryable;
		self
	}

	/// Set retry delay
	#[must_use]
	pub fn retry_after(mut self, duration: std::time::Duration) -> Self {
		self.retry_after = Some(duration);
		self.retryable = true;
		self
	}

	/// Add documentation link
	#[must_use]
	pub fn doc(mut self, link: impl Into<String>) -> Self {
		self.docs.push(link.into());
		self
	}
}

impl fmt::Display for ErrorRecovery {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		if !self.suggestions.is_empty() {
			writeln!(f, "\n💡 Suggestions:")?;
			for suggestion in &self.suggestions {
				writeln!(f, "   • {}", suggestion)?;
			}
		}

		if self.retryable {
			write!(f, "\n🔄 This operation can be retried")?;
			if let Some(duration) = self.retry_after {
				write!(f, " after {:?}", duration)?;
			}
			writeln!(f)?;
		}

		if !self.docs.is_empty() {
			writeln!(f, "\n📚 See documentation:")?;
			for doc in &self.docs {
				writeln!(f, "   • {}", doc)?;
			}
		}

		Ok(())
	}
}

/// Result type alias for Atipicial operations
pub type Result<T> = std::result::Result<T, AtipicialError>;

/// AWS-SDK-style metadata accessor for SDK errors.
///
/// This trait mirrors `aws_smithy_types::error::metadata::ProvideErrorMetadata`
/// from the AWS Rust SDK, giving callers a stable, vendor-neutral way to
/// extract a short error `code()` and human-readable `message()` for logging,
/// telemetry, and structured error reporting — without having to match on
/// every `#[non_exhaustive]` variant.
///
/// # Example
///
/// ```
/// use atipicial::atipicial_error::unified::{AtipicialError, ProvideErrorMetadata};
///
/// let err = AtipicialError::network("connect", "boom");
/// assert_eq!(ProvideErrorMetadata::code(&err), Some("Network"));
/// assert!(ProvideErrorMetadata::message(&err).is_some());
/// ```
pub trait ProvideErrorMetadata {
	/// Returns a short, stable identifier for this error (e.g. `"Network"`,
	/// `"RateLimit"`). Returns `None` only if no classification is available
	/// — every variant of [`AtipicialError`] produces a `Some`.
	fn code(&self) -> Option<&str>;

	/// Returns the human-readable error message (without source chain or
	/// recovery hints). Returns `None` only if no message is available.
	fn message(&self) -> Option<&str>;
}

impl ProvideErrorMetadata for AtipicialError {
	fn code(&self) -> Option<&str> {
		Some(match self.kind() {
			AtipicialErrorKind::Network => "Network",
			AtipicialErrorKind::Wallet => "Wallet",
			AtipicialErrorKind::Contract => "Contract",
			AtipicialErrorKind::Transaction => "Transaction",
			AtipicialErrorKind::Configuration => "Configuration",
			AtipicialErrorKind::Validation => "Validation",
			AtipicialErrorKind::InsufficientFunds => "InsufficientFunds",
			AtipicialErrorKind::Timeout => "Timeout",
			AtipicialErrorKind::RateLimit => "RateLimit",
			AtipicialErrorKind::Other => "Other",
		})
	}

	fn message(&self) -> Option<&str> {
		Some(AtipicialError::message(self))
	}
}

/// Coarse classification of [`AtipicialError`] variants, modeled after the
/// AWS SDK's `ProvideErrorMetadata` / `ErrorKind` accessors.
///
/// Use this to route errors into telemetry buckets, structured logs, or
/// retry decisions without depending on the exact enum variant layout
/// (which is `#[non_exhaustive]`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum AtipicialErrorKind {
	/// Transport / RPC / connectivity failure.
	Network,
	/// Wallet, key material, or account-state problem.
	Wallet,
	/// Smart-contract invocation problem (VM fault, missing method, …).
	Contract,
	/// Transaction construction, signing, or submission problem.
	Transaction,
	/// Bad client/SDK configuration.
	Configuration,
	/// User-input validation failure.
	Validation,
	/// Account balance is too low for the requested operation.
	InsufficientFunds,
	/// Operation exceeded its deadline.
	Timeout,
	/// Provider applied a rate limit.
	RateLimit,
	/// Uncategorised / future-compatible bucket.
	Other,
}

impl AtipicialError {
	/// Convert a provider error while preserving whether it is transient and which SDK domain it
	/// belongs to.
	pub fn provider(context: &str, err: crate::atipicial_clients::ProviderError) -> Self {
		use crate::atipicial_clients::ProviderError;

		let message = format!("{}: {}", context, err);
		if err.is_rate_limited() {
			let retry_after = err.retry_after();
			let mut recovery = ErrorRecovery::new()
				.suggest("Retry after backing off")
				.suggest("Reduce request concurrency")
				.retryable(true);
			if let Some(delay) = retry_after {
				recovery = recovery.retry_after(delay);
			}
			return AtipicialError::RateLimit { message, retry_after, recovery };
		}

		let retryable = err.is_retryable();
		match err {
			ProviderError::InvalidAddress => AtipicialError::Validation {
				message,
				field: "address".to_string(),
				value: None,
				recovery: ErrorRecovery::new().suggest("Provide a valid Atipicial address"),
			},
			ProviderError::NnsError(_) | ProviderError::NnsNotOwned(_) => AtipicialError::Validation {
				message,
				field: "name".to_string(),
				value: None,
				recovery: ErrorRecovery::new().suggest("Check the NNS name and ownership"),
			},
			ProviderError::TypeError(_) => AtipicialError::Validation {
				message,
				field: "provider_input".to_string(),
				value: None,
				recovery: ErrorRecovery::new().suggest("Check the provider request values"),
			},
			ProviderError::InvalidPassword
			| ProviderError::SignerUnavailable
			| ProviderError::CryptoError(_) => AtipicialError::Wallet {
				message,
				source: None,
				recovery: ErrorRecovery::new().suggest("Check wallet credentials and signer setup"),
			},
			ProviderError::UnsupportedRPC
			| ProviderError::UnsupportedNodeClient
			| ProviderError::ProtocolNotFound
			| ProviderError::NetworkNotFound => AtipicialError::Configuration {
				message,
				field: Some("provider".to_string()),
				recovery: ErrorRecovery::new()
					.suggest("Use an endpoint that supports the requested RPC operation"),
			},
			error @ (ProviderError::HTTPError(_) | ProviderError::JsonRpcError(_)) => {
				AtipicialError::Network {
					message,
					source: Some(Box::new(error)),
					recovery: ErrorRecovery::new()
						.suggest("Check network connectivity and the RPC endpoint")
						.retryable(retryable),
				}
			},
			error => AtipicialError::Other {
				message,
				source: Some(Box::new(error)),
				recovery: ErrorRecovery::new(),
			},
		}
	}

	/// Returns the coarse [`AtipicialErrorKind`] for this error.
	///
	/// Stable across non-exhaustive variant additions — prefer this in
	/// `match` arms over destructuring the enum directly.
	pub fn kind(&self) -> AtipicialErrorKind {
		match self {
			AtipicialError::Network { .. } => AtipicialErrorKind::Network,
			AtipicialError::Wallet { .. } => AtipicialErrorKind::Wallet,
			AtipicialError::Contract { .. } => AtipicialErrorKind::Contract,
			AtipicialError::Transaction { .. } => AtipicialErrorKind::Transaction,
			AtipicialError::Configuration { .. } => AtipicialErrorKind::Configuration,
			AtipicialError::Validation { .. } => AtipicialErrorKind::Validation,
			AtipicialError::InsufficientFunds { .. } => AtipicialErrorKind::InsufficientFunds,
			AtipicialError::Timeout { .. } => AtipicialErrorKind::Timeout,
			AtipicialError::RateLimit { .. } => AtipicialErrorKind::RateLimit,
			AtipicialError::Other { .. } => AtipicialErrorKind::Other,
		}
	}

	/// Returns the recovery hints attached to the error, if any.
	pub fn recovery(&self) -> &ErrorRecovery {
		match self {
			AtipicialError::Network { recovery, .. }
			| AtipicialError::Wallet { recovery, .. }
			| AtipicialError::Contract { recovery, .. }
			| AtipicialError::Transaction { recovery, .. }
			| AtipicialError::Configuration { recovery, .. }
			| AtipicialError::Validation { recovery, .. }
			| AtipicialError::InsufficientFunds { recovery, .. }
			| AtipicialError::Timeout { recovery, .. }
			| AtipicialError::RateLimit { recovery, .. }
			| AtipicialError::Other { recovery, .. } => recovery,
		}
	}

	/// Whether the underlying operation can sensibly be retried.
	///
	/// Mirrors `aws-smithy-runtime-api::client::retries::classifiers::RetryAction`
	/// — returns `true` for transient failures (timeouts, rate limits, network
	/// blips, RPC 5xx), `false` for deterministic failures (validation, signing,
	/// insufficient funds).
	pub fn is_retryable(&self) -> bool {
		matches!(self, AtipicialError::RateLimit { .. } | AtipicialError::Timeout { .. })
			|| self.recovery().retryable
	}

	/// Suggested delay before retry, if the error carries one.
	pub fn retry_after(&self) -> Option<std::time::Duration> {
		match self {
			AtipicialError::RateLimit { retry_after, .. } => *retry_after,
			_ => self.recovery().retry_after,
		}
	}

	/// Returns the human-readable message portion of the error (no source / recovery).
	pub fn message(&self) -> &str {
		match self {
			AtipicialError::Network { message, .. }
			| AtipicialError::Wallet { message, .. }
			| AtipicialError::Contract { message, .. }
			| AtipicialError::Transaction { message, .. }
			| AtipicialError::Configuration { message, .. }
			| AtipicialError::Validation { message, .. }
			| AtipicialError::Timeout { operation: message, .. }
			| AtipicialError::RateLimit { message, .. }
			| AtipicialError::Other { message, .. } => message,
			AtipicialError::InsufficientFunds { token, .. } => token,
		}
	}
}

/// Convert legacy `AtipicialError` values into the unified error type.
///
/// This is a best-effort mapping intended to ease migration for callers that
/// still receive `AtipicialError` from lower-level APIs.
impl From<super::LegacyError> for AtipicialError {
	fn from(err: super::LegacyError) -> Self {
		use super::{LegacyError, NetworkError, TransactionError};

		match err {
			LegacyError::Network(network_error) => match network_error {
				NetworkError::RateLimitExceeded => super::unified::AtipicialError::RateLimit {
					message: network_error.to_string(),
					retry_after: None,
					recovery: ErrorRecovery::new()
						.suggest("Retry after a short delay")
						.retryable(true),
				},
				_ => super::unified::AtipicialError::Network {
					message: network_error.to_string(),
					source: None,
					recovery: ErrorRecovery::new().retryable(true),
				},
			},
			LegacyError::Wallet(wallet_error) => super::unified::AtipicialError::Wallet {
				message: wallet_error.to_string(),
				source: None,
				recovery: ErrorRecovery::new(),
			},
			LegacyError::Transaction(tx_error) => match tx_error {
				TransactionError::InsufficientFunds { required, available } => {
					AtipicialError::InsufficientFunds {
						required: required.to_string(),
						available: available.to_string(),
						token: "unknown".to_string(),
						recovery: ErrorRecovery::new()
							.suggest("Check the account balance")
							.retryable(false),
					}
				},
				_ => super::unified::AtipicialError::Transaction {
					message: tx_error.to_string(),
					tx_hash: None,
					source: None,
					recovery: ErrorRecovery::new(),
				},
			},
			LegacyError::Contract(contract_error) => super::unified::AtipicialError::Contract {
				message: contract_error.to_string(),
				contract: None,
				method: None,
				source: None,
				recovery: ErrorRecovery::new(),
			},
			LegacyError::Config(message) => {
				AtipicialError::Configuration { message, field: None, recovery: ErrorRecovery::new() }
			},
			LegacyError::Crypto(crypto_error) => super::unified::AtipicialError::Other {
				message: crypto_error.to_string(),
				source: None,
				recovery: ErrorRecovery::new(),
			},
			LegacyError::Serialization(serialization_error) => super::unified::AtipicialError::Other {
				message: serialization_error.to_string(),
				source: None,
				recovery: ErrorRecovery::new(),
			},
			LegacyError::Generic { message } => {
				AtipicialError::Other { message, source: None, recovery: ErrorRecovery::new() }
			},
			LegacyError::UnsupportedOperation(message) => {
				AtipicialError::Other { message, source: None, recovery: ErrorRecovery::new() }
			},
		}
	}
}

/// Convenience constructors used by the high-level `sdk` module to keep call
/// sites concise. Each helper attaches the most common recovery hints for the
/// situation so callers don't repeat the same boilerplate.
impl AtipicialError {
	/// Construct a [`AtipicialError::Network`] from any `Display`-able source error.
	///
	/// Marks the failure retryable and includes generic connectivity hints —
	/// matches the AWS SDK convention where transport failures are retryable
	/// by default.
	pub fn network<E: fmt::Display>(context: &str, err: E) -> Self {
		AtipicialError::Network {
			message: format!("{}: {}", context, err),
			source: None,
			recovery: ErrorRecovery::new()
				.suggest("Check network connectivity")
				.suggest("Verify the RPC endpoint is accessible")
				.retryable(true),
		}
	}

	/// Construct a [`AtipicialError::Transaction`] from any `Display`-able source error.
	pub fn transaction<E: fmt::Display>(context: &str, err: E) -> Self {
		AtipicialError::Transaction {
			message: format!("{}: {}", context, err),
			tx_hash: None,
			source: None,
			recovery: ErrorRecovery::new(),
		}
	}

	/// Construct a [`AtipicialError::Contract`] error tagged with the contract hash and method.
	pub fn contract<E: fmt::Display>(
		context: &str,
		contract: Option<String>,
		method: Option<String>,
		err: E,
	) -> Self {
		AtipicialError::Contract {
			message: format!("{}: {}", context, err),
			contract,
			method,
			source: None,
			recovery: ErrorRecovery::new(),
		}
	}

	/// Construct a [`AtipicialError::Validation`] error for an invalid input value.
	pub fn validation<V: Into<String>>(field: &str, value: Option<V>, message: &str) -> Self {
		AtipicialError::Validation {
			message: message.to_string(),
			field: field.to_string(),
			value: value.map(Into::into),
			recovery: ErrorRecovery::new().suggest("Check the input value"),
		}
	}

	/// Construct a [`AtipicialError::Wallet`] error from any `Display`-able source error.
	pub fn wallet<E: fmt::Display>(context: &str, err: E) -> Self {
		AtipicialError::Wallet {
			message: format!("{}: {}", context, err),
			source: None,
			recovery: ErrorRecovery::new(),
		}
	}
}

/// Builder for creating detailed errors with context
pub struct ErrorBuilder {
	kind: ErrorKind,
	message: String,
	source: Option<Box<dyn std::error::Error + Send + Sync>>,
	recovery: ErrorRecovery,
	context: ErrorContext,
}

#[derive(Debug)]
#[allow(dead_code)] // Variants used by ErrorBuilder; not all paths are wired up yet
enum ErrorKind {
	Network,
	Wallet,
	Contract,
	Transaction,
	Configuration,
	Validation,
	InsufficientFunds,
	Timeout,
	RateLimit,
	Other,
}

#[derive(Debug, Default)]
#[allow(dead_code)] // Fields reserved for ErrorBuilder context; not all are consumed yet
struct ErrorContext {
	contract: Option<String>,
	method: Option<String>,
	tx_hash: Option<String>,
	field: Option<String>,
	value: Option<String>,
	required: Option<String>,
	available: Option<String>,
	token: Option<String>,
	duration: Option<std::time::Duration>,
	operation: Option<String>,
}

impl ErrorBuilder {
	/// Create a network error
	pub fn network(message: impl Into<String>) -> Self {
		Self {
			kind: ErrorKind::Network,
			message: message.into(),
			source: None,
			recovery: ErrorRecovery::default(),
			context: ErrorContext::default(),
		}
	}

	/// Create a wallet error
	pub fn wallet(message: impl Into<String>) -> Self {
		Self {
			kind: ErrorKind::Wallet,
			message: message.into(),
			source: None,
			recovery: ErrorRecovery::default(),
			context: ErrorContext::default(),
		}
	}

	/// Create a contract error
	pub fn contract(message: impl Into<String>) -> Self {
		Self {
			kind: ErrorKind::Contract,
			message: message.into(),
			source: None,
			recovery: ErrorRecovery::default(),
			context: ErrorContext::default(),
		}
	}

	/// Add source error
	#[must_use]
	pub fn source(mut self, source: impl std::error::Error + Send + Sync + 'static) -> Self {
		self.source = Some(Box::new(source));
		self
	}

	/// Add contract context
	#[must_use]
	pub fn with_contract(mut self, contract: impl Into<String>) -> Self {
		self.context.contract = Some(contract.into());
		self
	}

	/// Add method context
	#[must_use]
	pub fn with_method(mut self, method: impl Into<String>) -> Self {
		self.context.method = Some(method.into());
		self
	}

	/// Add recovery suggestion
	#[must_use]
	pub fn suggest(mut self, suggestion: impl Into<String>) -> Self {
		self.recovery = self.recovery.suggest(suggestion);
		self
	}

	/// Mark as retryable
	#[must_use]
	pub fn retryable(mut self) -> Self {
		self.recovery = self.recovery.retryable(true);
		self
	}

	/// Build the error
	pub fn build(self) -> AtipicialError {
		match self.kind {
			ErrorKind::Network => AtipicialError::Network {
				message: self.message,
				source: self.source,
				recovery: self.recovery,
			},
			ErrorKind::Wallet => AtipicialError::Wallet {
				message: self.message,
				source: self.source,
				recovery: self.recovery,
			},
			ErrorKind::Contract => AtipicialError::Contract {
				message: self.message,
				contract: self.context.contract,
				method: self.context.method,
				source: self.source,
				recovery: self.recovery,
			},
			_ => AtipicialError::Other {
				message: self.message,
				source: self.source,
				recovery: self.recovery,
			},
		}
	}
}

/// Extension trait for adding context to errors
pub trait ErrorContextExt {
	/// Add context to this error
	fn context(self, message: impl Into<String>) -> AtipicialError;

	/// Add recovery suggestions
	fn recover(self, suggestion: impl Into<String>) -> AtipicialError;
}

impl<E> ErrorContextExt for E
where
	E: std::error::Error + Send + Sync + 'static,
{
	fn context(self, message: impl Into<String>) -> AtipicialError {
		AtipicialError::Other {
			message: message.into(),
			source: Some(Box::new(self)),
			recovery: ErrorRecovery::default(),
		}
	}

	fn recover(self, suggestion: impl Into<String>) -> AtipicialError {
		AtipicialError::Other {
			message: self.to_string(),
			source: Some(Box::new(self)),
			recovery: ErrorRecovery::new().suggest(suggestion),
		}
	}
}

// =============================================================================
// From implementations for legacy error types
// =============================================================================

impl From<crate::atipicial_crypto::CryptoError> for AtipicialError {
	fn from(err: crate::atipicial_crypto::CryptoError) -> Self {
		use crate::atipicial_crypto::CryptoError;
		match &err {
			CryptoError::InvalidPassphrase(msg) => AtipicialError::Wallet {
				message: format!("Invalid passphrase: {}", msg),
				source: None,
				recovery: ErrorRecovery::new()
					.suggest("Check that the password is correct")
					.suggest("Ensure the wallet file is not corrupted"),
			},
			CryptoError::InvalidPrivateKey | CryptoError::InvalidPublicKey => AtipicialError::Wallet {
				message: err.to_string(),
				source: None,
				recovery: ErrorRecovery::new()
					.suggest("Verify the key format is correct")
					.suggest("Ensure the key was not truncated"),
			},
			CryptoError::SigningError | CryptoError::SignatureVerificationError => {
				AtipicialError::Transaction {
					message: err.to_string(),
					tx_hash: None,
					source: None,
					recovery: ErrorRecovery::new().suggest("Verify the signing key is correct"),
				}
			},
			CryptoError::DecryptionError(msg) => AtipicialError::Wallet {
				message: format!("Decryption failed: {}", msg),
				source: None,
				recovery: ErrorRecovery::new()
					.suggest("Check that the password is correct")
					.suggest("Ensure the encrypted data is not corrupted"),
			},
			_ => AtipicialError::Other {
				message: err.to_string(),
				source: None,
				recovery: ErrorRecovery::new(),
			},
		}
	}
}

impl From<crate::atipicial_wallets::WalletError> for AtipicialError {
	fn from(err: crate::atipicial_wallets::WalletError) -> Self {
		use crate::atipicial_wallets::WalletError;
		match &err {
			WalletError::NoKeyPair => AtipicialError::Wallet {
				message: "No key pair available".to_string(),
				source: None,
				recovery: ErrorRecovery::new()
					.suggest("Import a private key or create a new account")
					.suggest("Decrypt the wallet if it is encrypted"),
			},
			WalletError::NoDefaultAccount => AtipicialError::Wallet {
				message: "No default account set".to_string(),
				source: None,
				recovery: ErrorRecovery::new()
					.suggest("Set a default account using set_default_account()")
					.suggest("Add an account to the wallet first"),
			},
			WalletError::NoAccounts => AtipicialError::Wallet {
				message: "Wallet has no accounts".to_string(),
				source: None,
				recovery: ErrorRecovery::new()
					.suggest("Create a new account")
					.suggest("Import an existing account"),
			},
			WalletError::DecryptionError(msg) => AtipicialError::Wallet {
				message: format!("Decryption failed: {}", msg),
				source: None,
				recovery: ErrorRecovery::new()
					.suggest("Verify the password is correct")
					.suggest("Check if the wallet file is corrupted"),
			},
			WalletError::AccountState(msg) => AtipicialError::Wallet {
				message: format!("Account state error: {}", msg),
				source: None,
				recovery: ErrorRecovery::new(),
			},
			WalletError::FileError(msg) => AtipicialError::Wallet {
				message: format!("File operation failed: {}", msg),
				source: None,
				recovery: ErrorRecovery::new()
					.suggest("Check file permissions")
					.suggest("Verify the file path is correct"),
			},
			_ => AtipicialError::Wallet {
				message: err.to_string(),
				source: None,
				recovery: ErrorRecovery::new(),
			},
		}
	}
}

impl From<crate::atipicial_builder::BuilderError> for AtipicialError {
	fn from(err: crate::atipicial_builder::BuilderError) -> Self {
		use crate::atipicial_builder::BuilderError;
		match &err {
			BuilderError::SignerConfiguration(msg) => AtipicialError::Transaction {
				message: format!("Signer configuration error: {}", msg),
				tx_hash: None,
				source: None,
				recovery: ErrorRecovery::new()
					.suggest("Verify signer accounts are valid")
					.suggest("Check signer scopes are appropriate"),
			},
			BuilderError::TransactionConfiguration(msg) => AtipicialError::Transaction {
				message: format!("Transaction configuration error: {}", msg),
				tx_hash: None,
				source: None,
				recovery: ErrorRecovery::new()
					.suggest("Review transaction parameters")
					.suggest("Ensure all required fields are set"),
			},
			BuilderError::TooManySigners(msg) => AtipicialError::Transaction {
				message: format!("Too many signers: {}", msg),
				tx_hash: None,
				source: None,
				recovery: ErrorRecovery::new()
					.suggest("Reduce the number of signers")
					.doc("https://docs.atipicial.com/docs/n3/foundation/Transactions"),
			},
			BuilderError::InvalidScript(msg) => AtipicialError::Transaction {
				message: format!("Invalid script: {}", msg),
				tx_hash: None,
				source: None,
				recovery: ErrorRecovery::new()
					.suggest("Verify the script is correctly formatted")
					.suggest("Check for invalid opcodes"),
			},
			_ => AtipicialError::Transaction {
				message: err.to_string(),
				tx_hash: None,
				source: None,
				recovery: ErrorRecovery::new(),
			},
		}
	}
}

impl From<crate::atipicial_builder::TransactionError> for AtipicialError {
	fn from(err: crate::atipicial_builder::TransactionError) -> Self {
		use crate::atipicial_builder::TransactionError;
		match &err {
			TransactionError::NoSigners => AtipicialError::Transaction {
				message: "Transaction has no signers".to_string(),
				tx_hash: None,
				source: None,
				recovery: ErrorRecovery::new()
					.suggest("Add at least one signer to the transaction")
					.suggest("Use set_signers() or add_signer()"),
			},
			TransactionError::NoScript | TransactionError::EmptyScript => AtipicialError::Transaction {
				message: "Transaction has no script".to_string(),
				tx_hash: None,
				source: None,
				recovery: ErrorRecovery::new()
					.suggest("Set the transaction script using set_script()")
					.suggest("Build a script using ScriptBuilder"),
			},
			TransactionError::InsufficientFunds => AtipicialError::InsufficientFunds {
				required: "unknown".to_string(),
				available: "unknown".to_string(),
				token: "GAS".to_string(),
				recovery: ErrorRecovery::new()
					.suggest("Check account balance")
					.suggest("Acquire more GAS tokens")
					.retryable(false),
			},
			TransactionError::TxTooLarge => AtipicialError::Transaction {
				message: "Transaction exceeds maximum size".to_string(),
				tx_hash: None,
				source: None,
				recovery: ErrorRecovery::new()
					.suggest("Split the transaction into smaller parts")
					.suggest("Reduce the script size"),
			},
			_ => AtipicialError::Transaction {
				message: err.to_string(),
				tx_hash: None,
				source: None,
				recovery: ErrorRecovery::new(),
			},
		}
	}
}

impl From<crate::atipicial_contract::ContractError> for AtipicialError {
	fn from(err: crate::atipicial_contract::ContractError) -> Self {
		use crate::atipicial_contract::ContractError;
		match &err {
			ContractError::InvalidAtipicialName(name) => AtipicialError::Contract {
				message: format!("Invalid NNS name: {}", name),
				contract: Some("NameService".to_string()),
				method: None,
				source: None,
				recovery: ErrorRecovery::new()
					.suggest("Check the domain name format")
					.suggest("Ensure the name follows NNS naming rules"),
			},
			ContractError::UnresolvableDomainName(name) => AtipicialError::Contract {
				message: format!("Cannot resolve domain: {}", name),
				contract: Some("NameService".to_string()),
				method: Some("resolve".to_string()),
				source: None,
				recovery: ErrorRecovery::new()
					.suggest("Verify the domain is registered")
					.suggest("Check if the domain has expired"),
			},
			ContractError::InvocationFailed(msg) => AtipicialError::Contract {
				message: format!("Contract invocation failed: {}", msg),
				contract: None,
				method: None,
				source: None,
				recovery: ErrorRecovery::new()
					.suggest("Check contract parameters")
					.suggest("Verify the contract is deployed")
					.retryable(true),
			},
			ContractError::ProviderNotSet(msg) => AtipicialError::Configuration {
				message: format!("Provider not configured: {}", msg),
				field: Some("provider".to_string()),
				recovery: ErrorRecovery::new()
					.suggest("Set an RPC provider before calling contract methods")
					.suggest("Use with_provider() to configure the client"),
			},
			_ => AtipicialError::Contract {
				message: err.to_string(),
				contract: None,
				method: None,
				source: None,
				recovery: ErrorRecovery::new(),
			},
		}
	}
}

impl From<crate::atipicial_types::TypeError> for AtipicialError {
	fn from(err: crate::atipicial_types::TypeError) -> Self {
		use crate::atipicial_types::TypeError;
		match &err {
			TypeError::InvalidAddress => AtipicialError::Validation {
				message: "Invalid Atipicial address".to_string(),
				field: "address".to_string(),
				value: None,
				recovery: ErrorRecovery::new()
					.suggest("Check the address format (should start with 'N')")
					.suggest("Verify the address checksum"),
			},
			TypeError::InvalidPrivateKey | TypeError::InvalidPublicKey => AtipicialError::Validation {
				message: err.to_string(),
				field: "key".to_string(),
				value: None,
				recovery: ErrorRecovery::new()
					.suggest("Verify the key format")
					.suggest("Check key length (32 bytes for private, 33/65 for public)"),
			},
			TypeError::InvalidFormat(msg) => AtipicialError::Validation {
				message: format!("Invalid format: {}", msg),
				field: "data".to_string(),
				value: None,
				recovery: ErrorRecovery::new().suggest("Check the data format"),
			},
			TypeError::NumericOverflow => AtipicialError::Validation {
				message: "Numeric overflow".to_string(),
				field: "number".to_string(),
				value: None,
				recovery: ErrorRecovery::new().suggest("Use a smaller value"),
			},
			_ => AtipicialError::Other {
				message: err.to_string(),
				source: None,
				recovery: ErrorRecovery::new(),
			},
		}
	}
}

impl From<crate::codec::CodecError> for AtipicialError {
	fn from(err: crate::codec::CodecError) -> Self {
		match err {
			crate::codec::CodecError::InvalidPassphrase(message) => AtipicialError::Wallet {
				message: format!("Invalid passphrase: {message}"),
				source: None,
				recovery: ErrorRecovery::new().suggest("Verify the wallet passphrase"),
			},
			other => AtipicialError::Validation {
				message: other.to_string(),
				field: "encoded_data".to_string(),
				value: None,
				recovery: ErrorRecovery::new().suggest("Verify the encoded data format"),
			},
		}
	}
}

impl From<crate::atipicial_clients::ProviderError> for AtipicialError {
	fn from(err: crate::atipicial_clients::ProviderError) -> Self {
		AtipicialError::provider("provider request failed", err)
	}
}

impl From<std::io::Error> for AtipicialError {
	fn from(err: std::io::Error) -> Self {
		AtipicialError::Other {
			message: format!("IO error: {}", err),
			source: Some(Box::new(err)),
			recovery: ErrorRecovery::new()
				.suggest("Check file permissions")
				.suggest("Verify the path exists"),
		}
	}
}

impl From<hex::FromHexError> for AtipicialError {
	fn from(err: hex::FromHexError) -> Self {
		AtipicialError::Validation {
			message: format!("Invalid hex string: {}", err),
			field: "hex".to_string(),
			value: None,
			recovery: ErrorRecovery::new()
				.suggest("Ensure the string contains only hex characters (0-9, a-f)")
				.suggest("Check for correct string length"),
		}
	}
}

impl From<serde_json::Error> for AtipicialError {
	fn from(err: serde_json::Error) -> Self {
		AtipicialError::Other {
			message: format!("JSON error: {}", err),
			source: Some(Box::new(err)),
			recovery: ErrorRecovery::new().suggest("Check JSON format and structure"),
		}
	}
}

impl From<crate::atipicial_fs::AtipicialFsError> for AtipicialError {
	fn from(err: crate::atipicial_fs::AtipicialFsError) -> Self {
		use crate::atipicial_fs::AtipicialFsError;
		match &err {
			AtipicialFsError::ConnectionError(msg) => AtipicialError::Network {
				message: format!("AtipicialFs connection error: {}", msg),
				source: None,
				recovery: ErrorRecovery::new()
					.suggest("Check AtipicialFs endpoint connectivity")
					.retryable(true),
			},
			AtipicialFsError::AuthenticationError(msg) => AtipicialError::Wallet {
				message: format!("AtipicialFs authentication error: {}", msg),
				source: None,
				recovery: ErrorRecovery::new()
					.suggest("Verify AtipicialFs credentials")
					.suggest("Check that the session token is valid"),
			},
			AtipicialFsError::PermissionDenied(msg) => AtipicialError::Contract {
				message: format!("AtipicialFs permission denied: {}", msg),
				contract: Some("AtipicialFs".to_string()),
				method: None,
				source: None,
				recovery: ErrorRecovery::new()
					.suggest("Check container ACL settings")
					.suggest("Verify the bearer token has sufficient permissions"),
			},
			AtipicialFsError::NotFound(msg) => AtipicialError::Other {
				message: format!("AtipicialFs resource not found: {}", msg),
				source: None,
				recovery: ErrorRecovery::new()
					.suggest("Verify the container or object ID is correct"),
			},
			AtipicialFsError::Timeout(msg) => AtipicialError::Timeout {
				duration: std::time::Duration::from_secs(0),
				operation: format!("AtipicialFs: {}", msg),
				recovery: ErrorRecovery::new()
					.suggest("Increase timeout or retry the operation")
					.retryable(true),
			},
			_ => AtipicialError::Other {
				message: err.to_string(),
				source: None,
				recovery: ErrorRecovery::new(),
			},
		}
	}
}

impl From<crate::atipicial_protocol::ProtocolError> for AtipicialError {
	fn from(err: crate::atipicial_protocol::ProtocolError) -> Self {
		use crate::atipicial_protocol::ProtocolError;
		match &err {
			ProtocolError::RpcResponse { error } => AtipicialError::Network {
				message: format!("RPC response error: {}", error),
				source: None,
				recovery: ErrorRecovery::new()
					.suggest("Check the RPC request parameters")
					.retryable(true),
			},
			ProtocolError::InvocationFaultState { error } => AtipicialError::Contract {
				message: format!("VM fault: {}", error),
				contract: None,
				method: None,
				source: None,
				recovery: ErrorRecovery::new()
					.suggest("Check contract parameters and state")
					.suggest("Verify the contract is deployed on the target network"),
			},
			ProtocolError::ClientConnection { message } => AtipicialError::Network {
				message: format!("Client connection error: {}", message),
				source: None,
				recovery: ErrorRecovery::new()
					.suggest("Check network connectivity")
					.retryable(true),
			},
			ProtocolError::IllegalState { message } => AtipicialError::Other {
				message: format!("Illegal state: {}", message),
				source: None,
				recovery: ErrorRecovery::new(),
			},
			_ => AtipicialError::Other {
				message: err.to_string(),
				source: None,
				recovery: ErrorRecovery::new(),
			},
		}
	}
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_error_builder() {
		let error = ErrorBuilder::network("Connection failed")
			.suggest("Check your internet connection")
			.suggest("Try a different RPC endpoint")
			.retryable()
			.build();

		match error {
			AtipicialError::Network { recovery, .. } => {
				assert_eq!(recovery.suggestions.len(), 2);
				assert!(recovery.retryable);
			},
			_ => panic!("Wrong error type"),
		}
	}

	#[test]
	fn test_error_display() {
		let error = AtipicialError::InsufficientFunds {
			required: "100 GAS".to_string(),
			available: "50 GAS".to_string(),
			token: "GAS".to_string(),
			recovery: ErrorRecovery::new()
				.suggest("Acquire more GAS tokens")
				.suggest("Reduce the transaction amount")
				.doc("https://docs.atipicial.com/tokens/gas"),
		};

		let display = format!("{}", error);
		assert!(display.contains("Insufficient funds"));
		assert!(display.contains("need 100 GAS but have 50 GAS"));
	}

	#[test]
	fn kind_returns_stable_variant_classification() {
		let err = AtipicialError::network("connect", "boom");
		assert_eq!(err.kind(), AtipicialErrorKind::Network);
		assert!(err.is_retryable());

		let err = AtipicialError::validation("amount", Some("-1"), "must be positive");
		assert_eq!(err.kind(), AtipicialErrorKind::Validation);
		assert!(!err.is_retryable());
	}

	#[test]
	fn provider_conversion_preserves_retry_and_domain_classification() {
		let invalid_address = AtipicialError::from(crate::atipicial_clients::ProviderError::InvalidAddress);
		assert_eq!(invalid_address.kind(), AtipicialErrorKind::Validation);
		assert!(!invalid_address.is_retryable());

		let rate_limited = AtipicialError::from(crate::atipicial_clients::ProviderError::JsonRpcError(
			crate::atipicial_clients::JsonRpcError {
				code: 429,
				message: "too many requests".to_string(),
				data: None,
			},
		));
		assert_eq!(rate_limited.kind(), AtipicialErrorKind::RateLimit);
		assert!(rate_limited.is_retryable());

		let rate_limited = AtipicialError::from(crate::atipicial_clients::ProviderError::JsonRpcError(
			crate::atipicial_clients::JsonRpcError {
				code: 429,
				message: "too many requests".to_string(),
				data: Some(serde_json::json!({ "rate": { "backoff_seconds": 2 } })),
			},
		));
		assert_eq!(rate_limited.retry_after(), Some(std::time::Duration::from_secs(2)));
	}

	#[test]
	fn provider_debug_redacts_json_rpc_data_through_unified_error() {
		let error = AtipicialError::from(crate::atipicial_clients::ProviderError::JsonRpcError(
			crate::atipicial_clients::JsonRpcError {
				code: -32000,
				message: "provider rejected request".to_string(),
				data: Some(serde_json::json!({ "token": "super-secret" })),
			},
		));

		let debug = format!("{error:?}");
		assert!(debug.contains("provider rejected request"));
		assert!(!debug.contains("super-secret"));
	}

	#[test]
	fn codec_errors_convert_into_the_unified_boundary() {
		let malformed = AtipicialError::from(crate::codec::CodecError::InvalidEncoding(
			"truncated payload".to_string(),
		));
		assert_eq!(malformed.kind(), AtipicialErrorKind::Validation);
		assert!(!malformed.is_retryable());

		let passphrase = AtipicialError::from(crate::codec::CodecError::InvalidPassphrase(
			"decryption failed".to_string(),
		));
		assert_eq!(passphrase.kind(), AtipicialErrorKind::Wallet);
	}

	#[test]
	fn retry_after_round_trips_for_rate_limit() {
		let err = AtipicialError::RateLimit {
			message: "throttled".to_string(),
			retry_after: Some(std::time::Duration::from_secs(2)),
			recovery: ErrorRecovery::new(),
		};
		assert!(err.is_retryable());
		assert_eq!(err.retry_after(), Some(std::time::Duration::from_secs(2)));
	}

	#[test]
	fn convenience_constructors_attach_recovery_hints() {
		let err = AtipicialError::contract(
			"invoke failed",
			Some("abc".into()),
			Some("transfer".into()),
			"vm fault",
		);
		match err {
			AtipicialError::Contract { contract, method, .. } => {
				assert_eq!(contract.as_deref(), Some("abc"));
				assert_eq!(method.as_deref(), Some("transfer"));
			},
			other => panic!("expected Contract variant, got {other:?}"),
		}
	}

	#[test]
	fn provide_error_metadata_exposes_code_and_message() {
		// Cover every kind end-to-end via the trait
		let cases: &[(&str, AtipicialError)] = &[
			("Network", AtipicialError::network("connect", "boom")),
			("Wallet", AtipicialError::wallet("decrypt", "bad key")),
			("Contract", AtipicialError::contract("invoke", None, None, "fault")),
			("Transaction", AtipicialError::transaction("sign", "no key")),
			(
				"Configuration",
				AtipicialError::Configuration {
					message: "missing endpoint".into(),
					field: None,
					recovery: ErrorRecovery::new(),
				},
			),
			("Validation", AtipicialError::validation("amount", Some("-1"), "must be positive")),
			(
				"InsufficientFunds",
				AtipicialError::InsufficientFunds {
					required: "1".into(),
					available: "0".into(),
					token: "GAS".into(),
					recovery: ErrorRecovery::new(),
				},
			),
			(
				"Timeout",
				AtipicialError::Timeout {
					duration: std::time::Duration::from_secs(1),
					operation: "wait".into(),
					recovery: ErrorRecovery::new(),
				},
			),
			(
				"RateLimit",
				AtipicialError::RateLimit {
					message: "throttled".into(),
					retry_after: None,
					recovery: ErrorRecovery::new(),
				},
			),
			(
				"Other",
				AtipicialError::Other {
					message: "misc".into(),
					source: None,
					recovery: ErrorRecovery::new(),
				},
			),
		];

		for (expected_code, err) in cases {
			assert_eq!(
				ProvideErrorMetadata::code(err),
				Some(*expected_code),
				"code mismatch for {expected_code}"
			);
			assert!(
				ProvideErrorMetadata::message(err).is_some(),
				"missing message for {expected_code}"
			);
		}
	}
}
