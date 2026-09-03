//! # Atipicial Protocol
//!
//! Core protocol types and interfaces for the Atipicial blockchain.
//!
//! ## Overview
//!
//! The atipicial_protocol module provides the core types and interfaces for interacting with
//! the Atipicial blockchain protocol. It includes:
//!
//! - Account management and address handling
//! - AEP-2 password-protected key format support
//! - Protocol error definitions and handling
//! - Response types for Atipicial RPC calls
//! - Role-based access control definitions
//!
//! This module forms the foundation for blockchain interactions, defining the data structures
//! and interfaces that represent the Atipicial protocol.
//!
//! ## Examples
//!
//! ### Working with Atipicial accounts
//!
//! ```no_run
//! use atipicial::atipicial_protocol::{Account, AccountTrait};
//!
//! // Create a new account
//! let account = Account::create().unwrap();
//! println!("Address: {}", account.get_address());
//! println!("Script Hash: {}", account.get_script_hash());
//!
//! // Create an account from a WIF (Wallet Import Format) string
//! let wif = "your-private-key-wif";
//! let account = Account::from_wif(wif).unwrap();
//! ```
//!
//! ### Using AEP-2 password-protected keys
//!
//! ```no_run
//! use atipicial::atipicial_crypto::KeyPair;
//! use atipicial::atipicial_protocol::AEP2;
//!
//! // Create a key pair and encrypt with AEP-2
//! let key_pair = KeyPair::new_random();
//! let password = "your_secure_password_here";
//! let aep2_string = AEP2::encrypt(password, &key_pair).unwrap();
//!
//! // Decrypt a AEP-2 string back to a key pair
//! let decrypted = AEP2::decrypt(password, &aep2_string).unwrap();
//! ```

pub use account::*;
pub use aep2::*;
pub use protocol_error::*;
pub use responses::*;

mod account;
mod aep2;
mod protocol_error;
mod responses;
mod role;
