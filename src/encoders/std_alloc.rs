cfg_if::cfg_if!(
    if #[cfg(feature = "std")] {
        use std as reexport;
    } else if #[cfg(feature = "alloc")] {
        extern crate alloc;

        use alloc as reexport;
    }
);

use reexport::vec::Vec;

use crate::Encoder;

impl Encoder for Vec<u8> {
    type Error = core::convert::Infallible;

    fn put_slice(&mut self, slice: &[u8]) -> Result<(), Self::Error> {
        self.extend(slice);
        Ok(())
    }

    fn put_byte(&mut self, byte: u8) -> Result<(), Self::Error> {
        self.push(byte);
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use crate::Encoder;

    #[test]
    fn assert_that_vec_grows() {
        let mut buf = Vec::with_capacity(1);
        let encodable = b"hello";
        buf.put_slice(encodable).unwrap();
        assert_eq!(buf, b"hello", "The vector grows as necessary");
    }
}