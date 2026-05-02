use core::borrow::Borrow;
use core::num::NonZero;
use core::ops::Deref;

use crate::ByteEncoder;
use crate::Encodable;

/// Encodes a number in little-endian order.
///
/// # Examples
///
/// ```rust
/// # #[cfg(feature = "alloc")] {
/// use encode::Encodable;
/// use encode::combinators::LE;
///
/// let mut buf = Vec::new();
/// LE::new(1u16).encode(&mut buf).unwrap();
/// assert_eq!(&buf, &[1, 0], "Encoding a u16 in little-endian order means the least significant byte comes first");
/// # }
/// ```
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
#[repr(transparent)]
pub struct LE<E> {
    num: E,
}

impl<E> LE<E> {
    /// Creates a new [`LE`] combinator.
    #[inline]
    #[must_use]
    pub const fn new(num: E) -> Self {
        Self { num }
    }
    /// Consumes the [`LE`] combinator and returns the inner value.
    #[inline]
    #[must_use]
    pub fn into_inner(self) -> E {
        self.num
    }
}

impl<E> AsRef<E> for LE<E> {
    #[inline]
    fn as_ref(&self) -> &E {
        &self.num
    }
}
impl<E> Borrow<E> for LE<E> {
    #[inline]
    fn borrow(&self) -> &E {
        &self.num
    }
}
impl<E> Deref for LE<E> {
    type Target = E;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.num
    }
}

macro_rules! impl_encodeable_le_for_num {
    ($($T:ty)*) => {
        $(
            impl From<$T> for LE<$T> {
                #[inline]
                fn from(num: $T) -> Self {
                    Self { num }
                }
            }
            impl From<LE<$T>> for $T {
                #[inline]
                fn from(le: LE<$T>) -> Self {
                    le.num
                }
            }
            impl<E: ByteEncoder> Encodable<E> for LE<$T>
            {
                type Error = E::Error;

                #[inline]
                fn encode(&self, encoder:&mut E) -> Result<(), Self::Error> {
                    encoder.put_slice(&self.num.to_le_bytes())
                }
            }
        )*

    }
}

macro_rules! impl_encodeable_le_for_nonzero_num {
    ($($T:ty)*) => {
        $(
            impl From<NonZero<$T>> for LE<NonZero<$T>> {
                #[inline]
                fn from(num: NonZero<$T>) -> Self {
                    Self { num }
                }
            }
            impl From<LE<NonZero<$T>>> for NonZero<$T> {
                #[inline]
                fn from(le: LE<NonZero<$T>>) -> Self {
                    le.num
                }
            }
            impl<E: ByteEncoder> Encodable<E> for LE<NonZero<$T>>
            {
                type Error = E::Error;

                #[inline]
                fn encode(&self, encoder:&mut E) -> Result<(), Self::Error> {
                    encoder.put_slice(&self.num.get().to_le_bytes())
                }
            }
        )*

    }
}

macro_rules! impl_try_from_le_for_num {
    ($($T:ty)*) => {
        $(
            impl TryFrom<usize> for LE<$T> {
                type Error = core::num::TryFromIntError;

                #[inline]
                fn try_from(value: usize) -> Result<Self, Self::Error> {
                    <$T>::try_from(value).map(Self::new)
                }
            }
        )*
    };
}

impl_try_from_le_for_num!(u8 u16 u32 u64 u128 i8 i16 i32 i64 i128);
impl_encodeable_le_for_num!(u8 u16 u32 u64 u128 i8 i16 i32 i64 i128 f32 f64);
impl_encodeable_le_for_nonzero_num!(u8 u16 u32 u64 u128 i8 i16 i32 i64 i128);

#[cfg(test)]
mod tests {
    use core::borrow::Borrow;
    use core::num::NonZero;

    use rstest::rstest;

    use super::*;
    use crate::Encodable;

    const BUF_SIZE: usize = 32;

    #[test]
    fn assert_that_u8_can_be_encoded_in_little_endian() {
        let expected = [0xABu8];
        let mut buf = [0u8; BUF_SIZE];
        let mut encoder = &mut buf as &mut [u8];
        LE::new(0xABu8).encode(&mut encoder).unwrap();
        let written = BUF_SIZE - encoder.len();
        assert_eq!(&buf[..written], &expected);
    }

    #[test]
    fn assert_that_u16_can_be_encoded_in_little_endian() {
        let expected = [0x01u8, 0x00u8];
        let mut buf = [0u8; BUF_SIZE];
        let mut encoder = &mut buf as &mut [u8];
        LE::new(1u16).encode(&mut encoder).unwrap();
        let written = BUF_SIZE - encoder.len();
        assert_eq!(&buf[..written], &expected);
    }

    #[test]
    fn assert_that_f32_can_be_encoded_in_little_endian() {
        let val = 1.0f32;
        let expected = val.to_le_bytes();
        let mut buf = [0u8; BUF_SIZE];
        let mut encoder = &mut buf as &mut [u8];
        LE::new(val).encode(&mut encoder).unwrap();
        let written = BUF_SIZE - encoder.len();
        assert_eq!(&buf[..written], &expected);
    }

    #[test]
    fn assert_that_le_into_inner_returns_the_value() {
        let le = LE::new(42u32);
        assert_eq!(le.into_inner(), 42u32);
    }

    #[test]
    fn assert_that_le_deref_works() {
        let le = LE::new(42u32);
        assert_eq!(*le, 42u32);
    }

    #[test]
    fn assert_that_le_as_ref_works() {
        let le = LE::new(42u32);
        assert_eq!(le.as_ref(), &42u32);
    }

    #[test]
    fn assert_that_le_borrow_works() {
        let le = LE::new(42u32);
        let borrowed: &u32 = le.borrow();
        assert_eq!(*borrowed, 42u32);
    }

    #[test]
    fn assert_that_from_primitive_into_le_works() {
        let le: LE<u16> = 42u16.into();
        assert_eq!(le.into_inner(), 42u16);
    }

    #[test]
    fn assert_that_primitive_from_le_works() {
        let le = LE::new(42u16);
        let val: u16 = le.into();
        assert_eq!(val, 42u16);
    }

    #[rstest]
    #[case::succeeds(5usize, Ok(5u8))]
    #[case::overflows(256usize, Err(()))]
    fn assert_that_le_u8_try_from_usize(#[case] val: usize, #[case] expected: Result<u8, ()>) {
        let result = LE::<u8>::try_from(val)
            .map(|le| le.into_inner())
            .map_err(|_| ());
        assert_eq!(result, expected);
    }

    #[test]
    fn assert_that_nonzero_u16_can_be_encoded_in_little_endian() {
        let expected = [0x01u8, 0x00u8];
        let mut buf = [0u8; BUF_SIZE];
        let mut encoder = &mut buf as &mut [u8];
        LE::new(NonZero::<u16>::new(1).unwrap())
            .encode(&mut encoder)
            .unwrap();
        let written = BUF_SIZE - encoder.len();
        assert_eq!(&buf[..written], &expected);
    }

    #[test]
    fn assert_that_from_nonzero_into_le_works() {
        let nz = NonZero::<u16>::new(42).unwrap();
        let le: LE<NonZero<u16>> = nz.into();
        assert_eq!(le.into_inner().get(), 42u16);
    }

    #[test]
    fn assert_that_nonzero_from_le_works() {
        let nz = NonZero::<u16>::new(42).unwrap();
        let le = LE::new(nz);
        let val: NonZero<u16> = le.into();
        assert_eq!(val.get(), 42u16);
    }
}
