//! # Atipicial Utilities
//!
//! Utility functions and types for the AtipicialRust SDK.
//!
//! ## Overview
//!
//! The atipicial_utils module provides various utility functions and types that are used throughout the SDK.
//! These utilities include:
//!
//! - Error types and handling utilities
//! - Common helper functions
//! - Conversion utilities
//! - Formatting utilities
//!
//! This module serves as a foundation for the more specialized modules in the SDK.
//!
//! ## Examples
//!
//! ```rust
//! use atipicial::atipicial_error::legacy_error::AtipicialError;
//!
//! // Error handling with specific error types
//! fn example() -> Result<(), AtipicialError> {
//!     // Create and return a specific error
//!     let some_condition = true;
//!     if some_condition {
//!         return Err(AtipicialError::Generic { message: "Invalid format".to_string() });
//!     }
//!     
//!     Ok(())
//! }
//! ```

pub mod error;
