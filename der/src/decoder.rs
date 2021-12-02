//! DER decoder.

use core::marker::PhantomData;

pub struct Decoder<'a> {
    _pd: PhantomData<&'a ()>
}
