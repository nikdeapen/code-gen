/// An element with an optional expression in place of its statements.
pub trait WithExpression: Sized {
    /// Gets the optional expression.
    fn expression(&self) -> Option<&str>;

    /// Sets the expression.
    fn set_expression<S>(&mut self, expression: S)
    where
        S: Into<String>;

    /// Sets the expression.
    #[must_use]
    fn with_expression<S>(mut self, expression: S) -> Self
    where
        S: Into<String>,
    {
        self.set_expression(expression);
        self
    }
}
