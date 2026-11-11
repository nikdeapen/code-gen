use crate::python::WithIndentedBlock;
use crate::{Block, CodeBuffer, Expression, Literal, Statement, WithStatements};

/// An `if` statement with optional `else if` clauses & an optional `else` clause.
///
/// # Note
/// - An `else if` clause is written as `elif`.
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
    fn from(condition: &str) -> Self {
        Self::from(Literal::from(condition))
    }
}

impl From<String> for IfStatement {
    fn from(condition: String) -> Self {
        Self::from(Literal::from(condition))
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
        b.push(':');
        b.end_line();
        self.success_statements
            .write_indented_statement_block(b, level);
        for (condition, statements) in &self.else_ifs {
            b.indent(level);
            b.write("elif ");
            condition.write(b);
            b.push(':');
            b.end_line();
            statements.write_indented_statement_block(b, level);
        }
        if !self.else_statements.statements().is_empty() {
            b.line(level, "else:");
            self.else_statements
                .write_indented_statement_block(b, level);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_if() {
        let s = IfStatement::from("x > 0")
            .with_success_statements(Block::default().with_literal("do_something()"));
        assert_eq!(s.to_code(), "if x > 0:\n    do_something()\n");
    }

    #[test]
    fn if_else() {
        let s = IfStatement::from(String::from("x > 0"))
            .with_success_statements(Block::default().with_literal("yes()"))
            .with_else_statements(Block::default().with_literal("no()"));
        assert_eq!(s.to_code(), "if x > 0:\n    yes()\nelse:\n    no()\n");
    }

    #[test]
    fn if_else_if() {
        let s = IfStatement::from("x > 0")
            .with_success_statements(Block::default().with_literal("positive()"))
            .with_else_if("x < 0", Block::default().with_literal("negative()"));
        assert_eq!(
            s.to_code(),
            "if x > 0:\n    positive()\nelif x < 0:\n    negative()\n"
        );
    }

    #[test]
    fn if_else_ifs_else() {
        let s = IfStatement::from("x > 9")
            .with_success_statements(Block::default().with_literal("large()"))
            .with_else_if("x > 0", Block::default().with_literal("small()"))
            .with_else_if(String::from("x < 0"), Block::default())
            .with_else_statements(Block::default().with_literal("zero()"));
        assert_eq!(
            s.to_code(),
            "if x > 9:\n    large()\nelif x > 0:\n    small()\nelif x < 0:\n    pass\nelse:\n    zero()\n"
        );
        assert_eq!(s.else_ifs().len(), 2);
    }

    #[test]
    fn empty_if() {
        let s = IfStatement::from("x > 0");
        assert_eq!(s.to_code(), "if x > 0:\n    pass\n");
    }

    #[test]
    fn if_else_indented() {
        let s = IfStatement::from("x > 0")
            .with_success_statements(Block::default().with_literal("yes()"))
            .with_else_if("x < 0", Block::default().with_literal("maybe()"))
            .with_else_statements(Block::default().with_literal("no()"));
        let mut b = CodeBuffer::default();
        s.write(&mut b, 1);
        assert_eq!(
            b.peek(),
            "    if x > 0:\n        yes()\n    elif x < 0:\n        maybe()\n    else:\n        no()\n"
        );
    }
}
