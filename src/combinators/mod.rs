//! Combinators for composing and building complex encodables.
//!
//! [`Encodable`]: crate::Encodable
//!
//! This module provides utility types for building complex [`Encodable`]
//! types from simpler ones. These combinators allow common operations for
//! encoding values, such as conditional encoding, prefixes or custom byte
//! ordering.
//!
//! These building blocks follow the [`Encodable`] trait interface and can be
//! nested or chained together to define rich encoding behavior with minimal
//! boilerplate.
//!
//! # Categories of Encodables
//!
//! ## Primitive and Built-in Encodables
//!
//! These are the basic types that can be directly encoded.
//!
//! | Type | Description |
//! |------|-------------|
//! | [`(...)`](tuple) | Encodes a tuple by encoding each element in order |
//! | [`i8`] and [`u8`] | Encodes a single byte |
//! | [`char`] | Encodes a character as its UTF-8 byte representation |
//! | [`&str`](str) | Encodes a UTF-8 string |
//! | [`&CStr`](core::ffi::CStr) | Encodes a C string, including the null terminator (`\0`) |
//! | [`&[u8]`](slice) | Encodes a byte slice |
//! | [`Arguments`](core::fmt::Arguments) | Encodes formatted data from [`format_args!]` with zero allocations |
//!
//! ## Composition and Flow Combinators
//!
//! These types wrap or transform other encodables, allowing conditional
//! encoding, iteration, or control over the format and structure of the output.
//!
//! | Type | Description |
//! |------|-------------|
//! | [`Option`] | Encodes the inner value if `Some`; does nothing if `None` |
//! | [`Result`] | Encodes the value on [`Ok`]; returns the error on [`Err`] |
//! | [`Cond`] | Encodes a value only if a condition is met |
//! | [`Flags`] | Encodes a set of bit flags packed into a single byte |
//! | [`LE`] | Encodes a number in little-endian order |
//! | [`BE`] | Encodes a number in big-endian order |
//! | [`LengthPrefix`] | Encodes a length prefixed value ([TLV](https://en.wikipedia.org/wiki/Type–length–value)) |
//! | [`Separated`] | Encodes a sequence of encodables separated by a given delimiter |
//! | [`Iter`] | Encodes a sequence of encodables |
//! | [`FromError`] | Transforms the error type of an encodable. |
#![cfg_attr(
    feature = "alloc",
    doc = r"## Alloc Encodables (requires the `alloc` feature)

These types are supported when the `alloc` or `std` feature is enabled.

| Type | Description |
|------|-------------|
| [`Vec<u8>`] | Encodes a byte vector as a contiguous sequence of bytes |
| [`String`] | Encodes a heap-allocated UTF-8 string |
| [`CString`](std::ffi::CString) | Encodes a C-style string including the null terminator (`\0`) |
| [`Box<T>`] | Encodes the value pointed to by a `Box`, as if it were directly encoded |
"
)]

mod be;
mod cond;
mod flags;
mod from_error;
mod iter;
mod le;
mod length_prefix;
mod separated;

pub use be::BE;
pub use cond::Cond;
pub use flags::Flags;
pub use from_error::FromError;
pub use iter::Iter;
pub use le::LE;
pub use length_prefix::LengthPrefix;
pub use separated::Separated;
