use crate::{CodeBuffer, EmptyLine, Expression, ExpressionStatement, Literal, Semi, Statement};

/// An element with statements.
pub trait WithStatements: Sized {
    /// Gets the statements.
    fn statements(&self) -> &[Box<dyn Statement>];

    /// Adds the boxed `statement`.
    fn add_boxed_statement(&mut self, statement: Box<dyn Statement>);

    /// Adds the boxed `statement`.
    fn with_boxed_statement(mut self, statement: Box<dyn Statement>) -> Self {
        self.add_boxed_statement(statement);
        self
    }

    /// Adds the `statement`.
    fn add_statement<S>(&mut self, statement: S)
    where
        S: 'static + Statement,
    {
        self.add_boxed_statement(Box::new(statement));
    }

    /// Adds the `statement`.
    fn with_statement<S>(self, statement: S) -> Self
    where
        S: 'static + Statement,
    {
        self.with_boxed_statement(Box::new(statement))
    }

    /// Adds the `literal` statement.
    fn add_literal<L>(&mut self, literal: L)
    where
        L: Into<Literal>,
    {
        self.add_expression_statement(literal.into());
    }

    /// Adds the `literal` statement.
    fn with_literal<L>(self, literal: L) -> Self
    where
        L: Into<Literal>,
    {
        self.with_expression_statement(literal.into())
    }

    /// Adds the semicolon ended `literal` statement.
    fn add_semi<L>(&mut self, literal: L)
    where
        L: Into<Literal>,
    {
        self.add_statement(Semi::from(literal.into()))
    }

    /// Adds the semicolon ended `literal` statement.
    fn with_semi<L>(self, literal: L) -> Self
    where
        L: Into<Literal>,
    {
        self.with_statement(Semi::from(literal.into()))
    }

    /// Adds the `expression` as a statement.
    fn add_expression_statement<E>(&mut self, expression: E)
    where
        E: 'static + Expression,
    {
        self.add_statement(ExpressionStatement::from(expression));
    }

    /// Adds the `expression` as a statement.
    fn with_expression_statement<E>(self, expression: E) -> Self
    where
        E: 'static + Expression,
    {
        self.with_statement(ExpressionStatement::from(expression))
    }

    /// Adds an empty line.
    fn add_empty_line(&mut self) {
        self.add_statement(EmptyLine::default());
    }

    /// Adds an empty line.
    fn with_empty_line(self) -> Self {
        self.with_statement(EmptyLine::default())
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
