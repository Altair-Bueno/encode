/// Encodes a sequence of encodables separated by a given delimiter.
///
/// # Example
///
/// ```
/// # #[cfg(feature = "alloc")] {
/// use encode::Encodable;
/// use encode::combinators::Separated;
///
/// let mut buf = Vec::new();
/// let array = ["hello", "world", "another"];
/// Separated::new(&array, ", ").encode(&mut buf).unwrap();
/// assert_eq!(&buf, b"hello, world, another");
/// # }
/// ```
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Separated<I, S> {
    encodable_iter: I,
    separator: S,
}

impl<I, S> Separated<I, S> {
    /// Creates a new [`Separated`] combinator.
    #[inline]
    pub const fn new(encodable_iter: I, separator: S) -> Self {
        Self {
            encodable_iter,
            separator,
        }
    }
    /// Consumes the [`Separated`] combinator and returns the inner value.
    #[inline]
    #[must_use]
    pub fn into_inner(self) -> (I, S) {
        (self.encodable_iter, self.separator)
    }
}

impl<EncodableIter, Separator, Encoder> crate::Encodable<Encoder>
    for Separated<EncodableIter, Separator>
where
    EncodableIter: IntoIterator + Clone,
    EncodableIter::Item: crate::Encodable<Encoder, Error = Separator::Error>,
    Separator: crate::Encodable<Encoder>,
    Encoder: crate::BaseEncoder,
{
    type Error = Separator::Error;

    #[inline]
    fn encode(&self, encoder: &mut Encoder) -> Result<(), Self::Error> {
        let mut is_first = true;

        for encodable in self.encodable_iter.clone() {
            if is_first {
                is_first = false;
            } else {
                self.separator.encode(encoder)?;
            }
            encodable.encode(encoder)?;
        }
        Ok(())
    }
}
