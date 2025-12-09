use crate::rust::Var;
use crate::{CodeBuffer, Expression};

/// An element with variable parameters.
pub trait WithVarParams: Sized {
    /// Gets the parameters.
    fn params(&self) -> &[Var];

    /// Adds the parameter.
    fn add_param<V>(&mut self, param: V)
    where
        V: Into<Var>;

    /// Adds the parameter.
    fn with_param<V>(mut self, param: V) -> Self
    where
        V: Into<Var>,
    {
        self.add_param(param);
        self
    }

    /// Writes the parameters.
    fn write_params(&self, b: &mut CodeBuffer) {
        if let Some((first, rest)) = self.params().split_first() {
            first.write(b);
            for param in rest {
                b.write(", ");
                param.write(b);
            }
        }
    }
}
