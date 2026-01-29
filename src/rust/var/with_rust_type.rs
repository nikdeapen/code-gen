use crate::rust::RustType;
use crate::{CodeBuffer, Expression};

/// An element with a Rust type.
pub trait WithRustType {
    /// Gets the Rust type.
    fn rust_type(&self) -> &RustType;

    /// Writes the Rust type.
    fn write_rust_type(&self, b: &mut CodeBuffer) {
        self.rust_type().write(b);
    }
}
