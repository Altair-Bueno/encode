/// Encodes a string with a prefix prepended to each line, where lines are
/// determined by splitting on a given [`char`] pattern.
///
/// # Example
///
/// ```
/// # #[cfg(feature = "alloc")] {
/// use encode::Encodable;
/// use encode::combinators::Indent;
///
/// let mut buf = Vec::new();
/// Indent::new("i love\nducks", '\n', "   ").encode(&mut buf).unwrap();
/// assert_eq!(buf, b"   i love\n   ducks");
/// # }
/// ```
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Indent<'a, S> {
    content: &'a str,
    pattern: char,
    indent: S,
}

impl<'a, S> Indent<'a, S> {
    /// Creates a new [`Indent`] combinator.
    #[inline]
    pub const fn new(content: &'a str, pattern: char, indent: S) -> Self {
        Self {
            content,
            pattern,
            indent,
        }
    }
    /// Consumes the [`Indent`] combinator and returns the inner values.
    #[inline]
    #[must_use]
    pub fn into_inner(self) -> (&'a str, char, S) {
        (self.content, self.pattern, self.indent)
    }
}

impl<'a, S, Encoder> crate::Encodable<Encoder> for Indent<'a, S>
where
    S: crate::Encodable<Encoder>,
    S::Error: From<Encoder::Error>,
    Encoder: crate::StrEncoder,
{
    type Error = S::Error;

    #[inline]
    fn encode(&self, encoder: &mut Encoder) -> Result<(), Self::Error> {
        let mut first = true;
        for part in self.content.split(self.pattern) {
            if first {
                first = false;
            } else {
                self.pattern.encode(encoder).map_err(Into::into)?;
            }
            self.indent.encode(encoder)?;
            part.encode(encoder).map_err(Into::into)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Encodable;

    const BUF_SIZE: usize = 128;

    #[test]
    fn assert_that_indent_prepends_prefix_to_each_line() {
        let expected = b"   i love\n   ducks";
        let encodable = Indent::new("i love\nducks", '\n', "   ");

        let mut buf = [0u8; BUF_SIZE];
        let mut encoder = &mut buf as &mut [u8];
        encodable.encode(&mut encoder).unwrap();
        let written = BUF_SIZE - encoder.len();
        let result = &buf[..written];

        assert_eq!(expected, result);
    }

    #[test]
    fn assert_that_indent_works_on_single_line() {
        let expected = b"   hello";
        let encodable = Indent::new("hello", '\n', "   ");

        let mut buf = [0u8; BUF_SIZE];
        let mut encoder = &mut buf as &mut [u8];
        encodable.encode(&mut encoder).unwrap();
        let written = BUF_SIZE - encoder.len();
        let result = &buf[..written];

        assert_eq!(expected, result);
    }

    #[test]
    fn assert_that_indent_works_with_empty_content() {
        let expected = b"   ";
        let encodable = Indent::new("", '\n', "   ");

        let mut buf = [0u8; BUF_SIZE];
        let mut encoder = &mut buf as &mut [u8];
        encodable.encode(&mut encoder).unwrap();
        let written = BUF_SIZE - encoder.len();
        let result = &buf[..written];

        assert_eq!(expected, result);
    }

    #[test]
    fn assert_that_indent_handles_trailing_newline() {
        let expected = b"   hello\n   ";
        let encodable = Indent::new("hello\n", '\n', "   ");

        let mut buf = [0u8; BUF_SIZE];
        let mut encoder = &mut buf as &mut [u8];
        encodable.encode(&mut encoder).unwrap();
        let written = BUF_SIZE - encoder.len();
        let result = &buf[..written];

        assert_eq!(expected, result);
    }

    #[test]
    fn assert_that_indent_works_with_carriage_return() {
        let expected = b"\t line one\r\t line two";
        let encodable = Indent::new("line one\rline two", '\r', "\t ");

        let mut buf = [0u8; BUF_SIZE];
        let mut encoder = &mut buf as &mut [u8];
        encodable.encode(&mut encoder).unwrap();
        let written = BUF_SIZE - encoder.len();
        let result = &buf[..written];

        assert_eq!(expected, result);
    }
}
