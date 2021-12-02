//! ASN.1 `ANY` type.

use core::marker::PhantomData;

/// ASN.1 `ANY`: represents any explicitly tagged ASN.1 value.
///
/// Technically `ANY` hasn't been a recommended part of ASN.1 since the X.209
/// revision from 1988. It was deprecated and replaced by Information Object
/// Classes in X.680 in 1994, and X.690 no longer refers to it whatsoever.
///
/// Nevertheless, this crate defines an [`Any`] type as it remains a familiar
/// and useful concept which is still extensively used in things like
/// PKI-related RFCs.
pub struct Any<'a> {
    _pd: PhantomData<&'a ()>,
}
