#![doc = include_str!("../README.md")]
#![forbid(
    missing_docs,
    unsafe_code,
    clippy::std_instead_of_core,
    clippy::std_instead_of_alloc,
    clippy::dbg_macro,
    clippy::exit,
    clippy::infinite_loop,
    clippy::mem_forget,
    clippy::panic,
    clippy::cargo
)]
#![warn(clippy::pedantic)]
#![cfg_attr(not(feature = "std"), no_std)]

pub mod combinators;
pub mod encoders;
mod impls;

/// A type that can be encoded into an encoder.
///
/// An encoder must be pure, meaning that it has to uphold the following properties:
///
/// - It must have no side effects
/// - It must produce the same outputs for the same inputs
///
/// Some examples of non-indepotent encodes are:
///
/// - Writing to a file or other I/O device
/// - Using random number generators
/// - Panics
/// - Using global state
pub trait Encodable<E>
where
    E: Encoder,
{
    /// The error type that can be returned when encoding `self`.
    ///
    /// For example, some encodables may abort encoding if encoding would produce
    /// an invalid byte stream as dictated by the encoding format.
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

/// A type that can handle an [`Encodable`] type
///
/// An encoder is responsible for writing bytes into a buffer or other
/// destination. Encoders are used by encodables to write their data. Thus,
/// encoders must uphold the same properties as encodables.
pub trait Encoder {
    /// The error type that can be returned when encoding a value.
    ///
    /// For example, some encoders may return an error if the underlying buffer
    /// is full
    type Error;

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

/// An extension trait for types that can calculate the size of their encoded form.
///
/// This trait is implemented for all types that implement the [`Encodable`] trait for [`SizeEncoder`].
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
