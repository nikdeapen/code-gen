use crate::kotlin::Var;
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
    use crate::kotlin::Signature;

    /// Writes the parameters of the `signature`.
    fn code(signature: &Signature) -> String {
        let mut b = CodeBuffer::default();
        signature.write_params(&mut b);
        b.into()
    }

    #[test]
    fn no_params() {
        assert_eq!(code(&Signature::from("f")), "");
    }

    #[test]
    fn one_param() {
        assert_eq!(
            code(&Signature::from("f").with_param(("x", "Int"))),
            "x: Int"
        );
    }

    #[test]
    fn many_params() {
        let s = Signature::from("f")
            .with_param(("x", "Int"))
            .with_param(("y", "String"));
        assert_eq!(code(&s), "x: Int, y: String");
    }
}
