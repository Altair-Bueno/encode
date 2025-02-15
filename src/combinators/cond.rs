use core::ops::Deref;

/// Conditionally encodes an encodable.
///
/// # Examples
///
/// ```rust
/// use encode::Encodable;
/// use encode::combinators::Cond;
/// use std::ffi::CStr;
///
/// let non_empty = |s:&&CStr| !s.is_empty();
///
/// let mut buf = Vec::new();
/// Cond::new(c"hello", non_empty).encode(&mut buf).unwrap();
/// assert_eq!(&buf, b"hello\0", "A non-empty CStr includes the null terminator");
///
/// buf.clear();
///
/// Cond::new(c"", non_empty).encode(&mut buf).unwrap();
/// assert_eq!(&buf, b"", "An empty CStr does not produce any output");
/// ```
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Cond<E, F> {
    encodable: E,
    condition: F,
}

impl<E, F> Cond<E, F> {
    /// Creates a new [`Cond`] combinator.
    #[inline]
    pub fn new(encodable: E, condition: F) -> Self
    where
        F: Fn(&E) -> bool,
    {
        Self {
            encodable,
            condition,
        }
    }
}

impl<E, F> AsRef<E> for Cond<E, F> {
    #[inline]
    fn as_ref(&self) -> &E {
        &self.encodable
    }
}

impl<E, F> Deref for Cond<E, F> {
    type Target = E;
    #[inline]
    fn deref(&self) -> &Self::Target {
        self.as_ref()
    }
}

impl<Encodable, Encoder, F> crate::Encodable<Encoder> for Cond<Encodable, F>
where
    Encodable: crate::Encodable<Encoder>,
    Encoder: crate::Encoder,
    F: Fn(&Encodable) -> bool,
{
    type Error = Encodable::Error;

    #[inline]
    fn encode(&self, encoder: &mut Encoder) -> Result<(), Self::Error> {
        if (self.condition)(&self.encodable) {
            self.encodable.encode(encoder)
        } else {
            Ok(())
        }
    }
}
