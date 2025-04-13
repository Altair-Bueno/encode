use core::ffi::CStr;

use crate::ByteEncoder;
use crate::Encodable;
use crate::StrEncoder;

impl<E> Encodable<E> for [u8]
where
    E: ByteEncoder,
{
    type Error = E::Error;

    #[inline]
    fn encode(&self, encoder: &mut E) -> Result<(), Self::Error> {
        encoder.put_slice(self)
    }
}

impl<const SIZE: usize, E> Encodable<E> for [u8; SIZE]
where
    E: ByteEncoder,
{
    type Error = E::Error;

    #[inline]
    fn encode(&self, encoder: &mut E) -> Result<(), Self::Error> {
        (self as &[u8]).encode(encoder)
    }
}

impl<E> Encodable<E> for str
where
    E: StrEncoder,
{
    type Error = E::Error;

    #[inline]
    fn encode(&self, encoder: &mut E) -> Result<(), Self::Error> {
        encoder.put_str(self)
    }
}

impl<E> Encodable<E> for CStr
where
    E: ByteEncoder,
{
    type Error = E::Error;

    #[inline]
    fn encode(&self, encoder: &mut E) -> Result<(), Self::Error> {
        self.to_bytes_with_nul().encode(encoder)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const BUF_SIZE: usize = 64;

    #[test]
    fn assert_that_arrays_can_be_encoded() {
        let expected = b"\x01\x02\x03";
        let encodable: [u8; 3] = [1, 2, 3];

        let mut buf = [0u8; BUF_SIZE];
        let mut encoder = &mut buf as &mut [u8];
        encodable.encode(&mut encoder).unwrap();
        let written = BUF_SIZE - encoder.len();
        let result = &buf[..written];

        assert_eq!(expected, result);
    }

    #[test]
    fn assert_that_slices_can_be_encoded() {
        let expected = b"\x01\x02\x03";
        let encodable: &[u8] = &[1, 2, 3];

        let mut buf = [0u8; BUF_SIZE];
        let mut encoder = &mut buf as &mut [u8];
        encodable.encode(&mut encoder).unwrap();
        let written = BUF_SIZE - encoder.len();
        let result = &buf[..written];

        assert_eq!(expected, result);
    }

    #[test]
    fn assert_that_strs_can_be_encoded() {
        let expected = b"Hello world!";
        let encodable = "Hello world!";

        let mut buf = [0u8; BUF_SIZE];
        let mut encoder = &mut buf as &mut [u8];
        encodable.encode(&mut encoder).unwrap();
        let written = BUF_SIZE - encoder.len();
        let result = &buf[..written];

        assert_eq!(expected, result);
    }

    #[test]
    fn assert_that_cstrs_can_be_encoded() {
        let expected = b"Hello world!\0";
        let encodable = c"Hello world!";

        let mut buf = [0u8; BUF_SIZE];
        let mut encoder = &mut buf as &mut [u8];
        encodable.encode(&mut encoder).unwrap();
        let written = BUF_SIZE - encoder.len();
        let result = &buf[..written];

        assert_eq!(expected, result);
    }
}
