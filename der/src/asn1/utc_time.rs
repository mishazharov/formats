//! ASN.1 `UTCTime` support.

use crate::{
    asn1::Any,
    Encodable, Encoder, Error, Length, Result, Tag, Tagged,
};
use core::{convert::TryFrom, time::Duration};

/// ASN.1 `UTCTime` type.
///
/// This type implements the validity requirements specified in
/// [RFC 5280 Section 4.1.2.5.1][1], namely:
///
/// > For the purposes of this profile, UTCTime values MUST be expressed in
/// > Greenwich Mean Time (Zulu) and MUST include seconds (i.e., times are
/// > `YYMMDDHHMMSSZ`), even where the number of seconds is zero.  Conforming
/// > systems MUST interpret the year field (`YY`) as follows:
/// >
/// > - Where `YY` is greater than or equal to 50, the year SHALL be
/// >   interpreted as `19YY`; and
/// > - Where `YY` is less than 50, the year SHALL be interpreted as `20YY`.
///
/// [1]: https://tools.ietf.org/html/rfc5280#section-4.1.2.5.1
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
pub struct UtcTime(Duration);

impl UtcTime {
    /// Length of an RFC 5280-flavored ASN.1 DER-encoded [`UtcTime`].
    pub const LENGTH: Length = Length::new(13);

    /// Create a new [`UtcTime`] given a [`Duration`] since `UNIX_EPOCH`
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

impl From<&UtcTime> for UtcTime {
    fn from(_value: &UtcTime) -> UtcTime {
        unimplemented!()
    }
}

impl TryFrom<Any<'_>> for UtcTime {
    type Error = Error;

    fn try_from(_any: Any<'_>) -> Result<UtcTime> {
        unimplemented!()
    }
}

impl Encodable for UtcTime {
    fn encoded_len(&self) -> Result<Length> {
        unimplemented!()
    }

    fn encode(&self, _encoder: &mut Encoder) -> Result<()> {
        unimplemented!()
    }
}

impl Tagged for UtcTime {
    const TAG: Tag = Tag::UtcTime;
}
