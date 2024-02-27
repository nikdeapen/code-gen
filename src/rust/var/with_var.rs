use crate::rust::{Var, WithTypeTag};
use crate::{CodeBuffer, Expression, WithName};

/// An element with a variable.
pub trait WithVar: WithName + WithTypeTag {
    /// Gets the variable.
    fn var(&self) -> &Var;

    /// Writes the variable.
    fn write_var(&self, b: &mut CodeBuffer) {
        self.var().write(b);
    }
}
