use core::borrow::Borrow;
use core::fmt::Debug;
use core::marker::PhantomData;
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
#[repr(transparent)]
pub struct FromError<E, DstError> {
    encodable: E,
    error: PhantomData<DstError>,
}

impl<E, DstError> FromError<E, DstError> {
    /// Creates a new [`FromError`] combinator.
    #[inline]
    #[must_use]
    pub const fn new(encodable: E) -> Self {
        Self {
            encodable,
            error: PhantomData,
        }
    }
    /// Consumes the [`FromError`] combinator and returns the inner value.
    #[inline]
    #[must_use]
    pub fn into_inner(self) -> E {
        self.encodable
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
impl<E, DstError> Borrow<E> for FromError<E, DstError> {
    fn borrow(&self) -> &E {
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

// Manual trait implementations because the derive macro does not support
// phantom data fields.
impl<E: Debug, DstError> Debug for FromError<E, DstError> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("FromError")
            .field("encodable", &self.encodable)
            .finish()
    }
}
impl<E: Clone, DstError> Clone for FromError<E, DstError> {
    fn clone(&self) -> Self {
        Self {
            encodable: self.encodable.clone(),
            error: PhantomData,
        }
    }
}
impl<E: Copy, DstError> Copy for FromError<E, DstError> {}
impl<E: Default, DstError> Default for FromError<E, DstError> {
    fn default() -> Self {
        Self {
            encodable: Default::default(),
            error: PhantomData,
        }
    }
}
impl<E: PartialEq, DstError> PartialEq for FromError<E, DstError> {
    fn eq(&self, other: &Self) -> bool {
        self.encodable == other.encodable && self.error == other.error
    }
}
impl<E: Eq, DstError> Eq for FromError<E, DstError> {}
impl<E: PartialOrd, DstError> PartialOrd for FromError<E, DstError> {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        self.encodable.partial_cmp(&other.encodable)
    }
}
impl<E: Ord, DstError> Ord for FromError<E, DstError> {
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        self.encodable.cmp(&other.encodable)
    }
}
