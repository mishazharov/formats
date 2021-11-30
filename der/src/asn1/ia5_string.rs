//! ASN.1 `IA5String` support.

use crate::{
    asn1::Any, str_slice::StrSlice, Encodable, Encoder, Error, Length, Result, Tag, Tagged,
};
use core::{convert::TryFrom, fmt, str};

/// ASN.1 `IA5String` type.
///
/// Supports the [International Alphabet No. 5 (IA5)] character encoding, i.e.
/// the lower 128 characters of the ASCII alphabet. (Note: IA5 is now
/// technically known as the International Reference Alphabet or IRA as
/// specified in the ITU-T's T.50 recommendation).
///
/// For UTF-8, use [`Utf8String`][`crate::asn1::Utf8String`].
///
/// [International Alphabet No. 5 (IA5)]: https://en.wikipedia.org/wiki/T.50_%28standard%29
#[derive(Copy, Clone, Eq, PartialEq, PartialOrd, Ord)]
pub struct Ia5String<'a> {
    /// Inner value
    inner: StrSlice<'a>,
}

impl<'a> Ia5String<'a> {
    /// Create a new `IA5String`.
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

impl AsRef<str> for Ia5String<'_> {
    fn as_ref(&self) -> &str {
        unimplemented!()
    }
}

impl AsRef<[u8]> for Ia5String<'_> {
    fn as_ref(&self) -> &[u8] {
        unimplemented!()
    }
}

impl<'a> From<&Ia5String<'a>> for Ia5String<'a> {
    fn from(_value: &Ia5String<'a>) -> Ia5String<'a> {
        unimplemented!()
    }
}

impl<'a> TryFrom<Any<'a>> for Ia5String<'a> {
    type Error = Error;

    fn try_from(_any: Any<'a>) -> Result<Ia5String<'a>> {
        unimplemented!()
    }
}

impl<'a> From<Ia5String<'a>> for Any<'a> {
    fn from(_printable_string: Ia5String<'a>) -> Any<'a> {
        unimplemented!()
    }
}

impl<'a> From<Ia5String<'a>> for &'a [u8] {
    fn from(_printable_string: Ia5String<'a>) -> &'a [u8] {
        unimplemented!()
    }
}

impl<'a> Encodable for Ia5String<'a> {
    fn encoded_len(&self) -> Result<Length> {
        unimplemented!()
    }

    fn encode(&self, _encoder: &mut Encoder) -> Result<()> {
        unimplemented!()
    }
}

impl<'a> Tagged for Ia5String<'a> {
    const TAG: Tag = Tag::Ia5String;
}

impl<'a> fmt::Display for Ia5String<'a> {
    fn fmt(&self, _f: &mut fmt::Formatter<'_>) -> fmt::Result {
        unimplemented!()
    }
}

impl<'a> fmt::Debug for Ia5String<'a> {
    fn fmt(&self, _f: &mut fmt::Formatter<'_>) -> fmt::Result {
        unimplemented!()
    }
}
