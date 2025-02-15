/// An error that occurs when the provided buffer has no space left for encoding.
///
/// This error is returned by the [`Encoder`] implementation of mutable slices
/// ([`&mut [u8]`](slice)) when there is no space left in the buffer.
///
/// [`Encoder`]: crate::Encoder
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, thiserror::Error)]
#[error("The provided buffer has no space left for encoding")]
pub struct InsufficientSpace;
