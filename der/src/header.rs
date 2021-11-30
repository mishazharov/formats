//! ASN.1 DER headers.

use crate::{Decodable, Decoder, Length, Result, Tag};
use core::convert::TryInto;

/// ASN.1 DER headers: tag + length component of TLV-encoded values
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Header {
    /// Tag representing the type of the encoded value
    pub tag: Tag,

    /// Length of the encoded value
    pub length: Length,
}

impl Header {
    /// Create a new [`Header`] from a [`Tag`] and a specified length.
    ///
    /// Returns an error if the length exceeds the limits of [`Length`].
    pub fn new(_tag: Tag, _length: impl TryInto<Length>) -> Result<Self> {
        unimplemented!()
    }
}

impl Decodable<'_> for Header {
    fn decode(_decoder: &mut Decoder<'_>) -> Result<Header> {
        unimplemented!()
    }
}
