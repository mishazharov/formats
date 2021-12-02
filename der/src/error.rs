//! Error types.

pub use core::str::Utf8Error;

use crate::{Length, Tag};
use core::{convert::Infallible, fmt};

/// Result type.
pub type Result<T> = core::result::Result<T, Error>;

/// Error type.
pub struct Error {
}

impl Error {
    /// Create a new [`Error`].
    pub fn new(_kind: ErrorKind, _position: Length) -> Error {
        unimplemented!()
    }

    /// Get the [`ErrorKind`] which occurred.
    pub fn kind(self) -> ErrorKind {
        unimplemented!()
    }

    /// Get the position inside of the message where the error occurred.
    pub fn position(self) -> Option<Length> {
        unimplemented!()
    }

    /// For errors occurring inside of a nested message, extend the position
    /// count by the location where the nested message occurs.
    pub fn nested(self, _nested_position: Length) -> Self {
        unimplemented!()
    }
}

impl fmt::Display for Error {
    fn fmt(&self, _f: &mut fmt::Formatter<'_>) -> fmt::Result {
        unimplemented!()
    }
}

impl From<ErrorKind> for Error {
    fn from(_kind: ErrorKind) -> Error {
        unimplemented!()
    }
}

impl From<Infallible> for Error {
    fn from(_: Infallible) -> Error {
        unreachable!()
    }
}

impl From<Utf8Error> for Error {
    fn from(_err: Utf8Error) -> Error {
        unimplemented!()
    }
}

#[cfg(feature = "oid")]
impl From<const_oid::Error> for Error {
    fn from(_: const_oid::Error) -> Error {
        unimplemented!()
    }
}

/// Error type.
#[non_exhaustive]
pub enum ErrorKind {
    /// Indicates a field which is duplicated when only one is expected.
    DuplicateField {
        /// Tag of the duplicated field.
        tag: Tag,
    },

    /// This error indicates a previous DER parsing operation resulted in
    /// an error and tainted the state of a `Decoder` or `Encoder`.
    ///
    /// Once this occurs, the overall operation has failed and cannot be
    /// subsequently resumed.
    Failed,

    /// Incorrect length for a given field.
    Length {
        /// Tag of the value being decoded.
        tag: Tag,
    },

    /// Message is not canonically encoded.
    Noncanonical {
        /// Tag of the value which is not canonically encoded.
        tag: Tag,
    },

    /// Malformed OID
    MalformedOid,

    /// Integer overflow occurred (library bug!).
    Overflow,

    /// Message is longer than this library's internal limits support.
    Overlength,

    /// Undecoded trailing data at end of message.
    TrailingData {
        /// Length of the decoded data.
        decoded: Length,

        /// Total length of the remaining data left in the buffer.
        remaining: Length,
    },

    /// Unexpected end-of-message/nested field when decoding.
    Truncated,

    /// Encoded message is shorter than the expected length.
    ///
    /// (i.e. an `Encodable` impl on a particular type has a buggy `encoded_len`)
    Underlength {
        /// Expected length
        expected: Length,

        /// Actual length
        actual: Length,
    },

    /// Unexpected tag.
    UnexpectedTag {
        /// Tag the decoder was expecting (if there is a single such tag).
        ///
        /// `None` if multiple tags are expected/allowed, but the `actual` tag
        /// does not match any of them.
        expected: Option<Tag>,

        /// Actual tag encountered in the message.
        actual: Tag,
    },

    /// Unknown/unsupported tag.
    UnknownTag {
        /// Raw byte value of the tag.
        byte: u8,
    },

    /// UTF-8 errors.
    Utf8(Utf8Error),

    /// Unexpected value.
    Value {
        /// Tag of the unexpected value.
        tag: Tag,
    },
}

impl ErrorKind {
    /// Annotate an [`ErrorKind`] with context about where it occurred,
    /// returning an error.
    pub fn at(self, position: Length) -> Error {
        Error::new(self, position)
    }
}

impl fmt::Display for ErrorKind {
    fn fmt(&self, _f: &mut fmt::Formatter<'_>) -> fmt::Result {
        unimplemented!()
    }
}
