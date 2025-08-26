use crate::{CodeBuffer, Expression, Literal};

/// An element with expression params.
pub trait WithParams: Sized {
    /// Gets the params.
    fn params(&self) -> &[Box<dyn Expression>];

    /// Adds the boxed `param`.
    fn add_boxed_param(&mut self, param: Box<dyn Expression>);

    /// Adds the boxed `param`.
    fn with_boxed_param(mut self, param: Box<dyn Expression>) -> Self {
        self.add_boxed_param(param);
        self
    }

    /// Adds the `param`.
    fn add_param<E>(&mut self, param: E)
    where
        E: 'static + Expression,
    {
        self.add_boxed_param(Box::new(param));
    }

    /// Adds the `param`.
    fn with_param<E>(mut self, param: E) -> Self
    where
        E: 'static + Expression,
    {
        self.add_param(param);
        self
    }

    /// Adds the `literal` param.
    fn add_literal_param<L>(&mut self, literal: L)
    where
        L: Into<Literal>,
    {
        self.add_param(literal.into());
    }

    /// Adds the `literal` param.
    fn with_literal_param<L>(self, literal: L) -> Self
    where
        L: Into<Literal>,
    {
        self.with_param(literal.into())
    }

    /// Writes the params. (comma separated)
    fn write_params(&self, b: &mut CodeBuffer) {
        if let Some((first, rest)) = self.params().split_first() {
            first.write(b);
            for param in rest {
                b.write(", ");
                param.write(b);
            }
        }
    }

    /// Writes the params with parentheses. (comma separated)
    fn write_params_with_parens(&self, b: &mut CodeBuffer) {
        b.write("(");
        self.write_params(b);
        b.write(")");
    }
}
