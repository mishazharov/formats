//! DER encoder.

use crate::{Length, Result};
use core::convert::{TryInto};

pub struct Encoder {
}

impl Encoder {

    /// Encode an ASN.1 `SEQUENCE` of the given length.
    ///
    /// Spawns a nested [`Encoder`] which is expected to be exactly the
    /// specified length upon completion.
    pub fn sequence(&mut self, length: Length) -> Result<()>
    {
        if 0usize == length.try_into()? {
            unimplemented!()
        } else {
            unimplemented!()
        }
    }
}
