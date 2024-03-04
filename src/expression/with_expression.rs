use crate::Expression;

/// An element with an expression.
pub trait WithExpression {
    /// Gets the expression.
    fn expression(&self) -> &Box<dyn Expression>;
}
