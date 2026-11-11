use crate::{CodeBuffer, Expression, Literal, Statement, WithStatements};

/// A case of a `switch`, in the arrow form.
///
/// # Note
/// - The default value is the `default` case.
/// - A case with an expression & no statements is one line. (ex: `case 1 -> RED;`)
/// - A case with an expression after its statements yields it.
#[derive(Default)]
pub struct SwitchCase {
    label: Option<Box<dyn Expression>>,
    expression: Option<String>,
    statements: Vec<Box<dyn Statement>>,
}

impl<E: 'static + Expression> From<E> for SwitchCase {
    fn from(label: E) -> Self {
        Self {
            label: Some(Box::new(label)),
            expression: None,
            statements: Vec::default(),
        }
    }
}

impl From<&str> for SwitchCase {
    fn from(literal: &str) -> Self {
        Self::from(Literal::from(literal))
    }
}

impl From<String> for SwitchCase {
    fn from(literal: String) -> Self {
        Self::from(Literal::from(literal))
    }
}

impl SwitchCase {
    //! Expression

    /// Gets the optional expression.
    #[must_use]
    pub fn expression(&self) -> Option<&str> {
        self.expression.as_deref()
    }

    /// Sets the expression.
    pub fn set_expression<S>(&mut self, expression: S)
    where
        S: Into<String>,
    {
        self.expression = Some(expression.into());
    }

    /// Sets the expression.
    #[must_use]
    pub fn with_expression<S>(mut self, expression: S) -> Self
    where
        S: Into<String>,
    {
        self.set_expression(expression);
        self
    }
}

impl WithStatements for SwitchCase {
    fn statements(&self) -> &[Box<dyn Statement>] {
        self.statements.as_slice()
    }

    fn add_boxed_statement(&mut self, statement: Box<dyn Statement>) {
        self.statements.push(statement);
    }
}

impl Statement for SwitchCase {
    fn write(&self, b: &mut CodeBuffer, level: usize) {
        b.indent(level);
        match &self.label {
            Some(label) => {
                b.write("case ");
                label.write(b);
            }
            None => b.write("default"),
        }
        b.write(" -> ");
        match &self.expression {
            Some(expression) if self.statements.is_empty() => {
                b.write(expression.as_str());
                b.push(';');
            }
            Some(expression) => {
                b.push('{');
                b.end_line();
                self.write_statements(b, level + 1);
                b.indent(level + 1);
                b.write("yield ");
                b.write(expression.as_str());
                b.push(';');
                b.end_line();
                b.indent(level);
                b.push('}');
            }
            None => self.write_curly_statement_block(b, level),
        }
        b.end_line();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_case() {
        let c: SwitchCase = SwitchCase::from("1");
        assert_eq!(c.to_code(), "case 1 -> {}\n");
    }

    #[test]
    fn case_from_a_string() {
        let c: SwitchCase = SwitchCase::from(String::from("1"));
        assert_eq!(c.to_code(), "case 1 -> {}\n");
    }

    #[test]
    fn case_from_an_expression() {
        let c: SwitchCase = SwitchCase::from(Literal::from("1"));
        assert_eq!(c.to_code(), "case 1 -> {}\n");
    }

    #[test]
    fn default_case() {
        let c: SwitchCase = SwitchCase::default().with_semi("other()");
        assert_eq!(c.to_code(), "default -> {\n    other();\n}\n");
    }

    #[test]
    fn case_with_statements() {
        let c: SwitchCase = SwitchCase::from("1, 2")
            .with_semi("low()")
            .with_semi("return");
        assert_eq!(c.to_code(), "case 1, 2 -> {\n    low();\n    return;\n}\n");
    }

    #[test]
    fn case_with_expression() {
        let c: SwitchCase = SwitchCase::from("1").with_expression("RED");
        assert_eq!(c.to_code(), "case 1 -> RED;\n");
    }

    #[test]
    fn case_with_statements_and_expression() {
        let c: SwitchCase = SwitchCase::from("Circle circle")
            .with_semi("check(circle)")
            .with_expression("circle.radius()");
        let mut b: CodeBuffer = CodeBuffer::default();
        c.write(&mut b, 1);
        assert_eq!(
            b.peek(),
            "    case Circle circle -> {\n        check(circle);\n        yield circle.radius();\n    }\n"
        );
    }
}
