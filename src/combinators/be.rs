use core::borrow::Borrow;
use core::num::NonZero;
use core::ops::Deref;

use crate::ByteEncoder;
use crate::Encodable;

/// Encodes a number in big-endian order.
///
/// # Examples
///
/// ```rust
/// # #[cfg(feature = "alloc")] {
/// use encode::Encodable;
/// use encode::combinators::BE;
///
/// let mut buf = Vec::new();
/// BE::new(1u16).encode(&mut buf).unwrap();
/// assert_eq!(&buf, &[0, 1], "Encoding a u16 in big-endian order means the most significant byte comes first");
/// # }
/// ```
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
#[repr(transparent)]
pub struct BE<E> {
    num: E,
}

impl<E> BE<E> {
    /// Creates a new [`BE`] combinator.
    #[inline]
    #[must_use]
    pub const fn new(num: E) -> Self {
        Self { num }
    }
    /// Consumes the [`BE`] combinator and returns the inner value.
    #[inline]
    #[must_use]
    pub fn into_inner(self) -> E {
        self.num
    }
}

impl<E> AsRef<E> for BE<E> {
    #[inline]
    fn as_ref(&self) -> &E {
        &self.num
    }
}
impl<E> Borrow<E> for BE<E> {
    #[inline]
    fn borrow(&self) -> &E {
        &self.num
    }
}
impl<E> Deref for BE<E> {
    type Target = E;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.num
    }
}

macro_rules! impl_encodeable_be_for_num {
    ($($T:ty)*) => {
        $(
            impl From<$T> for BE<$T> {
                #[inline]
                fn from(num: $T) -> Self {
                    Self { num }
                }
            }
            impl From<BE<$T>> for $T {
                #[inline]
                fn from(be: BE<$T>) -> Self {
                    be.num
                }
            }
            impl<E: ByteEncoder> Encodable<E> for BE<$T>
            {
                type Error = E::Error;

                #[inline]
                fn encode(&self, encoder:&mut E) -> Result<(), Self::Error> {
                    encoder.put_slice(&self.num.to_be_bytes())
                }
            }
        )*

    }
}

macro_rules! impl_encodeable_be_for_nonzero_num {
    ($($T:ty)*) => {
        $(
            impl From<NonZero<$T>> for BE<NonZero<$T>> {
                #[inline]
                fn from(num: NonZero<$T>) -> Self {
                    Self { num }
                }
            }
            impl From<BE<NonZero<$T>>> for NonZero<$T> {
                #[inline]
                fn from(be: BE<NonZero<$T>>) -> Self {
                    be.num
                }
            }
            impl<E: ByteEncoder> Encodable<E> for BE<NonZero<$T>>
            {
                type Error = E::Error;

                #[inline]
                fn encode(&self, encoder:&mut E) -> Result<(), Self::Error> {
                    encoder.put_slice(&self.num.get().to_be_bytes())
                }
            }
        )*

    }
}

macro_rules! impl_try_from_be_for_num {
    ($($T:ty)*) => {
        $(
            impl TryFrom<usize> for BE<$T> {
                type Error = core::num::TryFromIntError;

                #[inline]
                fn try_from(value: usize) -> Result<Self, Self::Error> {
                    <$T>::try_from(value).map(Self::new)
                }
            }
        )*
    };
}

impl_try_from_be_for_num!(u8 u16 u32 u64 u128 i8 i16 i32 i64 i128);
impl_encodeable_be_for_num!(u8 u16 u32 u64 u128 i8 i16 i32 i64 i128 f32 f64);
impl_encodeable_be_for_nonzero_num!(u8 u16 u32 u64 u128 i8 i16 i32 i64 i128);

#[cfg(test)]
mod tests {
    use core::borrow::Borrow;
    use core::num::NonZero;

    use super::*;
    use crate::Encodable;

    const BUF_SIZE: usize = 32;

    #[test]
    fn assert_that_u8_can_be_encoded_in_big_endian() {
        let expected = [0xABu8];
        let mut buf = [0u8; BUF_SIZE];
        let mut encoder = &mut buf as &mut [u8];
        BE::new(0xABu8).encode(&mut encoder).unwrap();
        let written = BUF_SIZE - encoder.len();
        assert_eq!(&buf[..written], &expected);
    }

    #[test]
    fn assert_that_u16_can_be_encoded_in_big_endian() {
        let expected = [0x00u8, 0x01u8];
        let mut buf = [0u8; BUF_SIZE];
        let mut encoder = &mut buf as &mut [u8];
        BE::new(1u16).encode(&mut encoder).unwrap();
        let written = BUF_SIZE - encoder.len();
        assert_eq!(&buf[..written], &expected);
    }

    #[test]
    fn assert_that_f32_can_be_encoded_in_big_endian() {
        let val = 1.0f32;
        let expected = val.to_be_bytes();
        let mut buf = [0u8; BUF_SIZE];
        let mut encoder = &mut buf as &mut [u8];
        BE::new(val).encode(&mut encoder).unwrap();
        let written = BUF_SIZE - encoder.len();
        assert_eq!(&buf[..written], &expected);
    }

    #[test]
    fn assert_that_be_into_inner_returns_the_value() {
        let be = BE::new(42u32);
        assert_eq!(be.into_inner(), 42u32);
    }

    #[test]
    fn assert_that_be_deref_works() {
        let be = BE::new(42u32);
        assert_eq!(*be, 42u32);
    }

    #[test]
    fn assert_that_be_as_ref_works() {
        let be = BE::new(42u32);
        assert_eq!(be.as_ref(), &42u32);
    }

    #[test]
    fn assert_that_be_borrow_works() {
        let be = BE::new(42u32);
        let borrowed: &u32 = be.borrow();
        assert_eq!(*borrowed, 42u32);
    }

    #[test]
    fn assert_that_from_primitive_into_be_works() {
        let be: BE<u16> = 42u16.into();
        assert_eq!(be.into_inner(), 42u16);
    }

    #[test]
    fn assert_that_primitive_from_be_works() {
        let be = BE::new(42u16);
        let val: u16 = be.into();
        assert_eq!(val, 42u16);
    }

    #[test]
    fn assert_that_try_from_usize_succeeds() {
        let be = BE::<u8>::try_from(5usize).unwrap();
        assert_eq!(be.into_inner(), 5u8);
    }

    #[test]
    fn assert_that_try_from_usize_fails_on_overflow() {
        assert!(BE::<u8>::try_from(256usize).is_err());
    }

    #[test]
    fn assert_that_nonzero_u16_can_be_encoded_in_big_endian() {
        let expected = [0x00u8, 0x01u8];
        let mut buf = [0u8; BUF_SIZE];
        let mut encoder = &mut buf as &mut [u8];
        BE::new(NonZero::<u16>::new(1).unwrap())
            .encode(&mut encoder)
            .unwrap();
        let written = BUF_SIZE - encoder.len();
        assert_eq!(&buf[..written], &expected);
    }

    #[test]
    fn assert_that_from_nonzero_into_be_works() {
        let nz = NonZero::<u16>::new(42).unwrap();
        let be: BE<NonZero<u16>> = nz.into();
        assert_eq!(be.into_inner().get(), 42u16);
    }

    #[test]
    fn assert_that_nonzero_from_be_works() {
        let nz = NonZero::<u16>::new(42).unwrap();
        let be = BE::new(nz);
        let val: NonZero<u16> = be.into();
        assert_eq!(val.get(), 42u16);
    }
}
