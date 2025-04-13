use super::InsufficientSpace;
use crate::BaseEncoder;
use crate::ByteEncoder;

impl BaseEncoder for &mut [u8] {
    type Error = InsufficientSpace;
}

impl ByteEncoder for &mut [u8] {
    #[inline]
    fn put_slice(&mut self, slice: &[u8]) -> Result<(), Self::Error> {
        let (a, b) = core::mem::take(self)
            .split_at_mut_checked(slice.len())
            .ok_or(InsufficientSpace)?;
        a.copy_from_slice(slice);
        *self = b;
        Ok(())
    }

    #[inline]
    fn put_byte(&mut self, byte: u8) -> Result<(), Self::Error> {
        self.put_slice(&[byte])
    }
}

#[cfg(test)]
mod test {
    use crate::Encodable;

    #[test]
    fn assert_that_encoding_something_into_an_empty_slice_always_fails() {
        let mut encoder = &mut [0u8; 0] as &mut [u8];
        let encodable = "hello";
        assert!(
            encodable.encode(&mut encoder).is_err(),
            "Empty arrays should always fail"
        );
    }

    #[test]
    fn assert_that_slices_can_be_used_as_encoders() {
        const BUF_SIZE: usize = 64;
        let mut buf = [0u8; BUF_SIZE];
        let mut remaining_buf = &mut buf as &mut [u8];
        let encodable = "hello";

        encodable.encode(&mut remaining_buf).unwrap();
        let bytes_written = BUF_SIZE - remaining_buf.len();

        assert_eq!(bytes_written, 5, "The buffer should contain 5 bytes");
        assert_eq!(
            &buf[..bytes_written],
            b"hello",
            "The buffer should contain the encoded string"
        );
    }
}
