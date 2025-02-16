use core::borrow::Borrow;
use core::fmt::Debug;
use core::marker::PhantomData;
use core::ops::Deref;

/// Encodes a length prefixed value ([TLV](https://en.wikipedia.org/wiki/Type–length–value)).
///
/// # Examples
///
/// ```rust
/// # #[cfg(feature = "alloc")] {
/// use encode::Encodable;
/// use encode::combinators::LengthPrefix;
/// use encode::combinators::FromError;
/// use core::num::TryFromIntError;
///
/// let mut buf = Vec::new();
/// LengthPrefix::<_, u8, TryFromIntError>::new("hello").encode(&mut buf).unwrap();
/// assert_eq!(&buf, b"\x05hello", "Using a single byte to indicate the length of the string");
/// # }
/// ```
#[doc(alias("length", "prefix", "TLV"))]
#[repr(transparent)]
pub struct LengthPrefix<Encodable, Length, Error> {
    encodable: Encodable,
    phantom: PhantomData<(Length, Error)>,
}

impl<Encodable, Length, Error> LengthPrefix<Encodable, Length, Error> {
    /// Creates a new TLV combinator.
    #[inline]
    #[must_use]
    pub const fn new(encodable: Encodable) -> Self {
        Self {
            encodable,
            phantom: PhantomData,
        }
    }
    /// Consumes the combinator and returns the inner encodable.
    #[inline]
    #[must_use]
    pub fn into_inner(self) -> Encodable {
        self.encodable
    }
}

impl<Encodable, Length, Error> From<Encodable> for LengthPrefix<Encodable, Length, Error> {
    fn from(value: Encodable) -> Self {
        Self::new(value)
    }
}

impl<Encodable, Length, Error> AsRef<Encodable> for LengthPrefix<Encodable, Length, Error> {
    #[inline]
    fn as_ref(&self) -> &Encodable {
        &self.encodable
    }
}

impl<Encodable, Length, Error> Deref for LengthPrefix<Encodable, Length, Error> {
    type Target = Encodable;
    fn deref(&self) -> &Self::Target {
        self.as_ref()
    }
}

impl<Encodable, Length, Error> Borrow<Encodable> for LengthPrefix<Encodable, Length, Error> {
    fn borrow(&self) -> &Encodable {
        &self.encodable
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

// Manual trait implementations because the derive macro does not support
// phantom data fields.
impl<Encodable, Length, Error> Debug for LengthPrefix<Encodable, Length, Error>
where
    Encodable: Debug,
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("LengthPrefix")
            .field("encodable", &self.encodable)
            .finish()
    }
}
impl<Encodable, Length, Error> Clone for LengthPrefix<Encodable, Length, Error>
where
    Encodable: Clone,
{
    fn clone(&self) -> Self {
        Self {
            encodable: self.encodable.clone(),
            phantom: PhantomData,
        }
    }
}
impl<Encodable, Length, Error> Copy for LengthPrefix<Encodable, Length, Error> where Encodable: Copy {}
impl<Encodable, Length, Error> Default for LengthPrefix<Encodable, Length, Error>
where
    Encodable: Default,
{
    fn default() -> Self {
        Self {
            encodable: Default::default(),
            phantom: PhantomData,
        }
    }
}
impl<Encodable, Length, Error> PartialEq for LengthPrefix<Encodable, Length, Error>
where
    Encodable: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.encodable == other.encodable && self.phantom == other.phantom
    }
}
impl<Encodable, Length, Error> Eq for LengthPrefix<Encodable, Length, Error> where Encodable: Eq {}
impl<Encodable, Length, Error> PartialOrd for LengthPrefix<Encodable, Length, Error>
where
    Encodable: PartialOrd,
{
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        self.encodable.partial_cmp(&other.encodable)
    }
}
impl<Encodable, Length, Error> Ord for LengthPrefix<Encodable, Length, Error>
where
    Encodable: Ord,
{
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        self.encodable.cmp(&other.encodable)
    }
}
