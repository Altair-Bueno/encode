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
    #[test]
    fn assert_that_chars_can_be_encoded() {
        let encodable = 'a';
        encodable.encode(&mut ()).unwrap();
    }

    #[test]
    fn assert_that_bytes_can_be_encoded() {
        let encodable = 42u8;
        encodable.encode(&mut ()).unwrap();
    }

    #[test]
    fn assert_that_signed_bytes_can_be_encoded() {
        let encodable = 42i8;
        encodable.encode(&mut ()).unwrap();
    }

    #[test]
    fn assert_that_bools_can_be_encoded() {
        let encodable = true;
        encodable.encode(&mut ()).unwrap();
    }

    #[test]
    fn assert_that_references_to_encodables_can_be_encoded() {
        let encodable = &42u8;
        encodable.encode(&mut ()).unwrap();
    }
}
