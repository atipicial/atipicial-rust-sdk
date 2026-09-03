#![deny(missing_docs)]
//! Error types for the Atipicial SDK.
//!
//! Two error families live here:
//!
//! - **[`unified::AtipicialError`]** (recommended) — the modern, structured error
//!   used by the high-level [`crate::sdk`] module. Carries source-error
//!   chaining, optional contract/method/tx context, retryability hints, and
//!   AWS-style [`unified::AtipicialErrorKind`] classification.
//! - **[`LegacyError`]** (legacy) — an older flat enum still returned by some
//!   low-level helpers. It implements `Into<unified::AtipicialError>` so callers can convert
//!   transparently with the `?` operator.
//!
//! New code should prefer `unified::AtipicialError`. The legacy type is kept to
//! avoid breaking downstream APIs and will be revisited in a future major
//! release.

use thiserror::Error;

// Include the unified error module for improved developer experience
pub mod unified;

// Legacy error hierarchy retained for backward compatibility. New code should
// use `unified::AtipicialError`; documenting every variant here is deliberately
// skipped — each `#[error(...)]` message is its public contract.

/// Legacy comprehensive error type for the Atipicial SDK.
///
/// New code should prefer [`unified::AtipicialError`], which carries retryability
/// metadata, recovery hints, and matches the AWS-SDK `ProvideErrorMetadata`
/// pattern. This enum is kept so existing call sites that match on
/// `LegacyError::Crypto(_)`, etc. continue to work; the [`unified::AtipicialError`]
/// type has a `From<LegacyError>` impl for seamless interop.
#[allow(missing_docs)]
#[derive(Error, Debug)]
#[non_exhaustive]
pub enum LegacyError {
	/// Cryptographic operation errors
	#[error("Cryptographic error: {0}")]
	Crypto(#[from] CryptoError),

	/// Wallet operation errors
	#[error("Wallet error: {0}")]
	Wallet(#[from] WalletError),

	/// Network/RPC communication errors
	#[error("Network error: {0}")]
	Network(#[from] NetworkError),

	/// Transaction building/validation errors
	#[error("Transaction error: {0}")]
	Transaction(#[from] TransactionError),

	/// Smart contract interaction errors
	#[error("Contract error: {0}")]
	Contract(#[from] ContractError),

	/// Serialization/deserialization errors
	#[error("Serialization error: {0}")]
	Serialization(#[from] SerializationError),

	/// Configuration errors
	#[error("Configuration error: {0}")]
	Config(String),

	/// Generic errors with context
	#[error("Error: {message}")]
	Generic { message: String },

	/// Unsupported operation error
	#[error("Unsupported operation: {0}")]
	UnsupportedOperation(String),
}

/// Legacy crypto error subset of [`AtipicialError`]. See [`unified::AtipicialError`] for the modern API.
#[allow(missing_docs)]
#[derive(Error, Debug)]
pub enum CryptoError {
	#[error("Invalid private key: {0}")]
	InvalidPrivateKey(String),

	#[error("Invalid public key: {0}")]
	InvalidPublicKey(String),

	#[error("Signature verification failed")]
	SignatureVerificationFailed,

	#[error("Key generation failed: {0}")]
	KeyGenerationFailed(String),

	#[error("Hash operation failed: {0}")]
	HashFailed(String),

	#[error("Encryption failed: {0}")]
	EncryptionFailed(String),

	#[error("Decryption failed: {0}")]
	DecryptionFailed(String),
}

/// Legacy wallet error subset of [`AtipicialError`]. See [`unified::AtipicialError`] for the modern API.
#[allow(missing_docs)]
#[derive(Error, Debug)]
pub enum WalletError {
	#[error("Wallet not found: {0}")]
	NotFound(String),

	#[error("Invalid password")]
	InvalidPassword,

	#[error("Account not found: {0}")]
	AccountNotFound(String),

	#[error("Wallet is locked")]
	WalletLocked,

	#[error("Backup operation failed: {0}")]
	BackupFailed(String),

	#[error("Recovery operation failed: {0}")]
	RecoveryFailed(String),

	#[error("YubiHSM error: {0}")]
	YubiHsmError(String),

	#[error("Invalid wallet format: {0}")]
	InvalidFormat(String),

	#[error("IO error: {0}")]
	Io(#[from] std::io::Error),
}

/// Legacy network error subset of [`AtipicialError`]. See [`unified::AtipicialError`] for the modern API.
#[allow(missing_docs)]
#[derive(Error, Debug)]
#[non_exhaustive]
pub enum NetworkError {
	#[error("Connection failed: {0}")]
	ConnectionFailed(String),

	#[error("Request timeout")]
	Timeout,

	#[error("Invalid response: {0}")]
	InvalidResponse(String),

	#[error("RPC error: {code} - {message}")]
	RpcError { code: i32, message: String },

	#[error("Network unreachable: {0}")]
	NetworkUnreachable(String),

	#[error("Rate limit exceeded")]
	RateLimitExceeded,

	#[error("HTTP error: {0}")]
	Http(#[from] reqwest::Error),
}

/// Legacy transaction error subset of [`AtipicialError`]. See [`unified::AtipicialError`] for the modern API.
#[allow(missing_docs)]
#[derive(Error, Debug)]
pub enum TransactionError {
	#[error("Invalid transaction: {0}")]
	Invalid(String),

	#[error("Insufficient funds: required {required}, available {available}")]
	InsufficientFunds { required: u64, available: u64 },

	#[error("Transaction too large: {size} bytes (max: {max})")]
	TooLarge { size: usize, max: usize },

	#[error("Invalid signature")]
	InvalidSignature,

	#[error("Transaction expired")]
	Expired,

	#[error("Nonce too low: {provided} (expected: {expected})")]
	NonceTooLow { provided: u64, expected: u64 },

	#[error("Gas limit exceeded: {used} (limit: {limit})")]
	GasLimitExceeded { used: u64, limit: u64 },
}

/// Legacy contract error subset of [`AtipicialError`]. See [`unified::AtipicialError`] for the modern API.
#[allow(missing_docs)]
#[derive(Error, Debug)]
#[non_exhaustive]
pub enum ContractError {
	#[error("Contract not found: {0}")]
	NotFound(String),

	#[error("Method not found: {0}")]
	MethodNotFound(String),

	#[error("Invalid parameters: {0}")]
	InvalidParameters(String),

	#[error("Execution failed: {0}")]
	ExecutionFailed(String),

	#[error("Insufficient gas: {0}")]
	InsufficientGas(String),

	#[error("Contract deployment failed: {0}")]
	DeploymentFailed(String),
}

/// Legacy serialization error subset of [`AtipicialError`]. See [`unified::AtipicialError`] for the modern API.
#[allow(missing_docs)]
#[derive(Error, Debug)]
pub enum SerializationError {
	#[error("JSON error: {0}")]
	Json(#[from] serde_json::Error),

	#[error("Invalid format: {0}")]
	InvalidFormat(String),

	#[error("Encoding error: {0}")]
	Encoding(String),

	#[error("Decoding error: {0}")]
	Decoding(String),
}

/// Result type alias for Atipicial operations
pub type AtipicialResult<T> = Result<T, LegacyError>;

// Additional From implementations for common error types
impl From<std::io::Error> for LegacyError {
	fn from(err: std::io::Error) -> Self {
		LegacyError::Wallet(WalletError::Io(err))
	}
}

impl From<serde_json::Error> for LegacyError {
	fn from(err: serde_json::Error) -> Self {
		LegacyError::Serialization(SerializationError::Json(err))
	}
}

/// Trait for adding context to errors
pub trait ErrorContext<T> {
	/// Wraps the existing error with an additional context message.
	///
	/// The closure is invoked lazily and only on the error path so the
	/// happy-path cost is a single function pointer.
	fn with_context<F>(self, f: F) -> AtipicialResult<T>
	where
		F: FnOnce() -> String;
}

impl<T, E> ErrorContext<T> for Result<T, E>
where
	E: Into<LegacyError>,
{
	fn with_context<F>(self, f: F) -> AtipicialResult<T>
	where
		F: FnOnce() -> String,
	{
		self.map_err(|e| {
			let base_error = e.into();
			LegacyError::Generic { message: format!("{}: {}", f(), base_error) }
		})
	}
}

/// Macro for creating context-aware errors
#[macro_export]
macro_rules! atipicial_error {
    ($msg:expr) => {
        LegacyError::Generic {
            message: $msg.to_string(),
        }
    };
    ($fmt:expr, $($arg:tt)*) => {
        LegacyError::Generic {
            message: format!($fmt, $($arg)*),
        }
    };
}

/// Macro for early return with context
#[macro_export]
macro_rules! ensure {
    ($cond:expr, $msg:expr) => {
        if !$cond {
            return Err(atipicial_error!($msg));
        }
    };
    ($cond:expr, $fmt:expr, $($arg:tt)*) => {
        if !$cond {
            return Err(atipicial_error!($fmt, $($arg)*));
        }
    };
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_error_context() {
		let result: Result<(), std::io::Error> =
			Err(std::io::Error::new(std::io::ErrorKind::NotFound, "file not found"));

		let with_context = result.with_context(|| "Failed to read configuration file".to_string());

		assert!(with_context.is_err());
		let error_msg = with_context.unwrap_err().to_string();
		assert!(error_msg.contains("Failed to read configuration file"));
		assert!(error_msg.contains("file not found"));
	}

	#[test]
	fn test_error_macros() {
		let error = atipicial_error!("Something went wrong");
		assert_eq!(error.to_string(), "Error: Something went wrong");

		let error = atipicial_error!("Value {} is invalid", 42);
		assert_eq!(error.to_string(), "Error: Value 42 is invalid");
	}
}


/// Legacy enum re-exports for crates written against the old flat error type.
/// New code should use [`unified::AtipicialError`].
pub use legacy_error::{AtipicialError as LegacyAtipicialError, AtipicialResult as LegacyAtipicialResult};

/// The legacy flat error type, exposed under its historical name path
/// (`atipicial_error::AtipicialError`) for crates not yet migrated to the
/// unified error. Kept in [`legacy_error`] to avoid the name collision with
/// [`unified::AtipicialError`].
pub mod legacy_error {
    //! Re-exports of the pre-rename flat error family under their
    //! historical names, for crates not yet migrated to the unified error.
    pub use super::{
        ContractError, CryptoError, NetworkError, SerializationError, TransactionError,
        WalletError,
    };
    pub use super::AtipicialResult;
    pub use super::LegacyError as AtipicialError;
}
