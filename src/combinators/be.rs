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
