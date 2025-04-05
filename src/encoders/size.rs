use core::fmt::Write;

use crate::Encoder;

/// An encoder that counts the size of the encoded data.
///
/// This encoder is useful for calculating the size of the encoded data without
/// actually encoding it, allowing you to pre-allocate a buffer of the correct
/// size before encoding the data.
///
/// Note that this encoder runs all the same encoding logic as any other encoder,
/// so it will trigger the same side effects that other encoders would trigger
/// (e.g Allocations). See the [`Encodable`] trait for more information on
/// idempotent encodes.
///
/// # Example
///
/// ```
/// use encode::Encoder;
/// use encode::Encodable;
/// use encode::encoders::SizeEncoder;
///
/// let encodable = c"hello, world!";
/// let mut encoder = SizeEncoder::new();
/// encodable.encode(&mut encoder).unwrap();
/// assert_eq!(encoder.size(), 14, "13 bytes from the ASCII string and 1 byte for the null terminator");
/// ```
///
/// [`Encodable`]: crate::Encodable
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default)]
pub struct SizeEncoder {
    size: usize,
}

impl SizeEncoder {
    /// Creates a new [`SizeEncoder`].
    #[inline]
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Returns the size of the encoded data.
    #[inline]
    #[must_use]
    pub fn size(&self) -> usize {
        (*self).into()
    }
}

impl From<SizeEncoder> for usize {
    fn from(encoder: SizeEncoder) -> usize {
        encoder.size
    }
}

impl Write for SizeEncoder {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        self.size += s.len();
        Ok(())
    }
}

impl Encoder for SizeEncoder {
    type Error = core::convert::Infallible;

    #[inline]
    fn put_slice(&mut self, slice: &[u8]) -> Result<(), Self::Error> {
        self.size += slice.len();
        Ok(())
    }

    #[inline]
    fn put_byte(&mut self, _byte: u8) -> Result<(), Self::Error> {
        self.size += 1;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn assert_that_size_encoder_can_be_used_with_format_strings() {
        let mut size_encoder = SizeEncoder::new();
        let s = "hello world";
        write!(size_encoder, "{s}").unwrap();
        assert_eq!(size_encoder.size(), s.len());
    }
}
