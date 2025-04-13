use crate::BaseEncoder;
use crate::ByteEncoder;
use crate::StrEncoder;
use alloc::string::String;
use alloc::vec::Vec;

impl BaseEncoder for Vec<u8> {
    type Error = core::convert::Infallible;
}

impl ByteEncoder for Vec<u8> {
    fn put_slice(&mut self, slice: &[u8]) -> Result<(), Self::Error> {
        self.extend(slice);
        Ok(())
    }

    fn put_byte(&mut self, byte: u8) -> Result<(), Self::Error> {
        self.push(byte);
        Ok(())
    }
}

impl BaseEncoder for String {
    type Error = core::convert::Infallible;
}

impl StrEncoder for String {
    fn put_str(&mut self, string: &str) -> Result<(), Self::Error> {
        self.push_str(string);
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn assert_that_vec_grows() {
        let mut buf = Vec::with_capacity(1);
        let encodable = b"hello";
        buf.put_slice(encodable).unwrap();
        assert_eq!(buf, b"hello", "The vector grows as necessary");
    }
}
