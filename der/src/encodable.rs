//! Trait definition for [`Encodable`].

use crate::{Encoder, Length, Result};

/// Encoding trait.
pub trait Encodable {
    /// Compute the length of this value in bytes when encoded as ASN.1 DER.
    fn encoded_len(&self) -> Result<Length>;

    /// Encode this value as ASN.1 DER using the provided [`Encoder`].
    fn encode(&self, encoder: &mut Encoder) -> Result<()>;

    /// Encode this value to the provided byte slice, returning a sub-slice
    /// containing the encoded message.
    fn encode_to_slice<'a>(&self, _buf: &'a mut [u8]) -> Result<&'a [u8]> {
        unimplemented!()
    }
}
