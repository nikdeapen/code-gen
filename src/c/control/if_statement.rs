use crate::{Block, CodeBuffer, Expression, Literal, Statement, WithStatements};

/// An `if` statement with optional `else if` clauses & an optional `else` clause.
pub struct IfStatement {
    condition: Box<dyn Expression>,
    success_statements: Block,
    else_ifs: Vec<(Literal, Block)>,
    else_statements: Block,
}

impl<E: 'static + Expression> From<E> for IfStatement {
    fn from(expression: E) -> Self {
        Self {
            condition: Box::new(expression),
            success_statements: Block::default(),
            else_ifs: Vec::default(),
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

    /// Sets the success statements.
    pub fn set_success_statements<S>(&mut self, success_statements: S)
    where
        S: Into<Block>,
    {
        self.success_statements = success_statements.into();
    }

    /// Sets the success statements.
    #[must_use]
    pub fn with_success_statements<S>(mut self, success_statements: S) -> Self
    where
        S: Into<Block>,
    {
        self.set_success_statements(success_statements);
        self
    }
}

impl IfStatement {
    //! Else If

    /// Gets the `else if` clauses, each a condition & its statements.
    #[must_use]
    pub fn else_ifs(&self) -> &[(Literal, Block)] {
        self.else_ifs.as_slice()
    }

    /// Adds an `else if` clause.
    pub fn add_else_if<C, S>(&mut self, condition: C, statements: S)
    where
        C: Into<Literal>,
        S: Into<Block>,
    {
        self.else_ifs.push((condition.into(), statements.into()));
    }

    /// Adds an `else if` clause.
    #[must_use]
    pub fn with_else_if<C, S>(mut self, condition: C, statements: S) -> Self
    where
        C: Into<Literal>,
        S: Into<Block>,
    {
        self.add_else_if(condition, statements);
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

    /// Sets the else statements.
    pub fn set_else_statements<S>(&mut self, else_statements: S)
    where
        S: Into<Block>,
    {
        self.else_statements = else_statements.into();
    }

    /// Sets the else statements.
    #[must_use]
    pub fn with_else_statements<S>(mut self, else_statements: S) -> Self
    where
        S: Into<Block>,
    {
        self.set_else_statements(else_statements);
        self
    }
}

impl Statement for IfStatement {
    fn write(&self, b: &mut CodeBuffer, level: usize) {
        b.indent(level);
        b.write("if (");
        self.condition.write(b);
        b.write(") {");
        b.end_line();
        self.success_statements.write(b, level + 1);
        for (condition, statements) in &self.else_ifs {
            b.indent(level);
            b.write("} else if (");
            condition.write(b);
            b.write(") {");
            b.end_line();
            statements.write(b, level + 1);
        }
        if !self.else_statements.statements().is_empty() {
            b.line(level, "} else {");
            self.else_statements.write(b, level + 1);
        }
        b.line(level, "}");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_if() {
        let s: IfStatement = IfStatement::from("x > 0")
            .with_success_statements(Block::default().with_semi("do_something()"));
        assert_eq!(s.to_code(), "if (x > 0) {\n    do_something();\n}\n");
    }

    #[test]
    fn if_else() {
        let s: IfStatement = IfStatement::from(String::from("x > 0"))
            .with_success_statements(Block::default().with_semi("yes()"))
            .with_else_statements(Block::default().with_semi("no()"));
        assert_eq!(
            s.to_code(),
            "if (x > 0) {\n    yes();\n} else {\n    no();\n}\n"
        );
    }

    #[test]
    fn if_else_if() {
        let s: IfStatement = IfStatement::from("x > 0")
            .with_success_statements(Block::default().with_semi("more()"))
            .with_else_if("x < 0", Block::default().with_semi("less()"));
        assert_eq!(s.else_ifs().len(), 1);
        assert_eq!(
            s.to_code(),
            "if (x > 0) {\n    more();\n} else if (x < 0) {\n    less();\n}\n"
        );
    }

    #[test]
    fn if_else_ifs_else() {
        let s: IfStatement = IfStatement::from("x > 1")
            .with_success_statements(Block::default().with_semi("many()"))
            .with_else_if("x == 1", Block::default().with_semi("one()"))
            .with_else_if(String::from("x == 0"), Block::default().with_semi("zero()"))
            .with_else_statements(Block::default().with_semi("less()"));
        assert_eq!(
            s.to_code(),
            "if (x > 1) {\n    many();\n} else if (x == 1) {\n    one();\n} else if (x == 0) {\n    zero();\n} else {\n    less();\n}\n"
        );
    }

    #[test]
    fn if_indented() {
        let s: IfStatement = IfStatement::from("x > 0")
            .with_success_statements(Block::default().with_semi("more()"))
            .with_else_if("x < 0", Block::default().with_semi("less()"))
            .with_else_statements(Block::default().with_semi("zero()"));
        let mut b: CodeBuffer = CodeBuffer::default();
        s.write(&mut b, 1);
        assert_eq!(
            b.peek(),
            "    if (x > 0) {\n        more();\n    } else if (x < 0) {\n        less();\n    } else {\n        zero();\n    }\n"
        );
    }
}
