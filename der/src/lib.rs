#![no_std]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![doc(
    html_logo_url = "https://raw.githubusercontent.com/RustCrypto/meta/master/logo.svg",
    html_favicon_url = "https://raw.githubusercontent.com/RustCrypto/meta/master/logo.svg",
    html_root_url = "https://docs.rs/der/0.4.4"
)]
#![forbid(unsafe_code, clippy::unwrap_used)]
#![warn(rust_2018_idioms, unused_qualifications)]

pub mod asn1;
pub mod message;

mod choice;
mod decodable;
mod decoder;
mod encodable;
mod encoder;
mod error;
mod header;
mod length;
mod tag;

pub use crate::{
    choice::Choice,
    decodable::Decodable,
    decoder::Decoder,
    encodable::Encodable,
    encoder::Encoder,
    error::{Error, ErrorKind, Result},
    header::Header,
    length::Length,
    message::Message,
    tag::{Class, Tag, TagNumber, Tagged},
};

#[cfg(feature = "bigint")]
#[cfg_attr(docsrs, doc(cfg(feature = "bigint")))]
pub use crypto_bigint as bigint;
