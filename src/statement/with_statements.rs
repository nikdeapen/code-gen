use crate::{CodeBuffer, Expression, ExpressionStatement, Literal, Semi, Source, Statement};

/// An element with statements.
pub trait WithStatements: Sized {
    /// Gets the statements.
    fn statements(&self) -> &[Box<dyn Statement>];

    /// Adds the literal statement.
    fn add_literal<L>(&mut self, literal: L)
    where
        L: Into<Literal>,
    {
        self.add_expression_statement(literal.into());
    }

    /// Adds the literal statement.
    fn with_literal<L>(self, literal: L) -> Self
    where
        L: Into<Literal>,
    {
        self.with_expression_statement(literal.into())
    }

    /// Adds the semicolon ended statement.
    fn add_semi<L>(&mut self, literal: L)
    where
        L: Into<Literal>,
    {
        self.add_statement(Semi::from(literal.into()))
    }

    /// Adds the semicolon ended statement.
    fn with_semi<L>(self, literal: L) -> Self
    where
        L: Into<Literal>,
    {
        self.with_statement(Semi::from(literal.into()))
    }

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

    /// Adds the source code.
    fn add_source<S>(&mut self, source: S)
    where
        S: Into<Source>,
    {
        let source: Source = source.into();
        let mut source: Vec<Box<dyn Statement>> = source.into();
        for statement in source.drain(..) {
            self.add_boxed_statement(statement);
        }
    }

    /// Adds the source code.
    fn with_source<S>(mut self, source: S) -> Self
    where
        S: Into<Source>,
    {
        self.add_source(source);
        self
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
