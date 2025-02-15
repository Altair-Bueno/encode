use core::ops::Deref;

/// A combinator that transforms the error type of an encodable.
///
/// This combinator is useful when you want to encode an encodable that has an
/// error type that is not compatible with another encodable.
///
/// # Example
///
/// ```
/// # #[cfg(feature = "alloc")] {
/// use encode::Encodable;
/// use encode::combinators::FromError;
///
/// let mut buf = Vec::new();
/// FromError::<_, std::num::TryFromIntError>::new("hello").encode(&mut buf).unwrap();
/// assert_eq!(&buf, b"hello");
/// # }
/// ```
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
#[repr(transparent)]
pub struct FromError<E, DstError> {
    encodable: E,
    error: core::marker::PhantomData<DstError>,
}

impl<E, DstError> FromError<E, DstError> {
    /// Creates a new [`FromError`] combinator.
    #[inline]
    pub fn new(encodable: E) -> Self {
        Self {
            encodable,
            error: core::marker::PhantomData,
        }
    }
}

impl<E, DstError> Deref for FromError<E, DstError> {
    type Target = E;
    fn deref(&self) -> &Self::Target {
        self.as_ref()
    }
}
impl<E, DstError> AsRef<E> for FromError<E, DstError> {
    #[inline]
    fn as_ref(&self) -> &E {
        &self.encodable
    }
}

impl<Encodable, Encoder, DstError> crate::Encodable<Encoder> for FromError<Encodable, DstError>
where
    DstError: From<Encodable::Error> + From<Encoder::Error>,
    Encodable: crate::Encodable<Encoder>,
    Encoder: crate::Encoder,
{
    type Error = DstError;

    #[inline]
    fn encode(&self, encoder: &mut Encoder) -> Result<(), Self::Error> {
        self.encodable.encode(encoder)?;
        Ok(())
    }
}
