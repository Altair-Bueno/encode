use crate::Encodable;
use crate::Encoder;
use alloc::{borrow::Cow, boxed::Box, string::String, vec::Vec};

impl<E: Encoder> Encodable<E> for Vec<u8> {
    type Error = E::Error;

    #[inline]
    fn encode(&self, encoder: &mut E) -> Result<(), Self::Error> {
        encoder.put_slice(self.as_slice())
    }
}

impl<E: Encoder, T: Encodable<E>> Encodable<E> for Box<T> {
    type Error = T::Error;

    #[inline]
    fn encode(&self, encoder: &mut E) -> Result<(), Self::Error> {
        self.as_ref().encode(encoder)
    }
}

impl<E: Encoder, T: Encodable<E>> Encodable<E> for Cow<'_, T>
where
    T: Clone + Encodable<E>,
{
    type Error = T::Error;

    #[inline]
    fn encode(&self, encoder: &mut E) -> Result<(), Self::Error> {
        self.as_ref().encode(encoder)
    }
}

impl<E: Encoder> Encodable<E> for String {
    type Error = E::Error;

    #[inline]
    fn encode(&self, encoder: &mut E) -> Result<(), Self::Error> {
        encoder.put_slice(self.as_bytes())
    }
}
