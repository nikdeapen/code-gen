use crate::{CodeBuffer, Expression, Literal, Statement, WithStatements};

/// A `match` statement case.
///
/// # Note
/// - A case with an expression is written on one line. (ex: `1 => "one",`) Its statements are not
///   written.
pub struct MatchCase {
    pattern: Box<dyn Expression>,
    expression: Option<String>,
    statements: Vec<Box<dyn Statement>>,
}

impl<E: 'static + Expression> From<E> for MatchCase {
    fn from(expression: E) -> Self {
        Self {
            pattern: Box::new(expression),
            expression: None,
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

impl MatchCase {
    //! Expression

    /// Gets the optional expression, which is in place of the statements.
    #[must_use]
    pub fn expression(&self) -> Option<&str> {
        self.expression.as_deref()
    }

    /// Sets the `expression`, which is in place of the statements.
    pub fn set_expression<S>(&mut self, expression: S)
    where
        S: Into<String>,
    {
        self.expression = Some(expression.into());
    }

    /// Sets the `expression`, which is in place of the statements.
    #[must_use]
    pub fn with_expression<S>(mut self, expression: S) -> Self
    where
        S: Into<String>,
    {
        self.set_expression(expression);
        self
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
        self.pattern.write(b);
        b.write(" => ");
        if let Some(expression) = &self.expression {
            b.write(expression);
            b.push(',');
        } else {
            self.write_curly_statement_block(b, level);
        }
        b.end_line();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_with_expression() {
        let c: MatchCase = MatchCase::from("1")
            .with_literal("log()")
            .with_expression("\"one\"");
        assert_eq!(c.expression(), Some("\"one\""));
        assert_eq!(c.to_code(), "1 => \"one\",\n");
    }

    #[test]
    fn case_with_statements() {
        let c: MatchCase = MatchCase::from(String::from("Self::Circle(radius)"))
            .with_semi("target.write(1)")
            .with_semi("target.write(radius)");
        let mut b: CodeBuffer = CodeBuffer::default();
        c.write(&mut b, 1);
        assert_eq!(c.expression(), None);
        assert_eq!(
            b.peek(),
            "    Self::Circle(radius) => {\n        target.write(1);\n        target.write(radius);\n    }\n"
        );
    }

    #[test]
    fn empty_case() {
        let c: MatchCase = MatchCase::from("_");
        assert_eq!(c.to_code(), "_ => {}\n");
    }

    #[test]
    fn case_from_expression() {
        let c: MatchCase = MatchCase::from(Literal::from("2..=9")).with_expression("\"digit\"");
        let mut b: CodeBuffer = CodeBuffer::default();
        c.write(&mut b, 1);
        assert_eq!(b.peek(), "    2..=9 => \"digit\",\n");
    }
}
