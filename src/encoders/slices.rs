use super::InsufficientSpace;
use crate::BaseEncoder;
use crate::ByteEncoder;

impl BaseEncoder for &mut [u8] {
    type Error = InsufficientSpace;
}

impl ByteEncoder for &mut [u8] {
    #[inline]
    fn put_slice(&mut self, slice: &[u8]) -> Result<(), Self::Error> {
        if self.len() < slice.len() {
            return Err(InsufficientSpace);
        }
        let (a, b) = core::mem::take(self).split_at_mut(slice.len());
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
    use crate::ByteEncoder;
    use crate::Encodable;

    #[test]
    fn assert_that_a_failed_write_keeps_the_remaining_space() {
        let mut buf = [0u8; 8];
        let mut encoder = &mut buf as &mut [u8];
        encoder.put_slice(b"abc").unwrap();

        encoder.put_slice(b"0123456789").unwrap_err();

        assert_eq!(encoder.len(), 5);
    }

    #[test]
    fn assert_that_writes_that_fit_succeed_after_a_failed_write() {
        let mut buf = [0u8; 8];
        let mut encoder = &mut buf as &mut [u8];
        encoder.put_slice(b"abc").unwrap();
        encoder.put_slice(b"0123456789").unwrap_err();

        encoder.put_slice(b"de").unwrap();

        assert_eq!(&buf[..5], b"abcde");
    }

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
