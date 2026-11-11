use crate::c::Var;
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

    /// Writes the parameters. (`void` if there are none)
    fn write_params(&self, b: &mut CodeBuffer) {
        if let Some((first, rest)) = self.params().split_first() {
            first.write(b);
            for param in rest {
                b.write(", ");
                param.write(b);
            }
        } else {
            b.write("void");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::c::Signature;

    /// Writes the signature's parameters to a string.
    fn write(signature: &Signature) -> String {
        let mut b: CodeBuffer = CodeBuffer::default();
        signature.write_params(&mut b);
        b.into()
    }

    #[test]
    fn no_params() {
        assert_eq!(write(&Signature::from("foo")), "void");
    }

    #[test]
    fn one_param() {
        let signature: Signature = Signature::from("foo").with_param(("x", "int32_t"));
        assert_eq!(write(&signature), "int32_t x");
    }

    #[test]
    fn many_params() {
        let signature: Signature = Signature::from("foo")
            .with_param(("x", "int32_t"))
            .with_param(("y", "int32_t"));
        assert_eq!(write(&signature), "int32_t x, int32_t y");
    }
}
