use bytes::BufMut;
use bytes::BytesMut;

use crate::BaseEncoder;
use crate::ByteEncoder;

impl BaseEncoder for BytesMut {
    type Error = core::convert::Infallible;
}

impl ByteEncoder for BytesMut {
    #[inline]
    fn put_slice(&mut self, slice: &[u8]) -> Result<(), Self::Error> {
        BufMut::put_slice(self, slice);
        Ok(())
    }

    #[inline]
    fn put_byte(&mut self, byte: u8) -> Result<(), Self::Error> {
        BufMut::put_u8(self, byte);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::Encodable;

    use super::*;

    #[test]
    fn assert_that_bytesmut_can_be_used_as_encoder() {
        let mut buf = BytesMut::new();
        let encodable = ("hello", 0u8);

        encodable.encode(&mut buf).unwrap();
        let bytes = buf.freeze();

        assert_eq!(bytes.len(), 6, "The buffer should contain 5 bytes");
        assert_eq!(bytes, b"hello\0"[..]);
    }
}
