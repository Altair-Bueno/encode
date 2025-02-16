use core::borrow::Borrow;
use core::num::NonZero;
use core::ops::Deref;

use crate::Encodable;
use crate::Encoder;

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
                fn from(num: $T) -> Self {
                    Self { num }
                }
            }
            impl From<LE<$T>> for $T {
                fn from(le: LE<$T>) -> Self {
                    le.num
                }
            }
            impl<E: Encoder> Encodable<E> for LE<$T>
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
                fn from(num: NonZero<$T>) -> Self {
                    Self { num }
                }
            }
            impl From<LE<NonZero<$T>>> for NonZero<$T> {
                fn from(le: LE<NonZero<$T>>) -> Self {
                    le.num
                }
            }
            impl<E: Encoder> Encodable<E> for LE<NonZero<$T>>
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
