use crate::BaseEncoder;
use crate::ByteEncoder;
use crate::Encodable;
use crate::StrEncoder;
use alloc::{borrow::Cow, borrow::ToOwned, boxed::Box, ffi::CString, string::String, vec::Vec};

impl<E: ByteEncoder> Encodable<E> for Vec<u8> {
    type Error = E::Error;

    #[inline]
    fn encode(&self, encoder: &mut E) -> Result<(), Self::Error> {
        self.as_slice().encode(encoder)
    }
}

impl<E: BaseEncoder, T: Encodable<E>> Encodable<E> for Box<T> {
    type Error = T::Error;

    #[inline]
    fn encode(&self, encoder: &mut E) -> Result<(), Self::Error> {
        self.as_ref().encode(encoder)
    }
}

impl<E, T> Encodable<E> for Cow<'_, T>
where
    T: ToOwned + Encodable<E> + ?Sized,
    E: BaseEncoder,
{
    type Error = T::Error;

    #[inline]
    fn encode(&self, encoder: &mut E) -> Result<(), Self::Error> {
        self.as_ref().encode(encoder)
    }
}

impl<E: StrEncoder> Encodable<E> for String {
    type Error = E::Error;

    #[inline]
    fn encode(&self, encoder: &mut E) -> Result<(), Self::Error> {
        self.as_str().encode(encoder)
    }
}

impl<E: ByteEncoder> Encodable<E> for CString {
    type Error = E::Error;

    #[inline]
    fn encode(&self, encoder: &mut E) -> Result<(), Self::Error> {
        self.as_c_str().encode(encoder)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::vec;

    #[test]
    fn assert_that_boxes_can_be_encoded() {
        let encodable = Box::new(5u8);
        encodable.encode(&mut ()).unwrap();
    }

    #[test]
    fn assert_that_cows_can_be_encoded() {
        let cow = Cow::Borrowed("hello");
        // Explicit fully qualified call because otherwise autoref could just encode
        // `&[u8]`.
        <Cow<'_, _> as Encodable<()>>::encode(&cow, &mut ()).unwrap();
    }

    #[test]
    fn assert_that_vecs_can_be_encoded() {
        let encodable = vec![1, 2, 3, 4, 5];
        encodable.encode(&mut ()).unwrap();
    }

    #[test]
    fn assert_that_strings_can_be_encoded() {
        let encodable = String::from("hello");
        encodable.encode(&mut ()).unwrap();
    }

    #[test]
    fn assert_that_cstrings_can_be_encoded() {
        let encodable = CString::from(c"hello");
        encodable.encode(&mut ()).unwrap();
    }
}
