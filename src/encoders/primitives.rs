use crate::BaseEncoder;
use crate::ByteEncoder;
use core::convert::Infallible;

impl BaseEncoder for () {
    type Error = Infallible;
}

impl ByteEncoder for () {
    #[inline]
    fn put_slice(&mut self, _: &[u8]) -> Result<(), Self::Error> {
        Ok(())
    }

    #[inline]
    fn put_byte(&mut self, _: u8) -> Result<(), Self::Error> {
        Ok(())
    }
}
