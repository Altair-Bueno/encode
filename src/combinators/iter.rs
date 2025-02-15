/// A combinator that encodes an iterator of encodables as a sequence.
///
/// # Example
///
/// ```
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
/// ```
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
#[repr(transparent)]
pub struct Iter<I> {
    encodable_iter: I,
}

impl<I> Iter<I> {
    /// Creates a new [`Iter`] combinator.
    #[inline]
    pub fn new(encodable_iter: I) -> Self {
        Self { encodable_iter }
    }
}

impl<EncodableIter, Encoder> crate::Encodable<Encoder> for Iter<EncodableIter>
where
    EncodableIter: IntoIterator + Clone,
    EncodableIter::Item: crate::Encodable<Encoder>,
    Encoder: crate::Encoder,
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
