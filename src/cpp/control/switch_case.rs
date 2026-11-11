use crate::{CodeBuffer, Expression, Literal, Statement, WithStatements};

/// A `switch` statement case.
///
/// # Note
/// - The default value is the `default` case.
/// - The statements are in braces. A case can then declare a variable.
/// - The case ends with a `break`, unless it is told not to. (ex: a case that returns)
pub struct SwitchCase {
    expression: Option<Box<dyn Expression>>,
    statements: Vec<Box<dyn Statement>>,
    has_break: bool,
}

impl Default for SwitchCase {
    fn default() -> Self {
        Self {
            expression: None,
            statements: Vec::default(),
            has_break: true,
        }
    }
}

impl<E: 'static + Expression> From<E> for SwitchCase {
    fn from(expression: E) -> Self {
        Self {
            expression: Some(Box::new(expression)),
            statements: Vec::default(),
            has_break: true,
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

impl WithStatements for SwitchCase {
    fn statements(&self) -> &[Box<dyn Statement>] {
        self.statements.as_slice()
    }

    fn add_boxed_statement(&mut self, statement: Box<dyn Statement>) {
        self.statements.push(statement);
    }
}

impl SwitchCase {
    //! Break

    /// Checks if the case ends with a `break`.
    #[must_use]
    pub fn has_break(&self) -> bool {
        self.has_break
    }

    /// Sets if the case ends with a `break`.
    pub fn set_break(&mut self, has_break: bool) {
        self.has_break = has_break;
    }

    /// Sets if the case ends with a `break`.
    #[must_use]
    pub fn with_break(mut self, has_break: bool) -> Self {
        self.set_break(has_break);
        self
    }
}

impl Statement for SwitchCase {
    fn write(&self, b: &mut CodeBuffer, level: usize) {
        b.indent(level);
        match &self.expression {
            Some(expression) => {
                b.write("case ");
                expression.write(b);
            }
            None => b.write("default"),
        }
        b.write(": {");
        b.end_line();
        self.write_statements(b, level + 1);
        if self.has_break {
            b.line(level + 1, "break;");
        }
        b.line(level, "}");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_with_break() {
        let c = SwitchCase::from("1").with_semi("one()");
        assert!(c.has_break());
        assert_eq!(c.to_code(), "case 1: {\n    one();\n    break;\n}\n");
    }

    #[test]
    fn case_without_break() {
        let c = SwitchCase::from(String::from("Color::Red"))
            .with_semi("return 1")
            .with_break(false);
        assert!(!c.has_break());
        let mut b = CodeBuffer::default();
        c.write(&mut b, 1);
        assert_eq!(
            b.peek(),
            "    case Color::Red: {\n        return 1;\n    }\n"
        );
    }

    #[test]
    fn empty_case() {
        let c = SwitchCase::from("0");
        assert_eq!(c.to_code(), "case 0: {\n    break;\n}\n");
    }

    #[test]
    fn default_case() {
        let c = SwitchCase::default().with_semi("other()");
        assert!(c.has_break());
        assert_eq!(c.to_code(), "default: {\n    other();\n    break;\n}\n");
    }

    #[test]
    fn default_case_without_break() {
        let c = SwitchCase::default()
            .with_semi("return 0")
            .with_break(false);
        let mut b = CodeBuffer::default();
        c.write(&mut b, 1);
        assert_eq!(b.peek(), "    default: {\n        return 0;\n    }\n");
    }
}
