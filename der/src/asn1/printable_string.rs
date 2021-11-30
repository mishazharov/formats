//! ASN.1 `PrintableString` support.

use crate::{
    asn1::Any, str_slice::StrSlice, Encodable, Encoder, Error, Length, Result, Tag, Tagged,
};
use core::{convert::TryFrom, fmt, str};

/// ASN.1 `PrintableString` type.
///
/// Supports a subset the ASCII character set (desribed below).
///
/// For UTF-8, use [`Utf8String`][`crate::asn1::Utf8String`] instead. For the
/// full ASCII character set, use [`Ia5String`][`crate::asn1::Ia5String`].
///
/// # Supported characters
///
/// The following ASCII characters/ranges are supported:
///
/// - `A..Z`
/// - `a..z`
/// - `0..9`
/// - "` `" (i.e. space)
/// - `\`
/// - `(`
/// - `)`
/// - `+`
/// - `,`
/// - `-`
/// - `.`
/// - `/`
/// - `:`
/// - `=`
/// - `?`
#[derive(Copy, Clone, Eq, PartialEq, PartialOrd, Ord)]
pub struct PrintableString<'a> {
    /// Inner value
    inner: StrSlice<'a>,
}

impl<'a> PrintableString<'a> {
    /// Create a new ASN.1 `PrintableString`.
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

impl AsRef<str> for PrintableString<'_> {
    fn as_ref(&self) -> &str {
        unimplemented!()
    }
}

impl AsRef<[u8]> for PrintableString<'_> {
    fn as_ref(&self) -> &[u8] {
        unimplemented!()
    }
}

impl<'a> From<&PrintableString<'a>> for PrintableString<'a> {
    fn from(_value: &PrintableString<'a>) -> PrintableString<'a> {
        unimplemented!()
    }
}

impl<'a> TryFrom<Any<'a>> for PrintableString<'a> {
    type Error = Error;

    fn try_from(_any: Any<'a>) -> Result<PrintableString<'a>> {
        unimplemented!()
    }
}

impl<'a> From<PrintableString<'a>> for Any<'a> {
    fn from(_printable_string: PrintableString<'a>) -> Any<'a> {
        unimplemented!()
    }
}

impl<'a> From<PrintableString<'a>> for &'a [u8] {
    fn from(_printable_string: PrintableString<'a>) -> &'a [u8] {
        unimplemented!()
    }
}

impl<'a> Encodable for PrintableString<'a> {
    fn encoded_len(&self) -> Result<Length> {
        unimplemented!()
    }

    fn encode(&self, _encoder: &mut Encoder) -> Result<()> {
        unimplemented!()
    }
}

impl<'a> Tagged for PrintableString<'a> {
    const TAG: Tag = Tag::PrintableString;
}

impl<'a> fmt::Display for PrintableString<'a> {
    fn fmt(&self, _f: &mut fmt::Formatter<'_>) -> fmt::Result {
        unimplemented!()
    }
}

impl<'a> fmt::Debug for PrintableString<'a> {
    fn fmt(&self, _f: &mut fmt::Formatter<'_>) -> fmt::Result {
        unimplemented!()
    }
}
