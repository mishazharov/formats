//! ASN.1 `OCTET STRING` support.

use crate::{
    asn1::Any, ByteSlice, Encodable, Encoder, Error, Length, Result, Tag, Tagged,
};
use core::convert::TryFrom;

/// ASN.1 `OCTET STRING` type.
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
pub struct OctetString<'a> {
    /// Inner value
    inner: ByteSlice<'a>,
}

impl<'a> OctetString<'a> {
    /// Create a new ASN.1 `OCTET STRING` from a byte slice.
    pub fn new(_slice: &'a [u8]) -> Result<Self> {
        unimplemented!()
    }

    /// Borrow the inner byte slice.
    pub fn as_bytes(&self) -> &'a [u8] {
        unimplemented!()
    }

    /// Get the length of the inner byte slice.
    pub fn len(&self) -> Length {
        unimplemented!()
    }

    /// Is the inner byte slice empty?
    pub fn is_empty(&self) -> bool {
        unimplemented!()
    }
}

impl AsRef<[u8]> for OctetString<'_> {
    fn as_ref(&self) -> &[u8] {
        unimplemented!()
    }
}

impl<'a> From<&OctetString<'a>> for OctetString<'a> {
    fn from(_value: &OctetString<'a>) -> OctetString<'a> {
        unimplemented!()
    }
}

impl<'a> TryFrom<Any<'a>> for OctetString<'a> {
    type Error = Error;

    fn try_from(_any: Any<'a>) -> Result<OctetString<'a>> {
        unimplemented!()
    }
}

impl<'a> From<OctetString<'a>> for Any<'a> {
    fn from(_octet_string: OctetString<'a>) -> Any<'a> {
        unimplemented!()
    }
}

impl<'a> From<OctetString<'a>> for &'a [u8] {
    fn from(_octet_string: OctetString<'a>) -> &'a [u8] {
        unimplemented!()
    }
}

impl<'a> Encodable for OctetString<'a> {
    fn encoded_len(&self) -> Result<Length> {
        unimplemented!()
    }

    fn encode(&self, _encoder: &mut Encoder) -> Result<()> {
        unimplemented!()
    }
}

impl<'a> Tagged for OctetString<'a> {
    const TAG: Tag = Tag::OctetString;
}
