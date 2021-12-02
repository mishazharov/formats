//! ASN.1 tags.

mod class;
mod number;

pub use self::{class::Class, number::TagNumber};

/// Types with an associated ASN.1 [`Tag`].
pub trait Tagged {
    /// ASN.1 tag
    const TAG: Tag;
}

pub enum Tag {
}
