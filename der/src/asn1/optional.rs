//! ASN.1 `OPTIONAL` as mapped to Rust's `Option` type

use crate::{Choice, Decodable, Decoder, Encodable, Encoder, Length, Result};

impl<'a, T> Decodable<'a> for Option<T>
where
    T: Choice<'a>, // NOTE: all `Decodable + Tagged` types receive a blanket `Choice` impl
{
    fn decode(_decoder: &mut Decoder<'a>) -> Result<Option<T>> {
        unimplemented!()
    }
}

impl<T> Encodable for Option<T>
where
    T: Encodable,
{
    fn encoded_len(&self) -> Result<Length> {
        unimplemented!()
    }

    fn encode(&self, _encoder: &mut Encoder) -> Result<()> {
        unimplemented!()
    }
}
