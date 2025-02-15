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
