//! ASN.1 `SEQUENCE` support.

pub(super) mod iter;

use self::iter::SequenceIter;
use crate::{
    asn1::Any, ByteSlice, Decodable, Decoder, Encodable, Encoder, Error, Length, Result,
    Tag, Tagged,
};
use core::convert::TryFrom;

/// ASN.1 `SEQUENCE` type.
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
pub struct Sequence<'a> {
    /// Inner value
    inner: ByteSlice<'a>,
}

impl<'a> Sequence<'a> {
    /// Borrow the inner byte sequence.
    pub fn as_bytes(&self) -> &'a [u8] {
        unimplemented!()
    }

    /// Decode values nested within a sequence, creating a new [`Decoder`] for
    /// the data contained in the sequence's body and passing it to the provided
    /// [`FnOnce`].
    pub fn decode_nested<F, T>(&self, _f: F) -> Result<T>
    where
        F: FnOnce(&mut Decoder<'a>) -> Result<T>,
    {
        unimplemented!()
    }

    /// Iterate over the values in a heterogenously typed sequence.
    pub fn iter<T: Decodable<'a>>(&self) -> SequenceIter<'a, T> {
        unimplemented!()
    }
}

impl AsRef<[u8]> for Sequence<'_> {
    fn as_ref(&self) -> &[u8] {
        unimplemented!()
    }
}

impl<'a> TryFrom<Any<'a>> for Sequence<'a> {
    type Error = Error;

    fn try_from(_any: Any<'a>) -> Result<Self> {
        unimplemented!()
    }
}

impl<'a> From<Sequence<'a>> for Any<'a> {
    fn from(_seq: Sequence<'a>) -> Any<'a> {
        unimplemented!()
    }
}

impl<'a> Encodable for Sequence<'a> {
    fn encoded_len(&self) -> Result<Length> {
        unimplemented!()
    }

    fn encode(&self, _encoder: &mut Encoder) -> Result<()> {
        unimplemented!()
    }
}

impl<'a> Tagged for Sequence<'a> {
    const TAG: Tag = Tag::Sequence;
}
