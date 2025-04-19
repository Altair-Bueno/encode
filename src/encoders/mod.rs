//! Encoders for gathering encoded data into various output types.
//!
//! [`BaseEncoder`]: crate::BaseEncoder
//! [`StrEncoder`]: crate::StrEncoder
//! [`ByteEncoder`]: crate::ByteEncoder
//! [`Display`]: core::fmt::Display
//! [`Debug`]: core::fmt::Debug
//!
//! This module provides implementations of [`BaseEncoder`], [`StrEncoder`], and
//! [`ByteEncoder`] for a variety of output types, allowing encoded data to be
//! written into different buffer types, such as slices, strings, formatters and
//! more.
//!
//! # Supported Encoder Types
//!
//! | Type | Description | [`BaseEncoder`] | [`StrEncoder`] | [`ByteEncoder`] | Requires feature |
//! |------|-------------|-----------------|----------------|-----------------|------------------|
//! | [`()`](unit) | A no-op encoder. Useful for testing combinators or skipping output. | ✅ | ✅ | ✅ | - |
//! | [`Formatter`](core::fmt::Formatter) | Writes data into a Rust [`core::fmt::Write`]. Useful for implementing [`Display`] or [`Debug`]. | ✅ | ✅ | ❌ | - |
//! | [`SizeEncoder`] | Counts how many bytes would be encoded. Useful for sizing buffers. | ✅ | ✅ | ✅ | - |
//! | [`&mut [u8]`](slice) | Writes bytes into a fixed-size mutable slice. Fails if full. | ✅ | ✅ | ✅ | - |
#![cfg_attr(
    feature = "alloc",
    doc = "| [`Vec<u8>`](::alloc::vec::Vec) | Dynamically growing encoder that appends to a `Vec<u8>`. | ✅ | ✅ | ✅ | `alloc` OR `std` |"
)]
#![cfg_attr(
    feature = "alloc",
    doc = "| [`String`](::alloc::string::String) | Appends UTF-8 strings to a dynamically growing `String`. | ✅ | ✅ | ❌ | `alloc` OR `std` |"
)]
#![cfg_attr(
    feature = "std",
    doc = "| [`IoEncoder`] | Allows to use any [`std::io::Write`] implementor as an encoder. | ✅ | ✅ | ✅ | `std` |"
)]
#![cfg_attr(
    feature = "arrayvec",
    doc = "| [`ArrayVec`](::arrayvec::ArrayVec) | Encodes into a fixed-capacity `ArrayVec<u8, N>`. Fails if full. | ✅ | ✅ | ✅ | `arrayvec` |"
)]
#![cfg_attr(
    feature = "arrayvec",
    doc = "| [`ArrayString`](::arrayvec::ArrayString) | Encodes UTF-8 strings into a fixed-capacity `ArrayString`. Fails if full. | ✅ | ✅ | ❌ | `arrayvec` |"
)]
#![cfg_attr(
    feature = "bytes",
    doc = "| [`BufMut`](::bytes::BufMut) | Writes to any `BufMut` (e.g., from `bytes` crate). Note that preallocating the buffer improves performance. | ✅ | ✅ | ✅ | `bytes` |"
)]

#[cfg(feature = "alloc")]
mod alloc;
#[cfg(feature = "arrayvec")]
mod arrayvec;
#[cfg(feature = "bytes")]
mod bytes;
mod errors;
mod fmt;
mod primitives;
mod size;
mod slices;
#[cfg(feature = "std")]
mod std_io;

pub use errors::InsufficientSpace;
pub use size::SizeEncoder;
#[cfg(feature = "std")]
pub use std_io::IoEncoder;
