//! Binary encoding and decoding for the Atipicial wire format.
//!
//! This module implements the byte-level serialization rules the Atipicial
//! protocol uses on the wire and inside `AtipicialVM`. It is what
//! [`atipicial_builder`](crate::atipicial_builder) reaches for when assembling
//! transactions and what [`atipicial_clients`](crate::atipicial_clients) uses to parse
//! raw RPC payloads.
//!
//! ## Layers
//!
//! - [`Encoder`] / [`Decoder`] — low-level cursors over `&mut Vec<u8>` and
//!   `&[u8]` for primitives (varint, fixed-size integers, byte arrays, …).
//! - [`AtipicialSerializable`] — derived trait for types that round-trip through
//!   the wire format.
//! - [`CodecError`] — failures during decoding (truncated input, invalid
//!   discriminants, length-prefix overflow).
//!
//! ## Stability
//!
//! The encoding tracks the Atipicial reference implementation (C#); breaking
//! changes here would imply a protocol fork and are therefore extremely
//! rare. Higher-level abstractions in [`atipicial_builder`](crate::atipicial_builder)
//! should be preferred over hand-rolled codec usage.

pub use binary_decoder::*;
pub use binary_encoder::*;
pub use encode::*;
pub use error::*;

mod binary_decoder;
mod binary_encoder;
mod encode;
mod error;
