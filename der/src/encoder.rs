//! DER encoder.

use crate::{asn1::*, Encodable, Length, Result};
use core::convert::{TryInto};

pub struct Encoder {
}

impl Encoder {
    /// Encode the provided value as an ASN.1 `GeneralizedTime`
    pub fn generalized_time(&mut self, _value: impl TryInto<GeneralizedTime>) -> Result<()> {
        unimplemented!()
    }

    /// Encode a message with the provided [`Encodable`] fields as an
    /// ASN.1 `SEQUENCE`.
    pub fn message(&mut self, _fields: &[&dyn Encodable]) -> Result<()> {
        unimplemented!()
    }

    /// Encode an ASN.1 `SEQUENCE` of the given length.
    ///
    /// Spawns a nested [`Encoder`] which is expected to be exactly the
    /// specified length upon completion.
    pub fn sequence(&mut self, length: Length) -> Result<()>
    {
        if 0usize == length.try_into()? {
            unimplemented!()
        } else {
            unimplemented!()
        }
    }
}
