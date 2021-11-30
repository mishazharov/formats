//! DER decoder.

use crate::{asn1::*, Decodable, Error, ErrorKind, Length, Result, Tag, TagNumber};

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

    /// Attempt to decode an ASN.1 `NULL` value.
    pub fn null(&mut self) -> Result<Null> {
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
}

impl<'a> From<&'a [u8]> for Decoder<'a> {
    fn from(_bytes: &'a [u8]) -> Decoder<'a> {
        unimplemented!()
    }
}
