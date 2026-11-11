use crate::c::CType;
use crate::{CodeBuffer, Expression};

/// An element with a C type.
pub trait WithCType {
    /// Gets the C type.
    fn c_type(&self) -> &CType;

    /// Writes the C type.
    fn write_c_type(&self, b: &mut CodeBuffer) {
        self.c_type().write(b);
    }
}
