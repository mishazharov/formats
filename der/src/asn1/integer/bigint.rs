//! "Big" ASN.1 `INTEGER` types.

use crate::{
    asn1::Any, ByteSlice, Encodable, Encoder, Error, Length, Result, Tag, Tagged,
};
use core::convert::TryFrom;

#[cfg(feature = "bigint")]
use {
    crypto_bigint::{ArrayEncoding, UInt},
};

/// "Big" unsigned ASN.1 `INTEGER` type.
///
/// Provides direct access to the underlying big endian bytes which comprise an
/// unsigned integer value.
///
/// Intended for use cases like very large integers that are used in
/// cryptographic applications (e.g. keys, signatures).
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd)]
pub struct UIntBytes<'a> {
    /// Inner value
    inner: ByteSlice<'a>,
}

impl<'a> UIntBytes<'a> {
    /// Create a new [`UIntBytes`] from a byte slice.
    pub fn new(_bytes: &'a [u8]) -> Result<Self> {
        unimplemented!()
    }

    /// Borrow the inner byte slice which contains the least significant bytes
    /// of a big endian integer value with all leading zeros stripped.
    pub fn as_bytes(&self) -> &'a [u8] {
        unimplemented!()
    }

    /// Get the length of this [`UIntBytes`] in bytes.
    pub fn len(&self) -> Length {
        unimplemented!()
    }

    /// Is the inner byte slice empty?
    pub fn is_empty(&self) -> bool {
        unimplemented!()
    }
}

impl<'a> From<&UIntBytes<'a>> for UIntBytes<'a> {
    fn from(_value: &UIntBytes<'a>) -> UIntBytes<'a> {
        unimplemented!()
    }
}

impl<'a> TryFrom<Any<'a>> for UIntBytes<'a> {
    type Error = Error;

    fn try_from(_any: Any<'a>) -> Result<UIntBytes<'a>> {
        unimplemented!()
    }
}

impl<'a> Encodable for UIntBytes<'a> {
    fn encoded_len(&self) -> Result<Length> {
        unimplemented!()
    }

    fn encode(&self, _encoder: &mut Encoder) -> Result<()> {
        unimplemented!()
    }
}

impl<'a> Tagged for UIntBytes<'a> {
    const TAG: Tag = Tag::Integer;
}

#[cfg(feature = "bigint")]
#[cfg_attr(docsrs, doc(cfg(feature = "bigint")))]
impl<'a, const LIMBS: usize> TryFrom<Any<'a>> for UInt<LIMBS>
where
    UInt<LIMBS>: ArrayEncoding,
{
    type Error = Error;

    fn try_from(_any: Any<'a>) -> Result<UInt<LIMBS>> {
        unimplemented!()
    }
}

#[cfg(feature = "bigint")]
#[cfg_attr(docsrs, doc(cfg(feature = "bigint")))]
impl<'a, const LIMBS: usize> TryFrom<UIntBytes<'a>> for UInt<LIMBS>
where
    UInt<LIMBS>: ArrayEncoding,
{
    type Error = Error;

    fn try_from(_bytes: UIntBytes<'a>) -> Result<UInt<LIMBS>> {
        unimplemented!()
    }
}

#[cfg(feature = "bigint")]
#[cfg_attr(docsrs, doc(cfg(feature = "bigint")))]
impl<'a, const LIMBS: usize> Encodable for UInt<LIMBS>
where
    UInt<LIMBS>: ArrayEncoding,
{
    fn encoded_len(&self) -> Result<Length> {
        unimplemented!()
    }

    fn encode(&self, _encoder: &mut Encoder) -> Result<()> {
        unimplemented!()
    }
}

#[cfg(feature = "bigint")]
#[cfg_attr(docsrs, doc(cfg(feature = "bigint")))]
impl<'a, const LIMBS: usize> Tagged for UInt<LIMBS>
where
    UInt<LIMBS>: ArrayEncoding,
{
    const TAG: Tag = Tag::Integer;
}
