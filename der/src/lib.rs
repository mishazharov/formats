#![no_std]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![doc(
    html_logo_url = "https://raw.githubusercontent.com/RustCrypto/meta/master/logo.svg",
    html_favicon_url = "https://raw.githubusercontent.com/RustCrypto/meta/master/logo.svg",
    html_root_url = "https://docs.rs/der/0.4.4"
)]
#![forbid(unsafe_code, clippy::unwrap_used)]
#![warn(rust_2018_idioms, unused_qualifications)]

use core::marker::PhantomData;
use core::convert::TryInto;
use core::convert::TryFrom;

pub struct Any<'a> {
    _pd: PhantomData<&'a ()>,
}

pub use core::str::Utf8Error;
use core::{convert::Infallible};

/// Error type.
pub struct Error {
}

// repro: also needed for some reason
impl From<Infallible> for Error {
    fn from(_: Infallible) -> Error {
        unreachable!()
    }
}

// repro: Deleting this will allow the wasm build to pass
impl TryFrom<Any<'_>> for () {
    type Error = Error;

    fn try_from(_any: Any<'_>) -> Result<(), Error> {
        unimplemented!()
    }
}

pub struct Length(u32);

impl TryFrom<Length> for usize {
    type Error = Error;

    fn try_from(_len: Length) -> Result<usize, Error> {
        unimplemented!()
    }
}


pub struct Encoder {
}

impl Encoder {

    /// Encode an ASN.1 `SEQUENCE` of the given length.
    ///
    /// Spawns a nested [`Encoder`] which is expected to be exactly the
    /// specified length upon completion.
    pub fn sequence(&mut self, length: Length) -> Result<(), Error>
    {
        if 0usize == length.try_into()? {
            unimplemented!()
        } else {
            unimplemented!()
        }
    }
}

// repro: also important
use rand_core::{CryptoRng, RngCore};
