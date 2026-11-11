use crate::typescript::Var;
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
    use crate::typescript::Signature;

    #[test]
    fn no_params() {
        assert_eq!(Signature::from("foo").to_code(), "foo()");
    }

    #[test]
    fn one_param() {
        let s = Signature::from("foo").with_param(("x", "number"));
        assert_eq!(s.to_code(), "foo(x: number)");
    }

    #[test]
    fn many_params() {
        let s = Signature::from("foo")
            .with_param("x")
            .with_param(Var::from(("y", "number")).with_optional(true));
        assert_eq!(s.to_code(), "foo(x, y?: number)");
    }
}
