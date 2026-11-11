use crate::cpp::Var;
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
    #[must_use]
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cpp::Signature;

    #[test]
    fn write_params() {
        let signature = Signature::from("foo")
            .with_param(("x", "int"))
            .with_param(("y", "int"));
        let mut b = CodeBuffer::default();
        signature.write_params(&mut b);
        assert_eq!(b.peek(), "int x, int y");
    }

    #[test]
    fn write_no_params() {
        let signature = Signature::from("foo");
        let mut b = CodeBuffer::default();
        signature.write_params(&mut b);
        assert_eq!(b.peek(), "");
    }
}
