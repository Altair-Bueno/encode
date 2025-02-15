cfg_if::cfg_if!(
    if #[cfg(feature = "std")] {
        use std as reexport;
    } else if #[cfg(feature = "alloc")] {
        extern crate alloc;

        use alloc as reexport;
    }
);

use reexport::borrow::Cow;
use reexport::boxed::Box;
use reexport::string::String;
use reexport::vec::Vec;

use crate::Encodable;
use crate::Encoder;

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
