use core::ffi::CStr;

use crate::Encodable;
use crate::Encoder;

impl<E> Encodable<E> for [u8]
where
    E: Encoder,
{
    type Error = E::Error;

    #[inline]
    fn encode(&self, encoder: &mut E) -> Result<(), Self::Error> {
        encoder.put_slice(self)
    }
}

impl<const SIZE: usize, E> Encodable<E> for [u8; SIZE]
where
    E: Encoder,
{
    type Error = E::Error;

    #[inline]
    fn encode(&self, encoder: &mut E) -> Result<(), Self::Error> {
        encoder.put_slice(self)
    }
}

impl<E> Encodable<E> for str
where
    E: Encoder,
{
    type Error = E::Error;

    #[inline]
    fn encode(&self, encoder: &mut E) -> Result<(), Self::Error> {
        encoder.put_slice(self.as_bytes())
    }
}

impl<E> Encodable<E> for CStr
where
    E: Encoder,
{
    type Error = E::Error;

    #[inline]
    fn encode(&self, encoder: &mut E) -> Result<(), Self::Error> {
        encoder.put_slice(self.to_bytes_with_nul())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::EncodableSize as _;

    #[test]
    fn assert_that_arrays_can_be_encoded() {
        let encodable = [1, 2, 3, 4, 5];
        encodable.encode(&mut ()).unwrap();
    }

    #[test]
    fn assert_that_slices_can_be_encoded() {
        let encodable = [1, 2, 3, 4, 5].as_slice();
        encodable.encode(&mut ()).unwrap();
    }

    #[test]
    fn assert_that_strs_can_be_encoded() {
        let encodable = "Hello, world!";
        encodable.encode(&mut ()).unwrap();
    }

    #[test]
    fn assert_that_cstrs_can_be_encoded() {
        let encodable = c"Hello, world!";
        encodable.encode(&mut ()).unwrap();
        assert_eq!(
            encodable.encoded_size().unwrap(),
            encodable.count_bytes() + 1,
            "CStr must include the null terminator"
        )
    }
}
