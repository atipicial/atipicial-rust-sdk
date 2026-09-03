#![allow(warnings)]

//! # atipicial_builder::script
//!
//! This module provides utilities for building and reading Atipicial smart contract scripts.
//!
//! ## Modules
//!
//! ### `interop_service`
//!
//! Contains the [`InteropService`] enum, which represents various system calls available in the Atipicial virtual machine.
//!
//! ### `script_builder`
//!
//! Provides the [`ScriptBuilder`] struct for constructing Atipicial smart contract scripts programmatically.
//!
//! ### `script_reader`
//!
//! Offers the [`ScriptReader`] struct for parsing and interpreting Atipicial smart contract scripts.
//!
//! ## Usage
//!
//! To use the functionality provided by this module, you can import the necessary components:
//!
//! ```rust
//! use atipicial::atipicial_builder::{InteropService, ScriptBuilder, ScriptReader};
//! ```
//!
//! [`InteropService`]: interop_service::InteropService
//! [`ScriptBuilder`]: script_builder::ScriptBuilder
//! [`ScriptReader`]: script_reader::ScriptReader

pub use interop_service::*;
pub use script_builder::*;
pub use script_reader::*;

mod interop_service;
mod script_builder;
mod script_reader;
