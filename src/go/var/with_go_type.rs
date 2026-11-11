use crate::go::GoType;
use crate::{CodeBuffer, Expression};

/// An element with a Go type.
pub trait WithGoType {
    /// Gets the Go type.
    fn go_type(&self) -> &GoType;

    /// Writes the Go type.
    fn write_go_type(&self, b: &mut CodeBuffer) {
        self.go_type().write(b);
    }
}
