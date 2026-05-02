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
    #[inline]
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
    #[inline]
    fn deref(&self) -> &Self::Target {
        self.as_ref()
    }
}

impl<Encodable, Length, Error> Borrow<Encodable> for LengthPrefix<Encodable, Length, Error> {
    #[inline]
    fn borrow(&self) -> &Encodable {
        &self.encodable
    }
}

impl<Encodable, Length, Encoder, Error> crate::Encodable<Encoder>
    for LengthPrefix<Encodable, Length, Error>
where
    Encoder: crate::BaseEncoder,
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
    #[inline]
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
    #[inline]
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
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.encodable == other.encodable && self.phantom == other.phantom
    }
}
impl<Encodable, Length, Error> Eq for LengthPrefix<Encodable, Length, Error> where Encodable: Eq {}
impl<Encodable, Length, Error> PartialOrd for LengthPrefix<Encodable, Length, Error>
where
    Encodable: PartialOrd,
{
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        self.encodable.partial_cmp(&other.encodable)
    }
}
impl<Encodable, Length, Error> Ord for LengthPrefix<Encodable, Length, Error>
where
    Encodable: Ord,
{
    #[inline]
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        self.encodable.cmp(&other.encodable)
    }
}

#[cfg(test)]
mod tests {
    use core::borrow::Borrow;
    use core::num::TryFromIntError;

    use super::*;
    use crate::Encodable;

    const BUF_SIZE: usize = 32;

    #[cfg(feature = "alloc")]
    #[test]
    fn assert_that_length_prefix_encodes_length_and_value() {
        let mut buf = alloc::vec::Vec::new();
        LengthPrefix::<_, u8, TryFromIntError>::new("hello")
            .encode(&mut buf)
            .unwrap();
        assert_eq!(&buf, b"\x05hello");
    }

    #[test]
    fn assert_that_length_prefix_into_inner_returns_value() {
        let lp = LengthPrefix::<u8, u8, TryFromIntError>::new(42u8);
        assert_eq!(lp.into_inner(), 42u8);
    }

    #[test]
    fn assert_that_length_prefix_from_works() {
        let lp: LengthPrefix<u8, u8, TryFromIntError> = 42u8.into();
        assert_eq!(*lp, 42u8);
    }

    #[test]
    fn assert_that_length_prefix_as_ref_works() {
        let lp = LengthPrefix::<u8, u8, TryFromIntError>::new(42u8);
        assert_eq!(lp.as_ref(), &42u8);
    }

    #[test]
    fn assert_that_length_prefix_deref_works() {
        let lp = LengthPrefix::<u8, u8, TryFromIntError>::new(42u8);
        assert_eq!(*lp, 42u8);
    }

    #[test]
    fn assert_that_length_prefix_borrow_works() {
        let lp = LengthPrefix::<u8, u8, TryFromIntError>::new(42u8);
        let borrowed: &u8 = lp.borrow();
        assert_eq!(*borrowed, 42u8);
    }

    #[test]
    fn assert_that_length_prefix_clone_works() {
        let lp = LengthPrefix::<u8, u8, TryFromIntError>::new(42u8);
        let clone = lp.clone();
        assert_eq!(*lp, *clone);
    }

    #[test]
    fn assert_that_length_prefix_default_works() {
        let lp = LengthPrefix::<u8, u8, TryFromIntError>::default();
        assert_eq!(*lp, 0u8);
    }

    #[test]
    fn assert_that_length_prefix_eq_works() {
        let lp1 = LengthPrefix::<u8, u8, TryFromIntError>::new(42u8);
        let lp2 = LengthPrefix::<u8, u8, TryFromIntError>::new(42u8);
        assert_eq!(lp1, lp2);
    }

    #[test]
    fn assert_that_length_prefix_ord_works() {
        let lp1 = LengthPrefix::<u8, u8, TryFromIntError>::new(1u8);
        let lp2 = LengthPrefix::<u8, u8, TryFromIntError>::new(2u8);
        assert!(lp1 < lp2);
        assert_eq!(lp1.cmp(&lp2), core::cmp::Ordering::Less);
    }

    #[cfg(feature = "alloc")]
    #[test]
    fn assert_that_length_prefix_debug_works() {
        let lp = LengthPrefix::<u8, u8, TryFromIntError>::new(42u8);
        let debug_str = alloc::format!("{lp:?}");
        assert!(debug_str.contains("42"));
    }

    #[test]
    fn assert_that_length_prefix_encodes_size_correctly() {
        let lp = LengthPrefix::<_, u8, TryFromIntError>::new("hello");
        let mut encoder = crate::encoders::SizeEncoder::new();
        lp.encode(&mut encoder).unwrap();
        assert_eq!(
            encoder.size(),
            6,
            "Expected 6 bytes: 1 for length prefix + 5 for \"hello\""
        );
    }
}
