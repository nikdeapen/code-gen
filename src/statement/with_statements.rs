use crate::{CodeBuffer, Expression, ExpressionStatement, Statement};

/// An element with a list of statements.
pub trait WithStatements: Sized {
    /// Gets the statements.
    fn statements(&self) -> &[Box<dyn Statement>];

    /// Adds the statement.
    fn with_statement<S>(self, statement: S) -> Self
    where
        S: 'static + Statement,
    {
        self.with_boxed_statement(Box::new(statement))
    }

    /// Adds the statement.
    fn add_statement<S>(&mut self, statement: S)
    where
        S: 'static + Statement,
    {
        self.add_boxed_statement(Box::new(statement));
    }

    /// Adds the boxed statement.
    fn with_boxed_statement(mut self, statement: Box<dyn Statement>) -> Self {
        self.add_boxed_statement(statement);
        self
    }

    /// Adds the boxed statement.
    fn add_boxed_statement(&mut self, statement: Box<dyn Statement>);

    /// Adds the expression as a statement.
    fn with_expression_statement<E>(self, expression: E) -> Self
    where
        E: 'static + Expression,
    {
        self.with_statement(ExpressionStatement::from(expression))
    }

    /// Adds the expression as a statement.
    fn add_expression_statement<E>(&mut self, expression: E)
    where
        E: 'static + Expression,
    {
        self.add_statement(ExpressionStatement::from(expression));
    }

    /// Writes the statements.
    fn write_statements(&self, b: &mut CodeBuffer, level: usize) {
        for statement in self.statements() {
            statement.write(b, level);
        }
    }

    /// Writes the curly-bracketed statement block. (`level` is the outer level)
    fn write_curly_statement_block(&self, b: &mut CodeBuffer, level: usize) {
        b.write("{");
        if self.statements().is_empty() {
            b.write("}");
        } else {
            b.end_line();
            self.write_statements(b, level + 1);
            b.indent(level);
            b.write("}");
        }
    }
}
