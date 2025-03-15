//! Combinators for composing encodables.
//!
//! This module contains a number of combinators that can be used to compose
//! encodables. The objective is to provide a set of building blocks that can be
//! combined to create more complex encodables that can be encoded into byte
//! sequences.
//!
//! # Available encodables
//!
//! ## Basic encodables
//!
//! | Type | Description |
//! |------------|-------------|
//! | [`(...)`](tuple) | Encodes all encodables in sequence |
//! | [i8], [u8] | Encodes a single byte |
//! | [`char`] | Encodes the character as a UTF-8 byte sequence |
//! | [`&str`](str) | Encodes a string as a UTF-8 byte sequence |
//! | [`&CStr`](core::ffi::CStr) | Encodes a string as a byte sequence with a null terminator (`\0`) |
//! | [`&[u8]`](slice) | Encodes a slice of bytes |
//! | [`Arguments`](core::fmt::Arguments)([`format_args!`])| Runs [`core::fmt`](`core::fmt`) machinery and encodes the result, without allocations |
//!
//! ## Encodable combinators
//!
//! | Type | Description |
//! |------------|-------------|
//! | [`Option`] | Encodes `T` if `Some`, or does nothing on `None` |
//! | [`Result`] | Encodes `T` if [`Ok`], or bubbles up `E` on [`Err`] |
//! | [`Cond`] | Conditionally encodes an encodable if the given predicate is true |
//! | [`Flags`] | Encodes a set of bit flags as a byte |
//! | [`LE`] | Encodes a number in little-endian order. |
//! | [`BE`] | Encodes a number in big-endian order. |
//! | [`Separated`] | Encodes a sequence of encodables separated by a delimiter. |
//! | [`LengthPrefix`] | Encodes a value after its size. |
//! | [`Iter`] | Encodes an iterator of encodables as a sequence. |
#![cfg_attr(
    feature = "alloc",
    doc = r"## alloc encodables (requires `alloc` feature)

|         Type|                                                       Description |
|-------------|-------------------------------------------------------------------|
| [`Vec<u8>`] | Encodes a vector of bytes                                         |
| [`String`]  | Encodes a string as a UTF-8 byte sequence                         |
| [`CString`](std::ffi::CString) | Encodes a string as a byte sequence with a null terminator (`\0`) |
| [`Box`]     | Encodes a boxed value                                             |
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
