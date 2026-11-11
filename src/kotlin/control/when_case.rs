use crate::kotlin::WithExpression;
use crate::{CodeBuffer, Expression, Literal, Statement, WithStatements};

/// A case of a `when` statement.
///
/// # Note
/// - A case with an expression is written on one line. (ex: `1 -> "one"`) Its statements are not
///   written.
pub struct WhenCase {
    condition: Box<dyn Expression>,
    expression: Option<String>,
    statements: Vec<Box<dyn Statement>>,
}

impl<E: 'static + Expression> From<E> for WhenCase {
    fn from(expression: E) -> Self {
        Self {
            condition: Box::new(expression),
            expression: None,
            statements: Vec::default(),
        }
    }
}

impl From<&str> for WhenCase {
    fn from(literal: &str) -> Self {
        Self::from(Literal::from(literal))
    }
}

impl From<String> for WhenCase {
    fn from(literal: String) -> Self {
        Self::from(Literal::from(literal))
    }
}

impl WithExpression for WhenCase {
    fn expression(&self) -> Option<&str> {
        self.expression.as_deref()
    }

    fn set_expression<S>(&mut self, expression: S)
    where
        S: Into<String>,
    {
        self.expression = Some(expression.into());
    }
}

impl WithStatements for WhenCase {
    fn statements(&self) -> &[Box<dyn Statement>] {
        self.statements.as_slice()
    }

    fn add_boxed_statement(&mut self, statement: Box<dyn Statement>) {
        self.statements.push(statement);
    }
}

impl Statement for WhenCase {
    fn write(&self, b: &mut CodeBuffer, level: usize) {
        b.indent(level);
        self.condition.write(b);
        b.write(" -> ");
        if let Some(expression) = &self.expression {
            b.write(expression);
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
        let c = WhenCase::from("1")
            .with_literal("log()")
            .with_expression("\"one\"");
        assert_eq!(c.to_code(), "1 -> \"one\"\n");
    }

    #[test]
    fn case_with_statements() {
        let c = WhenCase::from(String::from("is Circle"))
            .with_literal("out.write(1)")
            .with_literal("out.write(radius)");
        let mut b = CodeBuffer::default();
        c.write(&mut b, 1);
        assert_eq!(
            b.peek(),
            "    is Circle -> {\n        out.write(1)\n        out.write(radius)\n    }\n"
        );
    }

    #[test]
    fn empty_case() {
        let c = WhenCase::from("else");
        assert_eq!(c.to_code(), "else -> {}\n");
    }

    #[test]
    fn case_from_expression() {
        let c = WhenCase::from(Literal::from("in 1..9")).with_expression("\"digit\"");
        assert_eq!(c.to_code(), "in 1..9 -> \"digit\"\n");
    }
}
