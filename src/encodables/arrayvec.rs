use crate::ByteEncoder;
use crate::Encodable;
use crate::StrEncoder;

use arrayvec::ArrayString;
use arrayvec::ArrayVec;

impl<E: ByteEncoder, const N: usize> Encodable<E> for ArrayVec<u8, N> {
    type Error = E::Error;

    #[inline]
    fn encode(&self, encoder: &mut E) -> Result<(), Self::Error> {
        let slice: &[u8] = &*self;
        slice.encode(encoder)
    }
}

impl<E: StrEncoder, const N: usize> Encodable<E> for ArrayString<N> {
    type Error = E::Error;

    #[inline]
    fn encode(&self, encoder: &mut E) -> Result<(), Self::Error> {
        let slice: &str = &*self;
        slice.encode(encoder)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Encodable;

    const BUF_SIZE: usize = 64;

    #[test]
    fn assert_that_arrayvec_can_be_encoded() {
        let expected = b"\x01\x02\x03";
        let encodable = ArrayVec::<u8, 3>::from([1, 2, 3]);

        let mut buf = [0u8; BUF_SIZE];
        let mut encoder = &mut buf as &mut [u8];
        encodable.encode(&mut encoder).unwrap();
        let written = BUF_SIZE - encoder.len();
        let result = &buf[..written];

        assert_eq!(expected, result);
    }

    #[test]
    fn assert_that_arraystring_can_be_encoded() {
        let expected = b"abc";
        let encodable = ArrayString::<3>::from("abc").unwrap();

        let mut buf = [0u8; BUF_SIZE];
        let mut encoder = &mut buf as &mut [u8];
        encodable.encode(&mut encoder).unwrap();
        let written = BUF_SIZE - encoder.len();
        let result = &buf[..written];

        assert_eq!(expected, result);
    }
}
