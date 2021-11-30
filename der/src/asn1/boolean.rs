//! ASN.1 `BOOLEAN` support.

use crate::{asn1::Any, Encodable, Encoder, Error, Length, Result, Tag, Tagged};
use core::convert::TryFrom;

impl TryFrom<Any<'_>> for bool {
    type Error = Error;

    fn try_from(_any: Any<'_>) -> Result<bool> {
        unimplemented!()
    }
}

impl Encodable for bool {
    fn encoded_len(&self) -> Result<Length> {
        unimplemented!()
    }

    fn encode(&self, _encoder: &mut Encoder) -> Result<()> {
        unimplemented!()
    }
}

impl Tagged for bool {
    const TAG: Tag = Tag::Boolean;
}
