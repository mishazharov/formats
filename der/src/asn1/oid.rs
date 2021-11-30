//! ASN.1 `OBJECT IDENTIFIER`

use crate::{asn1::Any, Encodable, Encoder, Error, Length, Result, Tag, Tagged};
use const_oid::ObjectIdentifier;
use core::convert::{TryFrom, TryInto};

impl TryFrom<Any<'_>> for ObjectIdentifier {
    type Error = Error;

    fn try_from(any: Any<'_>) -> Result<ObjectIdentifier> {
        unimplemented!()
    }
}

impl<'a> From<&'a ObjectIdentifier> for Any<'a> {
    fn from(oid: &'a ObjectIdentifier) -> Any<'a> {
        unimplemented!()
    }
}

impl Encodable for ObjectIdentifier {
    fn encoded_len(&self) -> Result<Length> {
        unimplemented!()
    }

    fn encode(&self, encoder: &mut Encoder) -> Result<()> {
        unimplemented!()
    }
}

impl<'a> Tagged for ObjectIdentifier {
    const TAG: Tag = Tag::ObjectIdentifier;
}
