//! ASN.1 `UTF8String` support.

use crate::{
    asn1::Any, str_slice::StrSlice, Encodable, Encoder, Error, Length, Result, Tag, Tagged,
};
use core::{convert::TryFrom, fmt, str};

/// ASN.1 `UTF8String` type.
///
/// Supports the full UTF-8 encoding.
///
/// Note that the [`Decodable`][`crate::Decodable`] and [`Encodable`] traits
/// are impl'd for Rust's [`str`][`prim@str`] primitive, which decodes/encodes
/// as a [`Utf8String`].
///
/// You are free to use [`str`][`prim@str`] instead of this type, however it's
/// still provided for explicitness in cases where it might be ambiguous with
/// other ASN.1 string encodings such as
/// [`PrintableString`][`crate::asn1::PrintableString`].
#[derive(Copy, Clone, Eq, PartialEq, PartialOrd, Ord)]
pub struct Utf8String<'a> {
    /// Inner value
    inner: StrSlice<'a>,
}

impl<'a> Utf8String<'a> {
    /// Create a new ASN.1 `UTF8String`.
    pub fn new<T>(_input: &'a T) -> Result<Self>
    where
        T: AsRef<[u8]> + ?Sized,
    {
        unimplemented!()
    }

    /// Borrow the string as a `str`.
    pub fn as_str(&self) -> &'a str {
        unimplemented!()
    }

    /// Borrow the string as bytes.
    pub fn as_bytes(&self) -> &'a [u8] {
        unimplemented!()
    }

    /// Get the length of the inner byte slice.
    pub fn len(&self) -> Length {
        unimplemented!()
    }

    /// Is the inner string empty?
    pub fn is_empty(&self) -> bool {
        unimplemented!()
    }
}

impl AsRef<str> for Utf8String<'_> {
    fn as_ref(&self) -> &str {
        unimplemented!()
    }
}

impl AsRef<[u8]> for Utf8String<'_> {
    fn as_ref(&self) -> &[u8] {
        unimplemented!()
    }
}

impl<'a> From<&Utf8String<'a>> for Utf8String<'a> {
    fn from(_value: &Utf8String<'a>) -> Utf8String<'a> {
        unimplemented!()
    }
}

impl<'a> TryFrom<Any<'a>> for Utf8String<'a> {
    type Error = Error;

    fn try_from(_any: Any<'a>) -> Result<Utf8String<'a>> {
        unimplemented!()
    }
}

impl<'a> From<Utf8String<'a>> for Any<'a> {
    fn from(_printable_string: Utf8String<'a>) -> Any<'a> {
        unimplemented!()
    }
}

impl<'a> From<Utf8String<'a>> for &'a [u8] {
    fn from(_utf8_string: Utf8String<'a>) -> &'a [u8] {
        unimplemented!()
    }
}

impl<'a> Encodable for Utf8String<'a> {
    fn encoded_len(&self) -> Result<Length> {
        unimplemented!()
    }

    fn encode(&self, _encoder: &mut Encoder) -> Result<()> {
        unimplemented!()
    }
}

impl<'a> Tagged for Utf8String<'a> {
    const TAG: Tag = Tag::Utf8String;
}

impl<'a> fmt::Display for Utf8String<'a> {
    fn fmt(&self, _f: &mut fmt::Formatter<'_>) -> fmt::Result {
        unimplemented!()
    }
}

impl<'a> fmt::Debug for Utf8String<'a> {
    fn fmt(&self, _f: &mut fmt::Formatter<'_>) -> fmt::Result {
        unimplemented!()
    }
}

impl<'a> TryFrom<Any<'a>> for &'a str {
    type Error = Error;

    fn try_from(_any: Any<'a>) -> Result<&'a str> {
        unimplemented!()
    }
}

impl Encodable for str {
    fn encoded_len(&self) -> Result<Length> {
        unimplemented!()
    }

    fn encode(&self, _encoder: &mut Encoder) -> Result<()> {
        unimplemented!()
    }
}

impl Tagged for str {
    const TAG: Tag = Tag::Utf8String;
}
