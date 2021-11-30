//! Length calculations for encoded ASN.1 DER values

use crate::{Decodable, Decoder, Encodable, Encoder, Error, Result};
use core::{
    convert::{TryFrom},
    fmt,
    ops::Add,
};

/// Maximum length as a `u32` (256 MiB).
const MAX_U32: u32 = 0xfff_ffff;

/// ASN.1-encoded length.
///
/// Maximum length is defined by the [`Length::MAX`] constant (256 MiB).
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq, PartialOrd, Ord)]
pub struct Length(u32);

impl Length {
    /// Length of `0`
    pub const ZERO: Self = Self(0);

    /// Length of `1`
    pub const ONE: Self = Self(1);

    /// Maximum length currently supported: 256 MiB
    pub const MAX: Self = Self(MAX_U32);

    /// Create a new [`Length`] for any value which fits inside of a [`u16`].
    ///
    /// This function is const-safe and therefore useful for [`Length`] constants.
    pub fn new(_value: u16) -> Self {
        unimplemented!()
    }

    /// Get the length of DER Tag-Length-Value (TLV) encoded data if `self`
    /// is the length of the inner "value" portion of the message.
    pub fn for_tlv(self) -> Result<Self> {
        unimplemented!()
    }
}

impl Add for Length {
    type Output = Result<Self>;

    fn add(self, _other: Self) -> Result<Self> {
        unimplemented!()
    }
}

impl Add<u8> for Length {
    type Output = Result<Self>;

    fn add(self, _other: u8) -> Result<Self> {
        unimplemented!()
    }
}

impl Add<u16> for Length {
    type Output = Result<Self>;

    fn add(self, _other: u16) -> Result<Self> {
        unimplemented!()
    }
}

impl Add<u32> for Length {
    type Output = Result<Self>;

    fn add(self, _other: u32) -> Result<Self> {
        unimplemented!()
    }
}

impl Add<usize> for Length {
    type Output = Result<Self>;

    fn add(self, _other: usize) -> Result<Self> {
        unimplemented!()
    }
}

impl Add<Length> for Result<Length> {
    type Output = Self;

    fn add(self, _other: Length) -> Self {
        unimplemented!()
    }
}

impl From<u8> for Length {
    fn from(_len: u8) -> Length {
        unimplemented!()
    }
}

impl From<u16> for Length {
    fn from(_len: u16) -> Length {
        unimplemented!()
    }
}

impl TryFrom<u32> for Length {
    type Error = Error;

    fn try_from(_len: u32) -> Result<Length> {
        unimplemented!()
    }
}

impl TryFrom<usize> for Length {
    type Error = Error;

    fn try_from(_len: usize) -> Result<Length> {
        unimplemented!()
    }
}

impl TryFrom<Length> for usize {
    type Error = Error;

    fn try_from(_len: Length) -> Result<usize> {
        unimplemented!()
    }
}

impl Decodable<'_> for Length {
    fn decode(_decoder: &mut Decoder<'_>) -> Result<Length> {
        unimplemented!()
    }
}

impl Encodable for Length {
    fn encoded_len(&self) -> Result<Length> {
        unimplemented!()
    }

    fn encode(&self, _encoder: &mut Encoder) -> Result<()> {
        unimplemented!()
    }
}

impl fmt::Display for Length {
    fn fmt(&self, _f: &mut fmt::Formatter<'_>) -> fmt::Result {
        unimplemented!()
    }
}
