//! Date and time functionality shared between various ASN.1 types
//! (e.g. `GeneralizedTime`, `UTCTime`)

// Adapted from the `humantime` crate.
// Copyright (c) 2016 The humantime Developers
// Released under the MIT OR Apache 2.0 licenses

use crate::{Encoder, Result, Tag};
use core::time::Duration;

/// Minimum year allowed in [`DateTime`] values.
const MIN_YEAR: u16 = 1970;

/// Maximum duration since `UNIX_EPOCH` which can be represented as a
/// [`DateTime`] (non-inclusive).
const MAX_UNIX_DURATION: Duration = Duration::from_secs(253_402_300_800);

/// Decode 2-digit decimal value
pub(crate) fn decode_decimal(_tag: Tag, _hi: u8, _lo: u8) -> Result<u16> {
    unimplemented!()
}

/// Encode 2-digit decimal value
pub(crate) fn encode_decimal(encoder: &mut Encoder, tag: Tag, value: u16) -> Result<()> {
    unimplemented!()
}

/// Inner date/time type shared by multiple ASN.1 types
/// (e.g. `GeneralizedTime`, `UTCTime`).
///
/// Following conventions from RFC 5280, this type is always Z-normalized
/// (i.e. represents a UTC time). However, it isn't named "UTC time" in order
/// to prevent confusion with ASN.1 `UTCTime`.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub(crate) struct DateTime {
    /// Full year (e.g. 2000).
    ///
    /// Must be >=1970 to permit positive conversions to Unix time.
    year: u16,

    /// Month (1-12)
    month: u16,

    /// Day of the month (1-31)
    day: u16,

    /// Hour (0-23)
    hour: u16,

    /// Minute (0-59)
    minute: u16,

    /// Second (0-59)
    second: u16,
}

impl DateTime {
    /// Create a new [`DateTime`] from the given UTC time components.
    ///
    /// Note that this does not fully validate the components of the date.
    /// To ensure the date is valid, it must be converted to a Unix timestamp
    /// by calling [`DateTime::unix_timestamp`].
    pub(crate) fn new(
        year: u16,
        month: u16,
        day: u16,
        hour: u16,
        minute: u16,
        second: u16,
    ) -> Option<Self> {
        unimplemented!()
    }

    /// Compute a [`DateTime`] from the given [`Duration`] since the `UNIX_EPOCH`.
    ///
    /// Returns `None` if the value is outside the supported date range.
    pub fn from_unix_duration(unix_duration: Duration) -> Option<Self> {
        unimplemented!()
    }

    /// Get the year
    pub fn year(&self) -> u16 {
        unimplemented!()
    }

    /// Get the month
    pub fn month(&self) -> u16 {
        unimplemented!()
    }

    /// Get the day
    pub fn day(&self) -> u16 {
        unimplemented!()
    }

    /// Get the hour
    pub fn hour(&self) -> u16 {
        unimplemented!()
    }

    /// Get the minute
    pub fn minute(&self) -> u16 {
        unimplemented!()
    }

    /// Get the second
    pub fn second(&self) -> u16 {
        unimplemented!()
    }

    /// Compute [`Duration`] since `UNIX_EPOCH` from the given calendar date.
    pub(crate) fn unix_duration(&self) -> Option<Duration> {
        unimplemented!()
    }

    /// Is the year a leap year?
    fn is_leap_year(&self) -> bool {
        unimplemented!()
    }
}
