use crate::{CodeBuffer, Expression, Literal, Statement, WithStatements};

/// A `match` statement case.
pub struct MatchCase {
    expression: Box<dyn Expression>,
    statements: Vec<Box<dyn Statement>>,
}

impl<E: 'static + Expression> From<E> for MatchCase {
    fn from(expression: E) -> Self {
        Self {
            expression: Box::new(expression),
            statements: Vec::default(),
        }
    }
}

impl From<&str> for MatchCase {
    fn from(literal: &str) -> Self {
        Self::from(Literal::from(literal))
    }
}

impl From<String> for MatchCase {
    fn from(literal: String) -> Self {
        Self::from(Literal::from(literal))
    }
}

impl WithStatements for MatchCase {
    fn statements(&self) -> &[Box<dyn Statement>] {
        self.statements.as_slice()
    }

    fn add_boxed_statement(&mut self, statement: Box<dyn Statement>) {
        self.statements.push(statement);
    }
}

impl Statement for MatchCase {
    fn write(&self, b: &mut CodeBuffer, level: usize) {
        b.indent(level);
        self.expression.write(b);
        b.write(" => ");
        self.write_curly_statement_block(b, level);
        b.end_line();
    }
}
