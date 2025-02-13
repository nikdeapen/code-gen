use crate::{CodeBuffer, Expression, Literal, Source, Statement, WithStatements};

/// An `if` statement with an optional `else` clause.
pub struct IfStatement {
    condition: Box<dyn Expression>,
    success_statements: Source,
    else_statements: Source,
}

impl<E: 'static + Expression> From<E> for IfStatement {
    fn from(expression: E) -> Self {
        Self {
            condition: Box::new(expression),
            success_statements: Source::default(),
            else_statements: Source::default(),
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
    pub fn success_statements(&self) -> &Source {
        &self.success_statements
    }

    /// Sets the `success` statements.
    pub fn set_success_statements<S>(&mut self, success: S)
    where
        S: Into<Source>,
    {
        self.success_statements = success.into();
    }

    /// Sets the `success` statements.
    pub fn with_success_statements<S>(mut self, success: S) -> Self
    where
        S: Into<Source>,
    {
        self.set_success_statements(success);
        self
    }
}

impl IfStatement {
    //! Else

    /// Gets the else statements.
    pub fn else_statements(&self) -> &Source {
        &self.else_statements
    }

    /// Sets the `else_statements`.
    pub fn set_else_statements<S>(&mut self, else_statements: S)
    where
        S: Into<Source>,
    {
        self.else_statements = else_statements.into();
    }

    /// Sets the `else_statements`.
    pub fn with_else_statements<S>(mut self, else_statements: S) -> Self
    where
        S: Into<Source>,
    {
        self.set_else_statements(else_statements);
        self
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
