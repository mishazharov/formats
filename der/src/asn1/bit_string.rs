//! ASN.1 `BIT STRING` support.

use crate::{ByteSlice, Encodable, Encoder, Length, Result, Tag, Tagged};

/// ASN.1 `BIT STRING` type.
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
pub struct BitString<'a> {
    /// Inner value
    pub(crate) inner: ByteSlice<'a>,

    /// Length after encoding (with leading `0`0 byte)
    pub(crate) encoded_len: Length,
}

impl<'a> BitString<'a> {
    /// Create a new ASN.1 `BIT STRING` from a byte slice.
    pub fn new(_bytes: &'a [u8]) -> Result<Self> {
        unimplemented!()
    }

    /// Borrow the inner byte slice.
    pub fn as_bytes(&self) -> &'a [u8] {
        unimplemented!()
    }

    /// Get the length of the inner byte slice (sans leading `0` byte).
    pub fn len(&self) -> Length {
        unimplemented!()
    }

    /// Is the inner byte slice empty?
    pub fn is_empty(&self) -> bool {
        unimplemented!()
    }
}

impl AsRef<[u8]> for BitString<'_> {
    fn as_ref(&self) -> &[u8] {
        unimplemented!()
    }
}

impl<'a> From<&BitString<'a>> for BitString<'a> {
    fn from(_value: &BitString<'a>) -> BitString<'a> {
        unimplemented!()
    }
}

impl<'a> From<BitString<'a>> for &'a [u8] {
    fn from(_bit_string: BitString<'a>) -> &'a [u8] {
        unimplemented!()
    }
}

impl<'a> Encodable for BitString<'a> {
    fn encoded_len(&self) -> Result<Length> {
        unimplemented!()
    }

    fn encode(&self, _encoder: &mut Encoder) -> Result<()> {
        unimplemented!()
    }
}

impl<'a> Tagged for BitString<'a> {
    const TAG: Tag = Tag::BitString;
}
