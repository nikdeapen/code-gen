use crate::rust::Var;
use crate::{CodeBuffer, Expression};

/// An element with a variable.
pub trait WithVar {
    /// Gets the variable.
    fn var(&self) -> &Var;

    /// Writes the variable.
    fn write_var(&self, b: &mut CodeBuffer) {
        self.var().write(b);
    }
}
