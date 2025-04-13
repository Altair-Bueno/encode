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
pub mod encoders;
mod impls;

/// A trait for types that can be encoded/serialized into an encoder.
///
/// # Overview
///
/// The `Encodable` trait defines the interface for encoding types into a
/// specified encoder. This trait is generic over the encoder type, allowing
/// for flexibility and specialization.
///
/// # Implementation Requirements
///
/// Implementations of the `Encodable` trait **must** be pure, meaning they must
/// adhere to the following properties:
///
/// - **No Side Effects:** The encoding process should not cause any side
///   effects.
/// - **Deterministic Output:** The same input must always produce the same
///   output.
///
/// Examples of non-idempotent encodes include:
///
/// - Writing to a file or other I/O device.
/// - Using random number generators.
/// - Panicking during the encoding process.
/// - Modifying or relying on global state.
///
/// # Encoder Specialization (requires a nightly compiler)
///
/// The `Encodable` trait is designed to be flexible and can be specialized for
/// different encoder types. For example, if you are running some sort of heavy
/// computation for encoding a type, but you are sure of the final size of the
/// encoded data, you can specialize the encoder for `SizeEncoder`.
///
/// # Errors
///
/// Implementations of this trait shall return an appropriate error if encoding
/// fails due to an encoder error or if the encoder fails to store the encoded
/// data.
///
/// Note that, contrary to `core::fmt::Write`, control flow operations are
/// allowed based on encoding errors.
pub trait Encodable<E>
where
    E: BaseEncoder,
{
    /// The error type that can be returned when encoding `self`.
    ///
    /// For example, some encodables may abort encoding if encoding would
    /// produce an invalid byte stream as dictated by the encoding format.
    type Error: From<E::Error>;

    /// Encodes `self` into the given `encoder`.
    ///
    /// # Errors
    ///
    /// Implementations of this method should return an error if encoding fails
    /// due to an encoder error or if the encoding would produce an invalid byte
    /// stream as dictated by the encoding format.
    fn encode(&self, encoder: &mut E) -> Result<(), Self::Error>;
}
/// A type that can handle an [`Encodable`] type.
///
/// The `BaseEncoder` trait specifies the basic interface required for a type
/// to be used as an encoder for an [`Encodable`].
pub trait BaseEncoder {
    /// The error type that can be returned when encoding a value.
    ///
    /// For example, some encoders may return an error if the underlying buffer
    /// is full
    type Error;
}

/// A type that can handle UTF-8 [`Encodable`]s
pub trait StrEncoder: BaseEncoder {
    /// Appends a UTF-8 string into the encoder, if possible.
    ///
    /// # Errors
    ///
    /// Implementations of this method should return an error if the underlying
    /// encoder fails to write the string.
    fn put_str(&mut self, string: &str) -> Result<(), Self::Error>;
}

/// A type that can handle any [`Encodable`] type
///
/// An encoder is responsible for writing bytes into a buffer or other
/// destination. Encoders are used by encodables to write their data. Thus,
/// encoders must uphold the same properties as encodables.
pub trait ByteEncoder: BaseEncoder {
    /// Copies a slice of bytes into the encoder
    ///
    /// # Errors
    ///
    /// Implementations of this method should return an error if the underlying
    /// encoder fails to write the slice of bytes.
    fn put_slice(&mut self, slice: &[u8]) -> Result<(), Self::Error>;
    /// Appends a single byte into the encoder, if possible.
    ///
    /// # Errors
    ///
    /// Implementations of this method should return an error if the underlying
    /// encoder fails to write the byte.
    fn put_byte(&mut self, byte: u8) -> Result<(), Self::Error>;
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

/// An extension trait for types that can calculate the size of their encoded
/// form.
///
/// This trait is implemented for all types that implement the [`Encodable`]
/// trait for [`SizeEncoder`].
///
/// See the [`SizeEncoder`] encoder for more information.
///
/// [`SizeEncoder`]: encoders::SizeEncoder
pub trait EncodableSize: Encodable<encoders::SizeEncoder> {
    /// Returns the size of the encoded form of `self`.
    ///
    /// # Errors
    ///
    /// If encoding fails, this method will return an error.
    fn encoded_size(&self) -> Result<usize, Self::Error>;
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
