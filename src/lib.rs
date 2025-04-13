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

/// A trait that defines common Encoder types and operations that are used to
/// encode [`Encodable`] types.
///
/// The `BaseEncoder` trait is the foundation for all encoders, providing a
/// common interface for encoding operations. It's the main building block for
/// all [`combinators`].
pub trait BaseEncoder {
    /// The error type returned by all encoding operations.
    ///
    /// For example, an encoder might return an error if the output buffer is
    /// full.
    type Error;
}

/// A trait for encoders that can encode UTF-8 strings.
///
/// This trait extends [`BaseEncoder`] to include a method specifically for
/// handling UTF-8 string values. It is implemented by encoders that support
/// text encoding.
pub trait StrEncoder: BaseEncoder {
    /// Writes an [`str`] into the encoder.
    ///
    /// # Errors
    ///
    /// Returns an error if the encoder is unable to write the string
    /// due to capacity limits, encoding errors, or internal failures.
    fn put_str(&mut self, string: &str) -> Result<(), Self::Error>;
}

/// A trait for encoders that can encode raw byte data.
///
/// This trait extends [`BaseEncoder`] with methods for writing individual
/// bytes or byte slices. It is implemented by encoders that support
/// binary encoding.
///
/// Note that all [`ByteEncoder`]s also implement [`StrEncoder`].
pub trait ByteEncoder: BaseEncoder {
    /// Writes a slice of bytes into the encoder.
    ///
    /// # Errors
    ///
    /// Returns an error if the encoder cannot write the entire slice.
    fn put_slice(&mut self, slice: &[u8]) -> Result<(), Self::Error>;
    /// Writes a single byte into the encoder.
    ///
    /// # Errors
    ///
    /// Returns an error if the encoder cannot write the byte.
    fn put_byte(&mut self, byte: u8) -> Result<(), Self::Error>;
}

/// An extension trait for types that can compute the size of their encoded form
/// using a [`SizeEncoder`].
///
/// This trait is automatically implemented for all types that implement
/// [`Encodable`] for [`SizeEncoder`].
///
/// Use this trait to pre-compute buffer sizes or perform validations before
/// full encoding.
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
