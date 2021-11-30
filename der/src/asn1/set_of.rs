//! ASN.1 `SET OF` support.

use crate::{
    asn1::Any, ByteSlice, Decodable, Encodable, Encoder, Error, Length, Result,
    Tag, Tagged,
};
use core::{convert::TryFrom, marker::PhantomData};

/// ASN.1 `SET OF` denotes a collection of zero or more occurrences of a
/// given type.
///
/// When encoded as DER, `SET OF` is lexicographically ordered. To implement
/// that requirement, types `T` which are elements of [`SetOf`] MUST provide
/// an impl of `Ord` which ensures that the corresponding DER encodings of
/// a given type are ordered.
pub trait SetOf<'a, 'b, T>: Decodable<'a> + Encodable
where
    T: Clone + Decodable<'a> + Encodable + Ord,
{
    /// Iterator over the elements of the set.
    ///
    /// The iterator type MUST maintain the invariant that messages are
    /// lexicographically ordered.
    ///
    /// See toplevel documentation about `Ord` trait requirements for
    /// more information.
    type Iter: Iterator<Item = T>;

    /// Iterate over the elements of the set.
    fn elements(&'b self) -> Self::Iter;
}

/// ASN.1 `SET OF` backed by a byte slice containing serialized DER.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct SetOfRef<'a, T>
where
    T: Clone + Decodable<'a> + Encodable + Ord,
{
    /// DER-encoded byte slice
    inner: ByteSlice<'a>,

    /// Set element type
    element_type: PhantomData<T>,
}

impl<'a, T> SetOfRef<'a, T>
where
    T: Clone + Decodable<'a> + Encodable + Ord,
{
    /// Create a new [`SetOfRef`] from a slice.
    pub fn new(_slice: &'a [u8]) -> Result<Self> {
        unimplemented!()
    }

    /// Borrow the inner byte sequence.
    pub fn as_bytes(&self) -> &'a [u8] {
        unimplemented!()
    }
}

impl<'a, T> AsRef<[u8]> for SetOfRef<'a, T>
where
    T: Clone + Decodable<'a> + Encodable + Ord,
{
    fn as_ref(&self) -> &[u8] {
        unimplemented!()
    }
}

impl<'a, T> TryFrom<Any<'a>> for SetOfRef<'a, T>
where
    T: Clone + Decodable<'a> + Encodable + Ord,
{
    type Error = Error;

    fn try_from(_any: Any<'a>) -> Result<Self> {
        unimplemented!()
    }
}

impl<'a, T> From<SetOfRef<'a, T>> for Any<'a>
where
    T: Clone + Decodable<'a> + Encodable + Ord,
{
    fn from(_set: SetOfRef<'a, T>) -> Any<'a> {
        unimplemented!()
    }
}

impl<'a, T> Encodable for SetOfRef<'a, T>
where
    T: Clone + Decodable<'a> + Encodable + Ord,
{
    fn encoded_len(&self) -> Result<Length> {
        unimplemented!()
    }

    fn encode(&self, _encoder: &mut Encoder) -> Result<()> {
        unimplemented!()
    }
}

impl<'a, 'b, T> SetOf<'a, 'b, T> for SetOfRef<'a, T>
where
    T: Clone + Decodable<'a> + Encodable + Ord,
{
    type Iter = SetOfRefIter<'a, T>;

    fn elements(&'b self) -> Self::Iter {
        unimplemented!()
    }
}

impl<'a, T> Tagged for SetOfRef<'a, T>
where
    T: Clone + Decodable<'a> + Encodable + Ord,
{
    const TAG: Tag = Tag::Set;
}

/// Iterator over the elements of an [`SetOfRef`].
pub struct SetOfRefIter<'a, T>
where
    T: Clone + Decodable<'a> + Encodable + Ord,
{
    /// Decoder which iterates over the elements of the message
    _decoder: PhantomData<&'a ()>,

    /// Element type
    element_type: PhantomData<T>,
}

impl<'a, T> Iterator for SetOfRefIter<'a, T>
where
    T: Clone + Decodable<'a> + Encodable + Ord,
{
    type Item = T;

    fn next(&mut self) -> Option<T> {
        unimplemented!()
    }
}

#[cfg(feature = "alloc")]
#[cfg_attr(docsrs, doc(cfg(feature = "alloc")))]
impl<'a, T> TryFrom<Any<'a>> for BTreeSet<T>
where
    T: Clone + Decodable<'a> + Encodable + Ord,
{
    type Error = Error;

    fn try_from(any: Any<'a>) -> Result<Self> {
        unimplemented!()
    }
}

#[cfg(feature = "alloc")]
#[cfg_attr(docsrs, doc(cfg(feature = "alloc")))]
impl<'a, T> Encodable for BTreeSet<T>
where
    T: Clone + Decodable<'a> + Encodable + Ord,
{
    fn encoded_len(&self) -> Result<Length> {
        unimplemented!()
    }

    fn encode(&self, encoder: &mut Encoder) -> Result<()> {
        unimplemented!()
    }
}

#[cfg(feature = "alloc")]
#[cfg_attr(docsrs, doc(cfg(feature = "alloc")))]
impl<'a, 'b, T: 'b> SetOf<'a, 'b, T> for BTreeSet<T>
where
    T: Clone + Decodable<'a> + Encodable + Ord,
{
    type Iter = core::iter::Cloned<btree_set::Iter<'b, T>>;

    fn elements(&'b self) -> Self::Iter {
        unimplemented!()
    }
}

#[cfg(feature = "alloc")]
#[cfg_attr(docsrs, doc(cfg(feature = "alloc")))]
impl<'a, T> Tagged for BTreeSet<T>
where
    T: Clone + Decodable<'a> + Encodable + Ord,
{
    const TAG: Tag = Tag::Set;
}

/// Get the encoded length of a [`BTreeSet`]
#[cfg(feature = "alloc")]
fn btreeset_inner_len<'a, T>(set: &BTreeSet<T>) -> Result<Length>
where
    T: Clone + Decodable<'a> + Encodable + Ord,
{
    unimplemented!()
}
