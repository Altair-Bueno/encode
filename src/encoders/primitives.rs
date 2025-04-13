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

#[cfg(test)]
mod tests {
    use crate::Encodable;

    #[test]
    fn assert_that_unit_can_be_used_as_an_encoder() {
        let mut encoder = ();
        let encodable = ("hello", 0u8);
        encodable.encode(&mut encoder).unwrap();
    }
}
