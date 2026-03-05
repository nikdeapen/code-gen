use crate::{Block, CodeBuffer, Expression, Literal, Statement, WithStatements};

/// An `if` statement with an optional `else` clause.
pub struct IfStatement {
    condition: Box<dyn Expression>,
    success_statements: Block,
    else_statements: Block,
}

impl<E: 'static + Expression> From<E> for IfStatement {
    fn from(expression: E) -> Self {
        Self {
            condition: Box::new(expression),
            success_statements: Block::default(),
            else_statements: Block::default(),
        }
    }
}

impl From<&str> for IfStatement {
    fn from(literal: &str) -> Self {
        Self::from(Literal::from(literal))
    }
}

impl From<String> for IfStatement {
    fn from(literal: String) -> Self {
        Self::from(Literal::from(literal))
    }
}

impl IfStatement {
    //! Success

    /// Gets the success statements.
    #[must_use]
    pub fn success_statements(&self) -> &Block {
        &self.success_statements
    }

    /// Sets the `success` statements.
    pub fn set_success_statements<S>(&mut self, success: S)
    where
        S: Into<Block>,
    {
        self.success_statements = success.into();
    }

    /// Sets the `success` statements.
    #[must_use]
    pub fn with_success_statements<S>(mut self, success: S) -> Self
    where
        S: Into<Block>,
    {
        self.set_success_statements(success);
        self
    }
}

impl IfStatement {
    //! Else

    /// Gets the else statements.
    #[must_use]
    pub fn else_statements(&self) -> &Block {
        &self.else_statements
    }

    /// Sets the `else_statements`.
    pub fn set_else_statements<S>(&mut self, else_statements: S)
    where
        S: Into<Block>,
    {
        self.else_statements = else_statements.into();
    }

    /// Sets the `else_statements`.
    #[must_use]
    pub fn with_else_statements<S>(mut self, else_statements: S) -> Self
    where
        S: Into<Block>,
    {
        self.set_else_statements(else_statements);
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_if() {
        let s = IfStatement::from("x > 0")
            .with_success_statements(Block::default().with_semi("do_something()"));
        assert_eq!(s.to_code(), "if x > 0 {\n    do_something();\n}\n");
    }

    #[test]
    fn if_else() {
        let s = IfStatement::from("x > 0")
            .with_success_statements(Block::default().with_semi("yes()"))
            .with_else_statements(Block::default().with_semi("no()"));
        assert_eq!(
            s.to_code(),
            "if x > 0 {\n    yes();\n} else {\n    no();\n}\n"
        );
    }
}

impl Statement for IfStatement {
    fn write(&self, b: &mut CodeBuffer, level: usize) {
        b.indent(level);
        b.write("if ");
        self.condition.write(b);
        b.write(" {");
        b.end_line();
        self.success_statements.write(b, level + 1);
        if !self.else_statements.statements().is_empty() {
            b.indent(level);
            b.write("} else {");
            b.end_line();
            self.else_statements.write(b, level + 1);
        }
        b.line(level, "}");
    }
}
