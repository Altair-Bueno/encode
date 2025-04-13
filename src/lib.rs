#![doc = include_str!("../README.md")]
#![forbid(
    missing_docs,
    missing_debug_implementations,
    unsafe_code,
    clippy::std_instead_of_core,
    clippy::std_instead_of_alloc,
    clippy::dbg_macro,
    clippy::exit,
    clippy::infinite_loop,
    clippy::mem_forget,
    clippy::panic,
    clippy::cargo,
    clippy::missing_const_for_fn,
    clippy::tabs_in_doc_comments,
    clippy::perf
)]
#![warn(clippy::pedantic)]
#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(feature = "alloc")]
extern crate alloc;

pub mod combinators;
mod encodables;
pub mod encoders;

/// A trait for types that can be encoded into a specific encoder.
///
/// Defines a generic interface for encoding data structures into
/// an encoder.
///
/// ## A note about purity
///
/// Implementations of `Encodable` **must be pure**. That means:
///
/// - **No side effects**: Implementations must not modify global or external
///   state.
/// - **Deterministic**: Given the same input and encoder, the output must
///   always be the same.
/// - **No panics**: Panicking inside an `encode` implementation is considered a
///   bug.
///
/// Ignoring these rules may lead to logic errors.
///
/// ## Encoder Specialization
///
/// On nightly Rust, you can use [trait specialization](https://rust-lang.github.io/rfcs/1210-impl-specialization.html)
/// to implement optimized encodings for specific encoders, such as
/// [`SizeEncoder`](encoders::SizeEncoder). For example, you may want to use
/// this if your encoder runs computationally expensive operations for obtaining
/// the size of the encoded form.
///
/// ## Errors
///
/// Implementations must return an appropriate error if encoding fails. Errors
/// can occur if:
/// - The encoder encounters an internal error (e.g., out of space).
/// - The encoded output would be invalid.
///
/// Control flow may depend on these errors (unlike [`core::fmt::Write`]).
pub trait Encodable<E>
where
    E: BaseEncoder,
{
    /// The error type returned by the `encode` method.
    ///
    /// This must include the encoder's error type via `From<E::Error>`.
    type Error: From<E::Error>;

    /// Encodes `self` into the given encoder.
    ///
    /// # Errors
    ///
    /// Returns an error if writing fails or if the encoded output would be
    /// invalid.
    fn encode(&self, encoder: &mut E) -> Result<(), Self::Error>;
}

/// A trait that defines common types and operations for encoders.
///
/// An encoder is a type that can gather encoded data into a specific output
/// format. These are typically buffers, but can also be other types that
/// perform some operation on the encoded result. An example type is the
/// [`SizeEncoder`] type, which counts how many bytes would be encoded and is
/// often used for sizing the output buffer before encoding.
///
/// The `BaseEncoder` trait is the foundation for all encoders, providing a
/// common interface for encoding operations. It's also the main building block
/// for all [`combinators`].
pub trait BaseEncoder {
    /// The error type returned by all encoding operations.
    ///
    /// For example, an encoder might return an error if the output buffer is
    /// full.
    type Error;
}

/// A trait for encoders that can handle UTF-8 encodables.
///
/// This trait extends [`BaseEncoder`] to include a types and methods
/// specifically used for handling UTF-8 encodables. Encoders may use this trait
/// as bounds for generic encodables to signal that the output will be UTF-8
/// encoded. Doing so allows encoders such as [`String`] to be used.
pub trait StrEncoder: BaseEncoder {
    /// Writes an [`str`] into the encoder.
    ///
    /// # Errors
    ///
    /// Returns an error if the encoder is unable to write the string
    /// due to capacity limits, encoding errors, or internal failures.
    fn put_str(&mut self, string: &str) -> Result<(), Self::Error>;
}

/// A trait for encoders that can handle byte encodables.
///
/// This trait extends [`BaseEncoder`] to include types and methods specifically
/// used for handling any kind of byte encodables. Encoders may use this
/// trait as bounds for generic encodables to signal that the output will be any
/// kind of byte steam.
///
/// Note that all [`ByteEncoder`]s also implement [`StrEncoder`].
pub trait ByteEncoder: BaseEncoder {
    /// Writes a slice of bytes into the encoder.
    ///
    /// # Errors
    ///
    /// Returns an error if the encoder cannot write the entire slice
    /// due to capacity limits, encoding errors, or internal failures.
    fn put_slice(&mut self, slice: &[u8]) -> Result<(), Self::Error>;
    /// Writes a single byte into the encoder.
    ///
    /// # Errors
    ///
    /// Returns an error if the encoder cannot write the byte
    /// due to capacity limits, encoding errors, or internal failures.
    fn put_byte(&mut self, byte: u8) -> Result<(), Self::Error>;
}

/// An extension trait for types that can compute the size of their encoded form
/// using a [`SizeEncoder`].
///
/// Use this trait to pre-compute buffer sizes or perform validations before
/// full encoding.
///
/// This trait is automatically implemented for all types that implement
/// [`Encodable`] for [`SizeEncoder`].
///
/// ## Errors
///
/// Returns an error if the [`Encodable`] fails to encode.
///
/// [`SizeEncoder`]: encoders::SizeEncoder
pub trait EncodableSize: Encodable<encoders::SizeEncoder> {
    /// Computes the size of the encoded representation of `self`.
    ///
    /// # Errors
    ///
    /// Returns an error if encoding fails internally during the size
    /// estimation.
    fn encoded_size(&self) -> Result<usize, Self::Error>;
}

impl<T> StrEncoder for T
where
    T: ByteEncoder,
{
    #[inline]
    fn put_str(&mut self, string: &str) -> Result<(), Self::Error> {
        string.as_bytes().encode(self)
    }
}

impl<T> EncodableSize for T
where
    T: Encodable<encoders::SizeEncoder>,
{
    #[inline]
    fn encoded_size(&self) -> Result<usize, Self::Error> {
        let mut encoder = encoders::SizeEncoder::new();
        self.encode(&mut encoder)?;
        Ok(encoder.size())
    }
}
