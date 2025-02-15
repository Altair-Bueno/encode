//! Encoders for encoding data into byte slices.
//!
//! This module contains different types that implement the [`Encoder`] trait.
//! As a summary, these types implement the [`Encodable`] trait:
//!
//! [`Encoder`]: crate::Encoder
//! [`Encodable`]: crate::Encodable
//!
//! - [`()`](unit): does nothing when encoding. Useful for testing.
//! - [`SizeEncoder`]: counts the number of bytes written.
//! - [`&mut [u8]`](slice): writes bytes into a slice, if there is enough space.
#![cfg_attr(
    any(feature = "std", feature = "alloc"),
    doc = "- [`Vec<u8>`] (`std` or `alloc` feature): writes bytes into a vector that grows if necessary."
)]
#![cfg_attr(
    feature = "arrayvec",
    doc = "- [`ArrayVec`](::arrayvec::ArrayVec) (`arrayvec` feature): writes bytes into an ArrayVec, if there is enough space."
)]
#[cfg(feature = "arrayvec")]
mod arrayvec;
mod errors;
mod size;
mod slices;
#[cfg(any(feature = "std", feature = "alloc"))]
mod std_alloc;

pub use errors::InsufficientSpace;
pub use size::SizeEncoder;

impl crate::Encoder for () {
    type Error = core::convert::Infallible;

    #[inline]
    fn put_slice(&mut self, _: &[u8]) -> Result<(), Self::Error> {
        Ok(())
    }

    #[inline]
    fn put_byte(&mut self, _: u8) -> Result<(), Self::Error> {
        Ok(())
    }
}
