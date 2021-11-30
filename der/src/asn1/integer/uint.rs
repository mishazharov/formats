//! Unsigned integer decoders/encoders.

use crate::{asn1::Any, Encoder, Length, Result};

/// Decode an unsigned integer into a byte array of the requested size
/// containing a big endian integer.
pub(super) fn decode_array<const N: usize>(_any: Any<'_>) -> Result<[u8; N]> {
    unimplemented!()
}

/// Encode the given big endian bytes representing an integer as ASN.1 DER.
pub(super) fn encode(_encoder: &mut Encoder, _bytes: &[u8]) -> Result<()> {
    unimplemented!()
}

/// Get the encoded length for the given unsigned integer serialized as bytes.
#[inline]
pub(super) fn encoded_len(_bytes: &[u8]) -> Result<Length> {
    unimplemented!()
}
