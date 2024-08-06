use crate::{CodeBuffer, Expression, Literal};

/// An element with expression parameters.
pub trait WithParams: Sized {
    /// Gets the parameters.
    fn params(&self) -> &[Box<dyn Expression>];

    /// Adds the boxed parameter.
    fn add_boxed_param(&mut self, param: Box<dyn Expression>);

    /// Adds the boxed parameter.
    fn with_boxed_param(mut self, param: Box<dyn Expression>) -> Self {
        self.add_boxed_param(param);
        self
    }

    /// Adds the parameter.
    fn add_param<E>(&mut self, param: E)
    where
        E: 'static + Expression,
    {
        self.add_boxed_param(Box::new(param));
    }

    /// Adds the parameter.
    fn with_param<E>(mut self, param: E) -> Self
    where
        E: 'static + Expression,
    {
        self.add_param(param);
        self
    }

    /// Adds the literal parameter.
    fn add_literal_param<L>(&mut self, literal: L)
    where
        L: Into<Literal>,
    {
        self.add_param(literal.into());
    }

    /// Adds the literal parameter.
    fn with_literal_param<L>(self, literal: L) -> Self
    where
        L: Into<Literal>,
    {
        self.with_param(literal.into())
    }

    /// Writes the parameters. (comma separated)
    fn write_params(&self, b: &mut CodeBuffer) {
        if let Some((first, rest)) = self.params().split_first() {
            first.write(b);
            for param in rest {
                b.write(", ");
                param.write(b);
            }
        }
    }

    /// Writes the parameters with parentheses. (comma separated)
    fn write_params_with_parens(&self, b: &mut CodeBuffer) {
        b.write("(");
        self.write_params(b);
        b.write(")");
    }
}
