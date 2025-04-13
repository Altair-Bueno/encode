use bytes::Bytes;
use bytes::BytesMut;

use crate::ByteEncoder;
use crate::Encodable;

impl<E: ByteEncoder> Encodable<E> for Bytes {
    type Error = E::Error;

    #[inline]
    fn encode(&self, encoder: &mut E) -> Result<(), Self::Error> {
        let slice: &[u8] = &*self;
        slice.encode(encoder)
    }
}

impl<E: ByteEncoder> Encodable<E> for BytesMut {
    type Error = E::Error;

    #[inline]
    fn encode(&self, encoder: &mut E) -> Result<(), Self::Error> {
        let slice: &[u8] = &*self;
        slice.encode(encoder)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Encodable;

    const BUF_SIZE: usize = 64;

    #[test]
    fn assert_that_bytes_can_be_encoded() {
        let expected = b"\x01\x02\x03";
        let encodable = Bytes::from_static(&[1, 2, 3]);

        let mut buf = [0u8; BUF_SIZE];
        let mut encoder = &mut buf as &mut [u8];
        encodable.encode(&mut encoder).unwrap();
        let written = BUF_SIZE - encoder.len();
        let result = &buf[..written];

        assert_eq!(expected, result);
    }

    #[test]
    fn assert_that_bytes_mut_can_be_encoded() {
        let expected = b"\x01\x02\x03";
        let mut encodable = BytesMut::new();
        encodable.extend_from_slice(&[1, 2, 3]);

        let mut buf = [0u8; BUF_SIZE];
        let mut encoder = &mut buf as &mut [u8];
        encodable.encode(&mut encoder).unwrap();
        let written = BUF_SIZE - encoder.len();
        let result = &buf[..written];

        assert_eq!(expected, result);
    }
}
