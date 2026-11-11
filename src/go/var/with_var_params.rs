use crate::go::Var;
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
    use crate::Expression;
    use crate::go::{Signature, WithVarParams};

    #[test]
    fn no_params() {
        let s = Signature::from("Encode");
        assert_eq!(s.to_code(), "Encode()");
    }

    #[test]
    fn one_param() {
        let s = Signature::from("Encode").with_param(("out", "[]byte"));
        assert_eq!(s.to_code(), "Encode(out []byte)");
    }

    #[test]
    fn several_params() {
        let s = Signature::from("Encode")
            .with_param(("out", "[]byte"))
            .with_param(("depth", "int"));
        assert_eq!(s.to_code(), "Encode(out []byte, depth int)");
    }
}
