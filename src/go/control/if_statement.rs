use crate::{Block, CodeBuffer, Expression, Literal, Statement, WithStatements};

/// An `if` statement with optional `else if` & `else` clauses.
pub struct IfStatement {
    condition: Box<dyn Expression>,
    success_statements: Block,
    else_ifs: Vec<(Literal, Block)>,
    else_statements: Block,
}

impl<E: 'static + Expression> From<E> for IfStatement {
    fn from(condition: E) -> Self {
        Self {
            condition: Box::new(condition),
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
        b.write("if ");
        self.condition.write(b);
        b.write(" {");
        b.end_line();
        self.success_statements.write(b, level + 1);
        for (condition, statements) in &self.else_ifs {
            b.indent(level);
            b.write("} else if ");
            condition.write(b);
            b.write(" {");
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
        let s = IfStatement::from("err != nil")
            .with_success_statements(Block::default().with_literal("return nil, err"));
        assert_eq!(s.to_code(), "if err != nil {\n    return nil, err\n}\n");
    }

    #[test]
    fn if_else() {
        let s = IfStatement::from("x < y")
            .with_success_statements(Block::default().with_literal("return -1"))
            .with_else_statements(Block::default().with_literal("return 1"));
        assert_eq!(
            s.to_code(),
            "if x < y {\n    return -1\n} else {\n    return 1\n}\n"
        );
    }

    #[test]
    fn if_else_if() {
        let s = IfStatement::from("x < y")
            .with_success_statements(Block::default().with_literal("return -1"))
            .with_else_if("x > y", Block::default().with_literal("return 1"));
        assert_eq!(
            s.to_code(),
            "if x < y {\n    return -1\n} else if x > y {\n    return 1\n}\n"
        );
    }

    #[test]
    fn if_else_ifs_and_else() {
        let s = IfStatement::from("x < y")
            .with_success_statements(Block::default().with_literal("return -1"))
            .with_else_if("x > y", Block::default().with_literal("return 1"))
            .with_else_if(String::from("x == 0"), Block::default())
            .with_else_statements(Block::default().with_literal("return 0"));
        assert_eq!(
            s.to_code(),
            "if x < y {\n    return -1\n} else if x > y {\n    return 1\n} else if x == 0 {\n} else {\n    return 0\n}\n"
        );
    }

    #[test]
    fn if_indented() {
        let s = IfStatement::from(String::from("ok"))
            .with_success_statements(Block::default().with_literal("count++"))
            .with_else_if("done", Block::default().with_literal("count = 0"))
            .with_else_statements(Block::default().with_literal("count--"));
        let mut b = CodeBuffer::default();
        s.write(&mut b, 1);
        assert_eq!(
            b.peek(),
            "    if ok {\n        count++\n    } else if done {\n        count = 0\n    } else {\n        count--\n    }\n"
        );
    }
}
