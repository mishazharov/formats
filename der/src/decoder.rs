//! DER decoder.

use crate::{asn1::*, Choice, Decodable, Error, ErrorKind, Length, Result, Tag, TagNumber};
use core::{
    convert::{TryInto},
};

/// DER decoder.
#[derive(Debug)]
pub struct Decoder<'a> {
    /// Byte slice being decoded.
    ///
    /// In the event an error was previously encountered this will be set to
    /// `None` to prevent further decoding while in a bad state.
    bytes: Option<&'a [u8]>,

    /// Position within the decoded slice.
    position: Length,
}

impl<'a> Decoder<'a> {
    /// Create a new decoder for the given byte slice.
    pub fn new(_bytes: &'a [u8]) -> Self {
        unimplemented!()
    }

    /// Decode a value which impls the [`Decodable`] trait.
    pub fn decode<T: Decodable<'a>>(&mut self) -> Result<T> {
        unimplemented!()
    }

    /// Return an error with the given [`ErrorKind`], annotating it with
    /// context about where the error occurred.
    pub fn error(&mut self, _kind: ErrorKind) -> Error {
        unimplemented!()
    }

    /// Return an error for an invalid value with the given tag.
    pub fn value_error(&mut self, _tag: Tag) -> Error {
        unimplemented!()
    }

    /// Did the decoding operation fail due to an error?
    pub fn is_failed(&self) -> bool {
        unimplemented!()
    }

    /// Finish decoding, returning the given value if there is no
    /// remaining data, or an error otherwise
    pub fn finish<T>(self, _value: T) -> Result<T> {
        unimplemented!()
    }

    /// Have we decoded all of the bytes in this [`Decoder`]?
    ///
    /// Returns `false` if we're not finished decoding or if a fatal error
    /// has occurred.
    pub fn is_finished(&self) -> bool {
        unimplemented!()
    }

    /// Attempt to decode an ASN.1 `ANY` value.
    pub fn any(&mut self) -> Result<Any<'a>> {
        unimplemented!()
    }

    /// Attempt to decode an `OPTIONAL` ASN.1 `ANY` value.
    pub fn any_optional(&mut self) -> Result<Option<Any<'a>>> {
        unimplemented!()
    }

    /// Attempt to decode ASN.1 `INTEGER` as `i8`
    pub fn int8(&mut self) -> Result<i8> {
        unimplemented!()
    }

    /// Attempt to decode ASN.1 `INTEGER` as `i16`
    pub fn int16(&mut self) -> Result<i16> {
        unimplemented!()
    }

    /// Attempt to decode unsigned ASN.1 `INTEGER` as `u8`
    pub fn uint8(&mut self) -> Result<u8> {
        unimplemented!()
    }

    /// Attempt to decode unsigned ASN.1 `INTEGER` as `u16`
    pub fn uint16(&mut self) -> Result<u16> {
        unimplemented!()
    }

    /// Attempt to decode an ASN.1 `INTEGER` as a [`UIntBytes`].
    #[cfg(feature = "bigint")]
    #[cfg_attr(docsrs, doc(cfg(feature = "bigint")))]
    pub fn uint_bytes(&mut self) -> Result<UIntBytes<'a>> {
        unimplemented!()
    }

    /// Attempt to decode an ASN.1 `BIT STRING`.
    pub fn bit_string(&mut self) -> Result<BitString<'a>> {
        unimplemented!()
    }

    /// Attempt to decode an ASN.1 `CONTEXT-SPECIFIC` field with the
    /// provided [`TagNumber`].
    ///
    /// This method has the following behavior which is designed to simplify
    /// handling of extension fields, which are denoted in an ASN.1 schema
    /// using the `...` ellipsis extension marker:
    ///
    /// - Skips over [`ContextSpecific`] fields with a tag number lower than
    ///   the current one, consuming and ignoring them.
    /// - Returns `Ok(None)` if a [`ContextSpecific`] field with a higher tag
    ///   number is encountered. These fields are not consumed in this case,
    ///   allowing a field with a lower tag number to be omitted, then the
    ///   higher numbered field consumed as a follow-up.
    /// - Returns `Ok(None)` if anything other than a [`ContextSpecific`] field
    ///   is encountered.
    pub fn context_specific(&mut self, _tag: TagNumber) -> Result<Option<Any<'a>>> {
        unimplemented!()
    }

    /// Attempt to decode an ASN.1 `GeneralizedTime`.
    pub fn generalized_time(&mut self) -> Result<GeneralizedTime> {
        unimplemented!()
    }

    /// Attempt to decode an ASN.1 `IA5String`.
    pub fn ia5_string(&mut self) -> Result<Ia5String<'a>> {
        unimplemented!()
    }

    /// Attempt to decode an ASN.1 `NULL` value.
    pub fn null(&mut self) -> Result<Null> {
        unimplemented!()
    }

    /// Attempt to decode an ASN.1 `OCTET STRING`.
    pub fn octet_string(&mut self) -> Result<OctetString<'a>> {
        unimplemented!()
    }

    /// Attempt to decode an ASN.1 `OPTIONAL` value.
    pub fn optional<T: Choice<'a>>(&mut self) -> Result<Option<T>> {
        unimplemented!()
    }

    /// Attempt to decode an ASN.1 `PrintableString`.
    pub fn printable_string(&mut self) -> Result<PrintableString<'a>> {
        unimplemented!()
    }

    /// Attempt to decode an ASN.1 `UTCTime`.
    pub fn utc_time(&mut self) -> Result<UtcTime> {
        unimplemented!()
    }

    /// Attempt to decode an ASN.1 `UTF8String`.
    pub fn utf8_string(&mut self) -> Result<Utf8String<'a>> {
        unimplemented!()
    }

    /// Attempt to decode an ASN.1 `SEQUENCE`, creating a new nested
    /// [`Decoder`] and calling the provided argument with it.
    pub fn sequence<F, T>(&mut self, _f: F) -> Result<T>
    where
        F: FnOnce(&mut Decoder<'a>) -> Result<T>,
    {
        unimplemented!()
    }

    /// Decode a single byte, updating the internal cursor.
    pub(crate) fn byte(&mut self) -> Result<u8> {
        unimplemented!()
    }

    /// Obtain a slice of bytes of the given length from the current cursor
    /// position, or return an error if we have insufficient data.
    pub(crate) fn bytes(&mut self, _len: impl TryInto<Length>) -> Result<&'a [u8]> {
        unimplemented!()
    }

    /// Peek at the next byte in the decoder without modifying the cursor.
    pub(crate) fn peek(&self) -> Option<u8> {
        unimplemented!()
    }

    /// Obtain the remaining bytes in this decoder from the current cursor
    /// position.
    fn remaining(&self) -> Result<&'a [u8]> {
        unimplemented!()
    }

    /// Get the number of bytes still remaining in the buffer.
    fn remaining_len(&self) -> Result<Length> {
        unimplemented!()
    }
}

impl<'a> From<&'a [u8]> for Decoder<'a> {
    fn from(_bytes: &'a [u8]) -> Decoder<'a> {
        unimplemented!()
    }
}
