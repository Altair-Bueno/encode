use crate::BaseEncoder;
use crate::ByteEncoder;
use crate::StrEncoder;
use alloc::string::String;
use alloc::vec::Vec;

impl BaseEncoder for Vec<u8> {
    type Error = core::convert::Infallible;
}

impl ByteEncoder for Vec<u8> {
    #[inline]
    fn put_slice(&mut self, slice: &[u8]) -> Result<(), Self::Error> {
        self.extend(slice);
        Ok(())
    }

    #[inline]
    fn put_byte(&mut self, byte: u8) -> Result<(), Self::Error> {
        self.push(byte);
        Ok(())
    }
}

impl BaseEncoder for String {
    type Error = core::convert::Infallible;
}

impl StrEncoder for String {
    #[inline]
    fn put_str(&mut self, string: &str) -> Result<(), Self::Error> {
        self.push_str(string);
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::Encodable;

    #[test]
    fn assert_that_vec_grows() {
        let mut buf = Vec::with_capacity(1);
        let encodable = b"hello";

        encodable.encode(&mut buf).unwrap();

        assert_eq!(buf, b"hello", "The vector grows as necessary");
    }

    #[test]
    fn assert_that_string_grows() {
        let mut buf = String::with_capacity(1);
        let encodable = "hello";

        encodable.encode(&mut buf).unwrap();

        assert_eq!(buf, "hello", "The string grows as necessary");
    }
}
