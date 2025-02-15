/// Encodes a length prefixed value ([TLV](https://en.wikipedia.org/wiki/Type–length–value)).
///
/// # Examples
///
/// ```rust
/// use encode::Encodable;
/// use encode::combinators::LengthPrefix;
/// use encode::combinators::FromError;
/// use core::num::TryFromIntError;
///
/// let mut buf = Vec::new();
/// LengthPrefix::<_, u8, TryFromIntError>::new("hello").encode(&mut buf).unwrap();
/// assert_eq!(&buf, b"\x05hello", "Using a single byte to indicate the length of the string");
/// ```
#[doc(alias("length", "prefix", "TLV"))]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
#[repr(transparent)]
pub struct LengthPrefix<Encodable, Length, Error> {
    encodable: Encodable,
    phantom: core::marker::PhantomData<(Length, Error)>,
}

impl<Encodable, Length, Error> LengthPrefix<Encodable, Length, Error> {
    /// Creates a new TLV combinator.
    #[inline]
    pub const fn new(encodable: Encodable) -> Self {
        Self {
            encodable,
            phantom: core::marker::PhantomData,
        }
    }
}

impl<Encodable, Length, Encoder, Error> crate::Encodable<Encoder>
    for LengthPrefix<Encodable, Length, Error>
where
    Encoder: crate::Encoder,
    Encodable: crate::Encodable<Encoder> + crate::EncodableSize,
    Length: crate::Encodable<Encoder> + TryFrom<usize>,
    Error: From<<Length as crate::Encodable<Encoder>>::Error>
        + From<<Length as TryFrom<usize>>::Error>
        + From<<Encodable as crate::Encodable<Encoder>>::Error>
        + From<<Encodable as crate::Encodable<crate::encoders::SizeEncoder>>::Error>
        + From<Encoder::Error>,
{
    type Error = Error;

    #[inline]
    fn encode(&self, encoder: &mut Encoder) -> Result<(), Self::Error> {
        let len = self.encodable.encoded_size()?;
        let len_encoder = Length::try_from(len)?;

        len_encoder.encode(encoder)?;
        self.encodable.encode(encoder)?;
        Ok(())
    }
}
