use core::borrow::Borrow;
use core::ops::Deref;

use crate::ByteEncoder;
use crate::Encodable;

/// Encodes any [`bytemuck::Pod`] value as its raw byte representation.
///
/// This is useful for encoding plain-old-data types, such as `#[repr(C)]`
/// structs, without having to manually implement [`Encodable`] for them.
///
/// Note that [`Pod`] encodes the value using the machine's native byte order.
/// Wrap the value with [`LE`](super::LE) or [`BE`](super::BE) beforehand if a
/// specific byte order is required.
///
/// # Examples
///
/// ```rust
/// # #[cfg(feature = "alloc")] {
/// use encode::Encodable;
/// use encode::combinators::Pod;
///
/// let mut buf = Vec::new();
/// Pod::new(42u32).encode(&mut buf).unwrap();
/// assert_eq!(buf.len(), 4);
/// # }
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct Pod<T> {
    value: T,
}

impl<T> Pod<T> {
    /// Creates a new [`Pod`] combinator.
    #[inline]
    #[must_use]
    pub const fn new(value: T) -> Self {
        Self { value }
    }
    /// Consumes the [`Pod`] combinator and returns the inner value.
    #[inline]
    #[must_use]
    pub fn into_inner(self) -> T {
        self.value
    }
}

impl<T> From<T> for Pod<T> {
    #[inline]
    fn from(value: T) -> Self {
        Self::new(value)
    }
}

impl<T> AsRef<T> for Pod<T> {
    #[inline]
    fn as_ref(&self) -> &T {
        &self.value
    }
}
impl<T> Borrow<T> for Pod<T> {
    #[inline]
    fn borrow(&self) -> &T {
        &self.value
    }
}
impl<T> Deref for Pod<T> {
    type Target = T;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl<E, T> Encodable<E> for Pod<T>
where
    E: ByteEncoder,
    T: bytemuck::Pod,
{
    type Error = E::Error;

    #[inline]
    fn encode(&self, encoder: &mut E) -> Result<(), Self::Error> {
        bytemuck::bytes_of(&self.value).encode(encoder)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const BUF_SIZE: usize = 32;

    #[test]
    fn assert_that_a_pod_value_can_be_encoded() {
        let expected = 0x2A_u32.to_ne_bytes();
        let mut buf = [0u8; BUF_SIZE];
        let mut encoder = &mut buf as &mut [u8];
        Pod::new(0x2A_u32).encode(&mut encoder).unwrap();
        let written = BUF_SIZE - encoder.len();
        assert_eq!(&buf[..written], &expected);
    }

    #[test]
    fn assert_that_pod_into_inner_returns_the_value() {
        let pod = Pod::new(42u32);
        assert_eq!(pod.into_inner(), 42u32);
    }

    #[test]
    fn assert_that_pod_deref_works() {
        let pod = Pod::new(42u32);
        assert_eq!(*pod, 42u32);
    }

    #[test]
    fn assert_that_pod_as_ref_works() {
        let pod = Pod::new(42u32);
        assert_eq!(pod.as_ref(), &42u32);
    }

    #[test]
    fn assert_that_pod_borrow_works() {
        let pod = Pod::new(42u32);
        let borrowed: &u32 = pod.borrow();
        assert_eq!(*borrowed, 42u32);
    }

    #[test]
    fn assert_that_from_value_into_pod_works() {
        let pod: Pod<u32> = 42u32.into();
        assert_eq!(pod.into_inner(), 42u32);
    }
}
