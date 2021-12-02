//! Length calculations for encoded ASN.1 DER values

use crate::{Error, Result};
use core::{
    convert::{TryFrom},
};

pub struct Length(u32);

impl TryFrom<Length> for usize {
    type Error = Error;

    fn try_from(_len: Length) -> Result<usize> {
        unimplemented!()
    }
}
