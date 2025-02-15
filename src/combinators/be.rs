use core::num::NonZero;

use crate::Encodable;
use crate::Encoder;

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
pub struct BE<E> {
    num: E,
}

impl<E> BE<E> {
    /// Creates a new [`BE`] combinator.
    #[inline]
    pub const fn new(num: E) -> Self {
        Self { num }
    }
}

macro_rules! impl_encodeable_be_for_num {
    ($($T:ty)*) => {
        $(
            impl From<$T> for BE<$T> {
                fn from(num: $T) -> Self {
                    Self { num }
                }
            }
            impl From<BE<$T>> for $T {
                fn from(be: BE<$T>) -> Self {
                    be.num
                }
            }
            impl<E: Encoder> Encodable<E> for BE<$T>
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
                fn from(num: NonZero<$T>) -> Self {
                    Self { num }
                }
            }
            impl From<BE<NonZero<$T>>> for NonZero<$T> {
                fn from(be: BE<NonZero<$T>>) -> Self {
                    be.num
                }
            }
            impl<E: Encoder> Encodable<E> for BE<NonZero<$T>>
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
