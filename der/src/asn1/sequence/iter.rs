//! Sequence iterator.

use crate::{Decodable, Decoder, Result};
use core::marker::PhantomData;

/// ASN.1 `SEQUENCE` iterator for [`Sequence`][`super::Sequence`] containing
/// homogeneously typed values.
pub struct SequenceIter<'a, T>
where
    T: Decodable<'a>,
{
    /// Type being decoded
    decodable: PhantomData<&'a T>,
}

impl<'a, T> SequenceIter<'a, T>
where
    T: Decodable<'a>,
{
    /// Create a new sequence iterator for the given type.
    pub(super) fn new(_decoder: Decoder<'a>) -> Self {
        unimplemented!()
    }
}

impl<'a, T> Iterator for SequenceIter<'a, T>
where
    T: Decodable<'a>,
{
    type Item = Result<T>;

    fn next(&mut self) -> Option<Result<T>> {
        unimplemented!()
    }
}
