use core::borrow::Borrow;
use core::fmt::Debug;
use core::marker::PhantomData;
use core::ops::Deref;

/// Transforms the error type of an encodable.
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
    #[inline]
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
    #[inline]
    fn borrow(&self) -> &E {
        &self.encodable
    }
}

impl<Encodable, Encoder, DstError> crate::Encodable<Encoder> for FromError<Encodable, DstError>
where
    DstError: From<Encodable::Error> + From<Encoder::Error>,
    Encodable: crate::Encodable<Encoder>,
    Encoder: crate::BaseEncoder,
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
    #[inline]
    fn clone(&self) -> Self {
        Self {
            encodable: self.encodable.clone(),
            error: PhantomData,
        }
    }
}
impl<E: Copy, DstError> Copy for FromError<E, DstError> {}
impl<E: Default, DstError> Default for FromError<E, DstError> {
    #[inline]
    fn default() -> Self {
        Self {
            encodable: Default::default(),
            error: PhantomData,
        }
    }
}
impl<E: PartialEq, DstError> PartialEq for FromError<E, DstError> {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.encodable == other.encodable && self.error == other.error
    }
}
impl<E: Eq, DstError> Eq for FromError<E, DstError> {}
impl<E: PartialOrd, DstError> PartialOrd for FromError<E, DstError> {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        self.encodable.partial_cmp(&other.encodable)
    }
}
impl<E: Ord, DstError> Ord for FromError<E, DstError> {
    #[inline]
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        self.encodable.cmp(&other.encodable)
    }
}

#[cfg(test)]
mod tests {
    use core::borrow::Borrow;
    use core::convert::Infallible;

    use rstest::rstest;

    use super::*;
    use crate::Encodable;

    const BUF_SIZE: usize = 32;

    #[test]
    fn assert_that_from_error_encodes_value() {
        let fe = FromError::<u8, crate::encoders::InsufficientSpace>::new(1u8);
        let mut buf = [0u8; BUF_SIZE];
        let mut encoder = &mut buf as &mut [u8];
        fe.encode(&mut encoder).unwrap();
        let written = BUF_SIZE - encoder.len();
        assert_eq!(&buf[..written], &[1u8]);
    }

    #[test]
    fn assert_that_from_error_into_inner_returns_value() {
        let fe = FromError::<_, Infallible>::new(42u8);
        assert_eq!(fe.into_inner(), 42u8);
    }

    #[test]
    fn assert_that_from_error_deref_works() {
        let fe = FromError::<_, Infallible>::new(42u8);
        assert_eq!(*fe, 42u8);
    }

    #[test]
    fn assert_that_from_error_as_ref_works() {
        let fe = FromError::<_, Infallible>::new(42u8);
        assert_eq!(fe.as_ref(), &42u8);
    }

    #[test]
    fn assert_that_from_error_borrow_works() {
        let fe = FromError::<_, Infallible>::new(42u8);
        let borrowed: &u8 = fe.borrow();
        assert_eq!(*borrowed, 42u8);
    }

    #[test]
    fn assert_that_from_error_clone_works() {
        let fe = FromError::<_, Infallible>::new(42u8);
        let clone = fe.clone();
        assert_eq!(*fe, *clone);
    }

    #[test]
    fn assert_that_from_error_default_works() {
        let fe = FromError::<u8, Infallible>::default();
        assert_eq!(*fe, 0u8);
    }

    #[rstest]
    #[case::less(1u8, 2u8, core::cmp::Ordering::Less)]
    #[case::equal(42u8, 42u8, core::cmp::Ordering::Equal)]
    #[case::greater(2u8, 1u8, core::cmp::Ordering::Greater)]
    fn assert_that_from_error_comparison_works(
        #[case] a: u8,
        #[case] b: u8,
        #[case] expected: core::cmp::Ordering,
    ) {
        let fe1 = FromError::<_, Infallible>::new(a);
        let fe2 = FromError::<_, Infallible>::new(b);
        assert_eq!(fe1.cmp(&fe2), expected);
        assert_eq!(fe1 == fe2, expected == core::cmp::Ordering::Equal);
    }

    #[cfg(feature = "alloc")]
    #[test]
    fn assert_that_from_error_debug_works() {
        let fe = FromError::<_, Infallible>::new(42u8);
        let debug_str = alloc::format!("{fe:?}");
        assert!(debug_str.contains("42"));
    }
}
