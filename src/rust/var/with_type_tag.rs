use crate::rust::RustType;
use crate::{CodeBuffer, Expression};

/// An element with a type tag.
pub trait WithTypeTag {
    /// Gets the type tag.
    fn type_tag(&self) -> &RustType;

    /// Writes the type tag.
    fn write_type_tag(&self, b: &mut CodeBuffer) {
        self.type_tag().write(b);
    }
}
