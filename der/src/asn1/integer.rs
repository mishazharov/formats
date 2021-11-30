//! ASN.1 `INTEGER` support.

pub(super) mod bigint;
mod int;
mod uint;

use crate::{asn1::Any, Encodable, Encoder, Error, Length, Result, Tag, Tagged};
use core::convert::TryFrom;

macro_rules! impl_int_encoding {
    ($($int:ty => $uint:ty),+) => {
        $(
            impl TryFrom<Any<'_>> for $int {
                type Error = Error;

                fn try_from(_any: Any<'_>) -> Result<Self> {
                    unimplemented!()
                }
            }

            impl Encodable for $int {
                fn encoded_len(&self) -> Result<Length> {
                    unimplemented!()
                }

                fn encode(&self, _encoder: &mut Encoder) -> Result<()> {
                    unimplemented!()
                }
            }

            impl Tagged for $int {
                const TAG: Tag = Tag::Integer;
            }
        )+
    };
}

macro_rules! impl_uint_encoding {
    ($($uint:ty),+) => {
        $(
            impl TryFrom<Any<'_>> for $uint {
                type Error = Error;

                fn try_from(_any: Any<'_>) -> Result<Self> {
                    unimplemented!()
                }
            }

            impl Encodable for $uint {
                fn encoded_len(&self) -> Result<Length> {
                    unimplemented!()
                }

                fn encode(&self, _encoder: &mut Encoder) -> Result<()> {
                    unimplemented!()
                }
            }

            impl Tagged for $uint {
                const TAG: Tag = Tag::Integer;
            }
        )+
    };
}

impl_int_encoding!(i8 => u8, i16 => u16, i32 => u32, i64 => u64, i128 => u128);
impl_uint_encoding!(u8, u16, u32, u64, u128);
