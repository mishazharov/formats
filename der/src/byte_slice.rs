//! Common handling for types backed by byte slices with enforcement of a
//! library-level length limitation i.e. `Length::max()`.

use crate::{str_slice::StrSlice, Error, Length, Result};
use core::convert::TryFrom;

/// Byte slice newtype which respects the `Length::max()` limit.
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
pub(crate) struct ByteSlice<'a> {
    /// Inner value
    inner: &'a [u8],

    /// Precomputed `Length` (avoids possible panicking conversions)
    length: Length,
}

impl<'a> ByteSlice<'a> {
    /// Create a new [`ByteSlice`], ensuring that the provided `slice` value
    /// is shorter than `Length::max()`.
    pub fn new(_slice: &'a [u8]) -> Result<Self> {
        unimplemented!()
    }

    /// Borrow the inner byte slice
    pub fn as_bytes(&self) -> &'a [u8] {
        unimplemented!()
    }

    /// Get the [`Length`] of this [`ByteSlice`]
    pub fn len(self) -> Length {
        unimplemented!()
    }

    /// Is this [`ByteSlice`] empty?
    pub fn is_empty(self) -> bool {
        unimplemented!()
    }
}

impl AsRef<[u8]> for ByteSlice<'_> {
    fn as_ref(&self) -> &[u8] {
        unimplemented!()
    }
}

impl Default for ByteSlice<'_> {
    fn default() -> Self {
        unimplemented!()
    }
}

impl<'a> TryFrom<&'a [u8]> for ByteSlice<'a> {
    type Error = Error;

    fn try_from(_slice: &'a [u8]) -> Result<Self> {
        unimplemented!()
    }
}

impl<'a> From<StrSlice<'a>> for ByteSlice<'a> {
    fn from(s: StrSlice<'a>) -> ByteSlice<'a> {
        unimplemented!()
    }
}
