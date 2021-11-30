//! Context-specific field.

use crate::{
    asn1::Any, Choice, Encodable, Encoder, Error, Length, Result, Tag, TagNumber,
};
use core::convert::TryFrom;

/// Context-specific field.
///
/// This type encodes a field which is specific to a particular context,
/// and is identified by a [`TagNumber`].
///
/// Any context-specific field can be decoded/encoded with this type.
/// The intended use is to dynamically dispatch off of the context-specific
/// tag number when decoding, which allows support for extensions, which are
/// denoted in an ASN.1 schema using the `...` ellipsis extension marker.
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
pub struct ContextSpecific<'a> {
    /// Context-specific tag number sans the leading `0b10000000` class
    /// identifier bit and `0b100000` constructed flag.
    pub tag_number: TagNumber,

    /// Value of the field.
    pub value: Any<'a>,
}

impl<'a> Choice<'a> for ContextSpecific<'a> {
    fn can_decode(_tag: Tag) -> bool {
        unimplemented!()
    }
}

impl<'a> Encodable for ContextSpecific<'a> {
    fn encoded_len(&self) -> Result<Length> {
        unimplemented!()
    }

    fn encode(&self, _encoder: &mut Encoder) -> Result<()> {
        unimplemented!()
    }
}

impl<'a> From<&ContextSpecific<'a>> for ContextSpecific<'a> {
    fn from(_value: &ContextSpecific<'a>) -> ContextSpecific<'a> {
        unimplemented!()
    }
}

impl<'a> TryFrom<Any<'a>> for ContextSpecific<'a> {
    type Error = Error;

    fn try_from(_any: Any<'a>) -> Result<ContextSpecific<'a>> {
        unimplemented!()
    }
}
