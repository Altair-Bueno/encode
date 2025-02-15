use core::fmt::Debug;
use core::ops::Deref;

use crate::Encodable;
use crate::Encoder;

/// A combinator that encodes a sequence of flags as a single byte.
///
/// # Examples
///
/// ```
/// # #[cfg(feature = "alloc")] {
/// use encode::Encodable;
/// use encode::combinators::Flags;
///
/// let mut buf = Vec::new();
/// Flags::new([false,false,false,false,false,false,false, false]).encode(&mut buf).unwrap();
/// assert_eq!(&buf, &[0]);
/// # }
/// ```
///
/// ```
/// # #[cfg(feature = "alloc")] {
/// use encode::Encodable;
/// use encode::combinators::Flags;
///
/// let mut buf = Vec::new();
/// Flags::new([true,false,false,true,false,false,false, false]).encode(&mut buf).unwrap();
/// assert_eq!(&buf, &[0b1001_0000]);
/// # }
/// ```
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[repr(transparent)]
pub struct Flags([bool; 8]);

impl Flags {
    /// Creates a new [`Flags`] combinator.
    #[inline]
    #[must_use]
    pub fn new(flags: [bool; 8]) -> Self {
        Self(flags)
    }
}

impl Deref for Flags {
    type Target = [bool; 8];

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl AsRef<[bool; 8]> for Flags {
    #[inline]
    fn as_ref(&self) -> &[bool; 8] {
        &self.0
    }
}

impl Debug for Flags {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "Flags({:08b})", u8::from(*self))
    }
}

impl From<u8> for Flags {
    #[inline]
    fn from(value: u8) -> Self {
        let mut slice = [false; 8];
        slice.iter_mut().enumerate().rev().for_each(|(i, v)| {
            *v = (value & (1 << i)) != 0;
        });
        Self(slice)
    }
}

impl From<Flags> for u8 {
    #[inline]
    fn from(value: Flags) -> Self {
        value
            .0
            .into_iter()
            .rev()
            .enumerate()
            .filter_map(|(i, v)| v.then_some(1 << i))
            .fold(0u8, core::ops::BitOr::bitor)
    }
}

impl<E: Encoder> Encodable<E> for Flags {
    type Error = E::Error;

    #[inline]
    fn encode(&self, encoder: &mut E) -> Result<(), Self::Error> {
        u8::from(*self).encode(encoder)
    }
}
