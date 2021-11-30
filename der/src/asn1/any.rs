//! ASN.1 `ANY` type.

use crate::{
    Choice, Decodable, Decoder, Encodable, Encoder, Error,
    Length, Result, Tag,
};
use core::convert::{TryFrom};
use core::marker::PhantomData;

/// ASN.1 `ANY`: represents any explicitly tagged ASN.1 value.
///
/// Technically `ANY` hasn't been a recommended part of ASN.1 since the X.209
/// revision from 1988. It was deprecated and replaced by Information Object
/// Classes in X.680 in 1994, and X.690 no longer refers to it whatsoever.
///
/// Nevertheless, this crate defines an [`Any`] type as it remains a familiar
/// and useful concept which is still extensively used in things like
/// PKI-related RFCs.
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
pub struct Any<'a> {
    /// Tag representing the type of the encoded value.
    tag: Tag,
    _pd: PhantomData<&'a ()>,

    /// Encoded length of this [`Any`] value.
    length: Length,
}

impl<'a> Any<'a> {
    /// Create a new [`Any`] from the provided [`Tag`] and byte slice.
    pub fn new(_tag: Tag, _bytes: &'a [u8]) -> Result<Self> {
        unimplemented!()
    }

    /// Get the tag for this [`Any`] type.
    pub fn tag(self) -> Tag {
        unimplemented!()
    }

    /// Get the [`Length`] of this [`Any`] type's value.
    pub fn len(self) -> Length {
        unimplemented!()
    }

    /// Is the body of this [`Any`] type empty?
    pub fn is_empty(self) -> bool {
        unimplemented!()
    }

    /// Is this value an ASN.1 NULL value?
    pub fn is_null(self) -> bool {
        unimplemented!()
    }

    /// Get the raw value for this [`Any`] type as a byte slice.
    pub fn as_bytes(self) -> &'a [u8] {
        unimplemented!()
    }

    /// Attempt to decode an ASN.1 `OBJECT IDENTIFIER`.
    #[cfg(feature = "oid")]
    #[cfg_attr(docsrs, doc(cfg(feature = "oid")))]
    pub fn oid(self) -> Result<ObjectIdentifier> {
        unimplemented!()
    }

    /// Attempt to decode an ASN.1 `OPTIONAL` value.
    pub fn optional<T>(self) -> Result<Option<T>>
    where
        T: Choice<'a> + TryFrom<Self, Error = Error>,
    {
        unimplemented!()
    }

    /// Attempt to decode this value an ASN.1 `SEQUENCE`, creating a new
    /// nested [`Decoder`] and calling the provided argument with it.
    pub fn sequence<F, T>(self, _f: F) -> Result<T>
    where
        F: FnOnce(&mut Decoder<'a>) -> Result<T>,
    {
        unimplemented!()
    }
}

impl<'a> Choice<'a> for Any<'a> {
    fn can_decode(_: Tag) -> bool {
        true
    }
}

impl<'a> Decodable<'a> for Any<'a> {
    fn decode(_decoder: &mut Decoder<'a>) -> Result<Any<'a>> {
        unimplemented!()
    }
}

impl<'a> Encodable for Any<'a> {
    fn encoded_len(&self) -> Result<Length> {
        unimplemented!()
    }

    fn encode(&self, _encoder: &mut Encoder) -> Result<()> {
        unimplemented!()
    }
}

impl<'a> TryFrom<&'a [u8]> for Any<'a> {
    type Error = Error;

    fn try_from(_bytes: &'a [u8]) -> Result<Any<'a>> {
        unimplemented!()
    }
}
