//! ASN.1 `NULL` support.

use crate::{
    asn1::Any, Encodable, Encoder, Error, Length, Result, Tag, Tagged,
};
use core::convert::TryFrom;

/// ASN.1 `NULL` type.
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
pub struct Null;

impl TryFrom<Any<'_>> for Null {
    type Error = Error;

    fn try_from(_any: Any<'_>) -> Result<Null> {
        unimplemented!()
    }
}

impl<'a> From<Null> for Any<'a> {
    fn from(_: Null) -> Any<'a> {
        unimplemented!()
    }
}

impl Encodable for Null {
    fn encoded_len(&self) -> Result<Length> {
        unimplemented!()
    }

    fn encode(&self, _encoder: &mut Encoder) -> Result<()> {
        unimplemented!()
    }
}

impl Tagged for Null {
    const TAG: Tag = Tag::Integer;
}

impl TryFrom<Any<'_>> for () {
    type Error = Error;

    fn try_from(_any: Any<'_>) -> Result<()> {
        unimplemented!()
    }
}

impl<'a> From<()> for Any<'a> {
    fn from(_: ()) -> Any<'a> {
        unimplemented!()
    }
}

impl Encodable for () {
    fn encoded_len(&self) -> Result<Length> {
        unimplemented!()
    }

    fn encode(&self, _encoder: &mut Encoder) -> Result<()> {
        unimplemented!()
    }
}

impl Tagged for () {
    const TAG: Tag = Tag::Null;
}
