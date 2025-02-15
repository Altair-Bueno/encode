use arrayvec::ArrayVec;

use super::InsufficientSpace;
use crate::Encoder;

impl<const SIZE: usize> Encoder for ArrayVec<u8, SIZE> {
    type Error = InsufficientSpace;

    #[inline]
    fn put_slice(&mut self, slice: &[u8]) -> Result<(), Self::Error> {
        self.try_extend_from_slice(slice)
            .map_err(|_| InsufficientSpace)
    }

    #[inline]
    fn put_byte(&mut self, byte: u8) -> Result<(), Self::Error> {
        self.try_push(byte).map_err(|_| InsufficientSpace)
    }
}

#[cfg(test)]
mod test {
    use crate::Encodable;
    use arrayvec::ArrayVec;

    #[test]
    fn assert_that_encoding_something_into_an_empty_arrayvec_always_fails() {
        let mut encoder = ArrayVec::<u8, 0>::new();
        let encodable = "hello";
        assert!(
            encodable.encode(&mut encoder).is_err(),
            "Empty arrays should always fail"
        );
    }

    #[test]
    fn assert_that_slices_can_be_used_as_encoders() {
        let mut buf = ArrayVec::<u8, 64>::new();
        let encodable = "hello";

        encodable.encode(&mut buf).unwrap();

        assert_eq!(buf.len(), 5, "The buffer should contain 5 bytes");
        assert_eq!(
            buf.as_slice(),
            b"hello",
            "The buffer should contain the encoded string"
        );
    }
}
