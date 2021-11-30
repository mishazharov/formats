//! ASN.1 tags.

mod class;
mod number;

pub use self::{class::Class, number::TagNumber};

use crate::{Decodable, Decoder, Error, Result};
use core::{convert::TryFrom, fmt};

/// Types with an associated ASN.1 [`Tag`].
pub trait Tagged {
    /// ASN.1 tag
    const TAG: Tag;
}

/// ASN.1 tags.
///
/// Tags are the leading identifier octet of the Tag-Length-Value encoding
/// used by ASN.1 DER and identify the type of the subsequent value.
///
/// They are described in X.690 Section 8.1.2: Identifier octets, and
/// structured as follows:
///
/// ```text
/// | Class | P/C | Tag Number |
/// ```
///
/// - Bits 8/7: [`Class`]
/// - Bit 6: primitive (0) or constructed (1)
/// - Bits 5-1: tag number
#[derive(Copy, Clone, Eq, PartialEq, PartialOrd, Ord)]
#[non_exhaustive]
pub enum Tag {
    /// `BOOLEAN` tag: 0x01
    Boolean,

    /// `INTEGER` tag: 0x02
    Integer,

    /// `BIT STRING` tag: 0x03
    BitString,

    /// `OCTET STRING` tag: 0x04
    OctetString,

    /// `NULL` tag: 0x05
    Null,

    /// `OBJECT IDENTIFIER` tag: 0x06
    ObjectIdentifier,

    /// `UTF8String` tag: 0x0C
    Utf8String,

    /// `SEQUENCE` tag: 0x10
    Sequence,

    /// `SET` and `SET OF` tag: 0x11
    Set,

    /// `PrintableString` tag: 0x13
    PrintableString,

    /// `IA5String` tag: 0x16
    Ia5String,

    /// `UTCTime` tag: 0x17
    UtcTime,

    /// Application tag.
    Application(TagNumber),

    /// Context-specific tag.
    ContextSpecific(TagNumber),

    /// Private tag number.
    Private(TagNumber),
}

impl Tag {
    /// Assert that this [`Tag`] matches the provided expected tag.
    ///
    /// On mismatch, returns an [`Error`] with [`ErrorKind::UnexpectedTag`].
    pub fn assert_eq(self, _expected: Tag) -> Result<Tag> {
        unimplemented!()
    }

    /// Get the [`Class`] that corresponds to this [`Tag`].
    pub fn class(self) -> Class {
        unimplemented!()
    }

    /// Get the octet encoding for this [`Tag`].
    pub fn octet(self) -> u8 {
        unimplemented!()
    }

    /// Create an [`Error`] for an non-canonical value with the ASN.1 type
    /// identified by this tag.
    pub fn non_canonical_error(self) -> Error {
        unimplemented!()
    }

    /// Create an [`Error`] because the current tag was unexpected, with an
    /// optional expected tag.
    pub fn unexpected_error(self, _expected: Option<Self>) -> Error {
        unimplemented!()
    }

    /// Create an [`Error`] for an invalid value with the ASN.1 type identified
    /// by this tag.
    pub fn value_error(self) -> Error {
        unimplemented!()
    }
}

impl TryFrom<u8> for Tag {
    type Error = Error;

    fn try_from(_byte: u8) -> Result<Tag> {
        unimplemented!()
    }
}

impl From<Tag> for u8 {
    fn from(_tag: Tag) -> u8 {
        unimplemented!()
    }
}

impl From<&Tag> for u8 {
    fn from(_tag: &Tag) -> u8 {
        unimplemented!()
    }
}

impl Decodable<'_> for Tag {
    fn decode(_decoder: &mut Decoder<'_>) -> Result<Self> {
        unimplemented!()
    }
}

impl fmt::Display for Tag {
    fn fmt(&self, _f: &mut fmt::Formatter<'_>) -> fmt::Result {
        unimplemented!()
    }
}

impl fmt::Debug for Tag {
    fn fmt(&self, _f: &mut fmt::Formatter<'_>) -> fmt::Result {
        unimplemented!()
    }
}
