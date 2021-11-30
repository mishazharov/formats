//! ASN.1 `NULL` support.

use crate::{
    asn1::Any, Error, Result
};
use core::convert::TryFrom;

pub struct Null;

// repro: Deleting this will allow the wasm build to pass
impl TryFrom<Any<'_>> for () {
    type Error = Error;

    fn try_from(_any: Any<'_>) -> Result<()> {
        unimplemented!()
    }
}
