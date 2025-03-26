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

impl<E, T> Encodable<E> for Cow<'_, T>
where
    T: ToOwned + Encodable<E> + ?Sized,
    E: Encoder,
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
mod tests {
    use alloc::borrow::Cow;

    use crate::Encodable;

    #[test]
    fn assert_that_cow_slice_can_be_encoded() {
        let cow: Cow<'_, [u8]> = Cow::Borrowed(&[][..]);
        // Explicit fully qualified call because otherwise autoref could just encode `&[u8]`.
        <Cow<'_, _> as Encodable<()>>::encode(&cow, &mut ()).unwrap();
    }
}
