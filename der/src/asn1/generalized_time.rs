//! ASN.1 `GeneralizedTime` support.

use crate::{
    asn1::Any,
    Encodable, Encoder, Error, Length, Result, Tag, Tagged,
};
use core::{convert::TryFrom, time::Duration};

/// ASN.1 `GeneralizedTime` type.
///
/// This type implements the validity requirements specified in
/// [RFC 5280 Section 4.1.2.5.2][1], namely:
///
/// > For the purposes of this profile, GeneralizedTime values MUST be
/// > expressed in Greenwich Mean Time (Zulu) and MUST include seconds
/// > (i.e., times are `YYYYMMDDHHMMSSZ`), even where the number of seconds
/// > is zero.  GeneralizedTime values MUST NOT include fractional seconds.
///
/// [1]: https://tools.ietf.org/html/rfc5280#section-4.1.2.5.2
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
pub struct GeneralizedTime(Duration);

impl GeneralizedTime {
    /// Length of an RFC 5280-flavored ASN.1 DER-encoded [`GeneralizedTime`].
    pub const LENGTH: Length = Length::new(15);

    /// Create a new [`GeneralizedTime`] given a [`Duration`] since `UNIX_EPOCH`
    /// (a.k.a. "Unix time")
    pub fn new(_unix_duration: Duration) -> Result<Self> {
        unimplemented!()
    }

    /// Get the duration of this timestamp since `UNIX_EPOCH`.
    pub fn unix_duration(&self) -> Duration {
        unimplemented!()
    }

    /// Instantiate from [`SystemTime`].
    #[cfg(feature = "std")]
    #[cfg_attr(docsrs, doc(cfg(feature = "std")))]
    pub fn from_system_time(time: SystemTime) -> Result<Self> {
        unimplemented!()
    }

    /// Convert to [`SystemTime`].
    #[cfg(feature = "std")]
    #[cfg_attr(docsrs, doc(cfg(feature = "std")))]
    pub fn to_system_time(&self) -> SystemTime {
        unimplemented!()
    }
}

impl From<&GeneralizedTime> for GeneralizedTime {
    fn from(_value: &GeneralizedTime) -> GeneralizedTime {
        unimplemented!()
    }
}

impl TryFrom<Any<'_>> for GeneralizedTime {
    type Error = Error;

    fn try_from(_any: Any<'_>) -> Result<GeneralizedTime> {
        unimplemented!()
    }
}

impl Encodable for GeneralizedTime {
    fn encoded_len(&self) -> Result<Length> {
        unimplemented!()
    }

    fn encode(&self, _encoder: &mut Encoder) -> Result<()> {
        unimplemented!()
    }
}

impl Tagged for GeneralizedTime {
    const TAG: Tag = Tag::GeneralizedTime;
}
