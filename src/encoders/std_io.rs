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

#[cfg(test)]
mod tests {
    use std::borrow::BorrowMut;
    use std::io::Cursor;

    use super::*;
    use crate::Encodable;

    #[test]
    fn assert_that_io_encoder_can_encode_bytes() {
        let mut encoder = IoEncoder(Cursor::new(Vec::new()));
        (b"hello" as &[u8], 0u8).encode(&mut encoder).unwrap();
        assert_eq!(encoder.0.into_inner(), b"hello\0");
    }

    #[test]
    fn assert_that_io_encoder_put_slice_writes_bytes() {
        let mut encoder = IoEncoder(Cursor::new(Vec::new()));
        encoder.put_slice(b"world").unwrap();
        assert_eq!(encoder.0.into_inner(), b"world");
    }

    #[test]
    fn assert_that_io_encoder_put_byte_writes_a_single_byte() {
        let mut encoder = IoEncoder(Cursor::new(Vec::new()));
        encoder.put_byte(0xAB).unwrap();
        assert_eq!(encoder.0.into_inner(), [0xABu8]);
    }

    #[test]
    fn assert_that_io_encoder_as_ref_works() {
        let encoder = IoEncoder(Cursor::new(Vec::new()));
        let _: &Cursor<Vec<u8>> = encoder.as_ref();
    }

    #[test]
    fn assert_that_io_encoder_as_mut_works() {
        let mut encoder = IoEncoder(Cursor::new(Vec::new()));
        let _: &mut Cursor<Vec<u8>> = encoder.as_mut();
    }

    #[test]
    fn assert_that_io_encoder_borrow_works() {
        use std::borrow::Borrow;
        let encoder = IoEncoder(Cursor::new(Vec::new()));
        let _: &Cursor<Vec<u8>> = encoder.borrow();
    }

    #[test]
    fn assert_that_io_encoder_borrow_mut_works() {
        let mut encoder = IoEncoder(Cursor::new(Vec::new()));
        let _: &mut Cursor<Vec<u8>> = encoder.borrow_mut();
    }

    #[test]
    fn assert_that_io_encoder_deref_works() {
        let encoder = IoEncoder(Cursor::new(Vec::new()));
        let _: &Cursor<Vec<u8>> = &*encoder;
    }

    #[test]
    fn assert_that_io_encoder_deref_mut_works() {
        let mut encoder = IoEncoder(Cursor::new(Vec::new()));
        let _: &mut Cursor<Vec<u8>> = &mut *encoder;
    }
}
