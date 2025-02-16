/// An error that occurs when the provided buffer has no space left for encoding.
///
/// This error is returned by the [`Encoder`] implementation of mutable slices
/// ([`&mut [u8]`](slice)) when there is no space left in the buffer.
///
/// [`Encoder`]: crate::Encoder
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct InsufficientSpace;

impl core::error::Error for InsufficientSpace {}
impl core::fmt::Display for InsufficientSpace {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "The provided buffer has no space left for encoding")
    }
}
