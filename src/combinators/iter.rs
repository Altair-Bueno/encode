use core::borrow::Borrow;
use core::ops::Deref;

/// A combinator that encodes an iterator of encodables as a sequence.
///
/// # Example
///
/// ```
/// # #[cfg(feature = "alloc")] {
/// use encode::Encodable;
/// use encode::combinators::Iter;
///
/// let compact_map = [
///     (c"hello", 1u8),
///     (c"world", 2u8),
/// ];
/// let mut buf = Vec::new();
/// Iter::new(&compact_map).encode(&mut buf).unwrap();
/// assert_eq!(&buf, b"hello\0\x01world\0\x02");
/// # }
/// ```
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
#[repr(transparent)]
pub struct Iter<I> {
    encodable_iter: I,
}

impl<I> Iter<I> {
    /// Creates a new [`Iter`] combinator.
    #[inline]
    #[must_use]
    pub const fn new(encodable_iter: I) -> Self {
        Self { encodable_iter }
    }
    /// Consumes the [`Iter`] combinator and returns the inner value.
    #[inline]
    #[must_use]
    pub fn into_inner(self) -> I {
        self.encodable_iter
    }
}

impl<I> AsRef<I> for Iter<I> {
    #[inline]
    fn as_ref(&self) -> &I {
        &self.encodable_iter
    }
}
impl<I> Borrow<I> for Iter<I> {
    #[inline]
    fn borrow(&self) -> &I {
        &self.encodable_iter
    }
}
impl<I> Deref for Iter<I> {
    type Target = I;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.encodable_iter
    }
}

impl<EncodableIter, Encoder> crate::Encodable<Encoder> for Iter<EncodableIter>
where
    EncodableIter: IntoIterator + Clone,
    EncodableIter::Item: crate::Encodable<Encoder>,
    Encoder: crate::BaseEncoder,
{
    type Error = <EncodableIter::Item as crate::Encodable<Encoder>>::Error;

    #[inline]
    fn encode(&self, encoder: &mut Encoder) -> Result<(), Self::Error> {
        for encodable in self.encodable_iter.clone() {
            encodable.encode(encoder)?;
        }
        Ok(())
    }
}
