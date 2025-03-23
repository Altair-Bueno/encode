use crate::Encodable;
use crate::Encoder;
#[cfg(feature = "alloc")]
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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn assert_that_vecs_can_be_encoded() {
        let encodable = vec![1, 2, 3, 4, 5];
        encodable.encode(&mut ()).unwrap();
    }

    #[test]
    fn assert_that_boxes_can_be_encoded() {
        let encodable = Box::new(5u8);
        encodable.encode(&mut ()).unwrap();
    }

    #[test]
    fn assert_that_cows_can_be_encoded() {
        let encodable = Cow::Borrowed("hello");
        encodable.encode(&mut ()).unwrap();
    }

    #[test]
    fn assert_that_strings_can_be_encoded() {
        let encodable = String::from("hello");
        encodable.encode(&mut ()).unwrap();
    }
}
