use crate::BaseEncoder;
use crate::ByteEncoder;
use std::io::Write;

/// An encoder that adapts [`std::io::Write`] to the [`BaseEncoder`] and [`ByteEncoder`] traits.
///
/// This encoder allows you to write encoded data directly to any type that implements
/// [`std::io::Write`], such as a file or network stream.
///
/// # Example
///
/// ```
/// use encode::Encodable;
/// use encode::encoders::IoEncoder;
///
/// let mut encoder = IoEncoder(std::io::stdout().lock());
/// (b"hello, world!", 0u8).encode(&mut encoder).unwrap();
/// ```
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, PartialOrd, Ord)]
pub struct IoEncoder<W>(pub W);

impl<W> core::convert::AsRef<W> for IoEncoder<W> {
    #[inline]
    fn as_ref(&self) -> &W {
        &self.0
    }
}

impl<W> core::convert::AsMut<W> for IoEncoder<W> {
    #[inline]
    fn as_mut(&mut self) -> &mut W {
        &mut self.0
    }
}

impl<W> core::borrow::Borrow<W> for IoEncoder<W> {
    #[inline]
    fn borrow(&self) -> &W {
        &self.0
    }
}

impl<W> core::borrow::BorrowMut<W> for IoEncoder<W> {
    #[inline]
    fn borrow_mut(&mut self) -> &mut W {
        &mut self.0
    }
}

impl<W> core::ops::Deref for IoEncoder<W> {
    type Target = W;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<W> core::ops::DerefMut for IoEncoder<W> {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<W> BaseEncoder for IoEncoder<W>
where
    W: Write,
{
    type Error = std::io::Error;
}

impl<W> ByteEncoder for IoEncoder<W>
where
    W: Write,
{
    #[inline]
    fn put_slice(&mut self, slice: &[u8]) -> Result<(), Self::Error> {
        self.0.write_all(slice)
    }

    #[inline]
    fn put_byte(&mut self, byte: u8) -> Result<(), Self::Error> {
        self.0.write_all(&[byte] as &[u8])
    }
}
