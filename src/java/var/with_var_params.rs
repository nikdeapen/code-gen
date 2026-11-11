use crate::java::Var;
use crate::{CodeBuffer, Expression};

/// An element with variable parameters.
pub trait WithVarParams: Sized {
    /// Gets the parameters.
    fn params(&self) -> &[Var];

    /// Adds a parameter.
    fn add_param<V>(&mut self, param: V)
    where
        V: Into<Var>;

    /// Adds a parameter.
    #[must_use]
    fn with_param<V>(mut self, param: V) -> Self
    where
        V: Into<Var>,
    {
        self.add_param(param);
        self
    }

    /// Writes the parameters, without their parentheses.
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
    use crate::java::Signature;

    /// Writes the parameters of the `signature`.
    fn write(signature: Signature) -> String {
        let mut b: CodeBuffer = CodeBuffer::default();
        signature.write_params(&mut b);
        b.into()
    }

    #[test]
    fn no_params() {
        assert_eq!(write(Signature::from("f")), "");
    }

    #[test]
    fn one_param() {
        assert_eq!(
            write(Signature::from("f").with_param(("x", "int"))),
            "int x"
        );
    }

    #[test]
    fn many_params() {
        let signature: Signature = Signature::from("f")
            .with_param(("x", "int"))
            .with_param(("name", "String"));
        assert_eq!(write(signature), "int x, String name");
    }
}
