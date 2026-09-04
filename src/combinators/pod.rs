use core::borrow::Borrow;

/// Encodes any [`bytemuck::Pod`] value as its raw byte representation.
///
/// This is useful for encoding plain-old-data types, such as `#[repr(C)]`
/// structs, without having to manually implement [`Encodable`](crate::Encodable)
/// for them.
///
/// Must be wrapped in [`LE`](super::LE) or [`BE`](super::BE), and only the one
/// matching the target's byte order exists as a raw representation cannot be
/// meaningfully reversed.
///
/// # Examples
///
/// ```rust
/// # #[cfg(all(feature = "alloc", target_endian = "little"))] {
/// use encode::Encodable;
/// use encode::combinators::{LE, Pod};
///
/// let mut buf = Vec::new();
/// LE::new(Pod::new(42u32)).encode(&mut buf).unwrap();
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Encodable;

    const BUF_SIZE: usize = 32;

    #[cfg(target_endian = "little")]
    #[test]
    fn assert_that_a_le_wrapped_pod_value_is_encoded_in_little_endian_order() {
        use crate::combinators::LE;

        let expected = 0x0102_0304_u32.to_le_bytes();
        let mut buf = [0u8; BUF_SIZE];
        let mut encoder = &mut buf as &mut [u8];
        LE::new(Pod::new(0x0102_0304_u32))
            .encode(&mut encoder)
            .unwrap();
        let written = BUF_SIZE - encoder.len();
        assert_eq!(&buf[..written], &expected);
    }

    #[cfg(target_endian = "big")]
    #[test]
    fn assert_that_a_be_wrapped_pod_value_is_encoded_in_big_endian_order() {
        use crate::combinators::BE;

        let expected = 0x0102_0304_u32.to_be_bytes();
        let mut buf = [0u8; BUF_SIZE];
        let mut encoder = &mut buf as &mut [u8];
        BE::new(Pod::new(0x0102_0304_u32))
            .encode(&mut encoder)
            .unwrap();
        let written = BUF_SIZE - encoder.len();
        assert_eq!(&buf[..written], &expected);
    }

    #[test]
    fn assert_that_pod_into_inner_returns_the_value() {
        let pod = Pod::new(42u32);
        assert_eq!(pod.into_inner(), 42u32);
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
