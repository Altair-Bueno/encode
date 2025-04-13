/// An error that occurs when the provided buffer has no space left for
/// encoding.
///
/// This error is returned by encoders with a fixed size buffer that cannot
/// grow, such as [`&mut [u8]`](slice), [`ArrayVec`](arrayvec::ArrayVec) or
/// [`ArrayString`](arrayvec::ArrayString).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct InsufficientSpace;

impl core::error::Error for InsufficientSpace {}
impl core::fmt::Display for InsufficientSpace {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "The provided buffer has no space left for encoding")
    }
}
