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

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;
    use crate::Encodable;

    const BUF_SIZE: usize = 32;

    #[rstest]
    #[case::multiple_elements(&[b"a" as &[u8], b"b", b"c"], b"a, b, c")]
    #[case::single_element(&[b"only" as &[u8]], b"only")]
    #[case::empty(&[], b"")]
    fn assert_that_separated_encodes_correctly(#[case] items: &[&[u8]], #[case] expected: &[u8]) {
        let sep = Separated::new(items, b", " as &[u8]);
        let mut buf = [0u8; BUF_SIZE];
        let mut encoder = &mut buf as &mut [u8];
        sep.encode(&mut encoder).unwrap();
        let written = BUF_SIZE - encoder.len();
        assert_eq!(&buf[..written], expected);
    }

    #[test]
    fn assert_that_separated_into_inner_returns_values() {
        let items = [1u8, 2u8];
        let sep = Separated::new(items, 0u8);
        let (arr, s) = sep.into_inner();
        assert_eq!(arr, [1u8, 2u8]);
        assert_eq!(s, 0u8);
    }
}
