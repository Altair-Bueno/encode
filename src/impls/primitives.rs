use crate::Encodable;
use crate::Encoder;

impl<E: Encoder> Encodable<E> for char {
    type Error = E::Error;

    #[inline]
    fn encode(&self, encoder: &mut E) -> Result<(), Self::Error> {
        let mut buf = [0; 4];
        let s = self.encode_utf8(&mut buf);
        encoder.put_slice(s.as_bytes())
    }
}

impl<E: Encoder> Encodable<E> for u8 {
    type Error = E::Error;

    #[inline]
    fn encode(&self, encoder: &mut E) -> Result<(), Self::Error> {
        encoder.put_byte(*self)
    }
}

impl<E: Encoder> Encodable<E> for i8 {
    type Error = E::Error;

    #[inline]
    fn encode(&self, encoder: &mut E) -> Result<(), Self::Error> {
        #[allow(clippy::cast_sign_loss)]
        (*self as u8).encode(encoder)
    }
}

impl<Encoder> crate::Encodable<Encoder> for bool
where
    Encoder: crate::Encoder,
{
    type Error = Encoder::Error;

    #[inline]
    fn encode(&self, encoder: &mut Encoder) -> Result<(), Self::Error> {
        u8::from(*self).encode(encoder)
    }
}

impl<T, Encoder> crate::Encodable<Encoder> for &T
where
    T: crate::Encodable<Encoder> + ?Sized,
    Encoder: crate::Encoder,
{
    type Error = T::Error;

    #[inline]
    fn encode(&self, encoder: &mut Encoder) -> Result<(), Self::Error> {
        T::encode(self, encoder)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const BUF_SIZE: usize = 64;

    #[test]
    fn assert_that_chars_can_be_encoded() {
        let expected = b"a";
        let encodable = 'a';

        let mut buf = [0u8; BUF_SIZE];
        let mut encoder = &mut buf as &mut [u8];
        encodable.encode(&mut encoder).unwrap();
        let written = BUF_SIZE - encoder.len();
        let result = &buf[..written];

        assert_eq!(expected, result);
    }

    #[test]
    fn assert_that_unsigned_bytes_can_be_encoded() {
        let expected = b"\xFF";
        let encodable = u8::MAX;

        let mut buf = [0u8; BUF_SIZE];
        let mut encoder = &mut buf as &mut [u8];
        encodable.encode(&mut encoder).unwrap();
        let written = BUF_SIZE - encoder.len();
        let result = &buf[..written];

        assert_eq!(expected, result);
    }

    #[test]
    fn assert_that_signed_bytes_can_be_encoded() {
        let expected = b"\x80";
        let encodable = i8::MIN;

        let mut buf = [0u8; BUF_SIZE];
        let mut encoder = &mut buf as &mut [u8];
        encodable.encode(&mut encoder).unwrap();
        let written = BUF_SIZE - encoder.len();
        let result = &buf[..written];

        assert_eq!(expected, result);
    }

    #[test]
    fn assert_that_true_can_be_encoded() {
        let expected = b"\x01";
        let encodable = true;

        let mut buf = [0u8; BUF_SIZE];
        let mut encoder = &mut buf as &mut [u8];
        encodable.encode(&mut encoder).unwrap();
        let written = BUF_SIZE - encoder.len();
        let result = &buf[..written];

        assert_eq!(expected, result);
    }

    #[test]
    fn assert_that_false_can_be_encoded() {
        let expected = b"\x00";
        let encodable = false;

        let mut buf = [0u8; BUF_SIZE];
        let mut encoder = &mut buf as &mut [u8];
        encodable.encode(&mut encoder).unwrap();
        let written = BUF_SIZE - encoder.len();
        let result = &buf[..written];

        assert_eq!(expected, result);
    }

    #[test]
    fn assert_that_references_to_encodables_can_be_encoded() {
        let expected = b"\x00";
        let encodable = &false;

        let mut buf = [0u8; BUF_SIZE];
        let mut encoder = &mut buf as &mut [u8];
        encodable.encode(&mut encoder).unwrap();
        let written = BUF_SIZE - encoder.len();
        let result = &buf[..written];

        assert_eq!(expected, result);
    }
}
