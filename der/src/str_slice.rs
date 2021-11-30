//! Common handling for types backed by `str` slices with enforcement of a
//! library-level length limitation i.e. `Length::max()`.

use crate::{Length, Result};
use core::{str};

/// String slice newtype which respects the [`Length::max`] limit.
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
pub(crate) struct StrSlice<'a> {
    /// Inner value
    pub(crate) inner: &'a str,

    /// Precomputed `Length` (avoids possible panicking conversions)
    pub(crate) length: Length,
}

impl<'a> StrSlice<'a> {
    /// Create a new [`StrSlice`], ensuring that the byte representation of
    /// the provided `str` value is shorter than `Length::max()`.
    pub fn new(_s: &'a str) -> Result<Self> {
        unimplemented!()
    }

    /// Parse a [`StrSlice`] from UTF-8 encoded bytes.
    pub fn from_bytes(_bytes: &'a [u8]) -> Result<Self> {
        unimplemented!()
    }

    /// Borrow the inner `str`
    pub fn as_str(&self) -> &'a str {
        unimplemented!()
    }

    /// Borrow the inner byte slice
    pub fn as_bytes(&self) -> &'a [u8] {
        unimplemented!()
    }

    /// Get the [`Length`] of this [`StrSlice`]
    pub fn len(self) -> Length {
        unimplemented!()
    }

    /// Is this [`StrSlice`] empty?
    pub fn is_empty(self) -> bool {
        unimplemented!()
    }
}

impl AsRef<str> for StrSlice<'_> {
    fn as_ref(&self) -> &str {
        unimplemented!()
    }
}

impl AsRef<[u8]> for StrSlice<'_> {
    fn as_ref(&self) -> &[u8] {
        unimplemented!()
    }
}
