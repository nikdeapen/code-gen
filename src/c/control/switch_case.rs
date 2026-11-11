use crate::{CodeBuffer, Expression, Literal, Statement, WithStatements};

/// A `switch` statement case.
///
/// # Note
/// - The default value is the `default` case.
/// - The statements are in a block, since a declaration can't follow a label.
/// - A case ends with a `break`, unless it is set not to.
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
        if let Some(expression) = &self.expression {
            b.write("case ");
            expression.write(b);
        } else {
            b.write("default");
        }
        b.write(": {");
        if !self.statements.is_empty() || self.has_break {
            b.end_line();
            self.write_statements(b, level + 1);
            if self.has_break {
                b.line(level + 1, "break;");
            }
            b.indent(level);
        }
        b.push('}');
        b.end_line();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_with_break() {
        let c: SwitchCase = SwitchCase::from("1").with_semi("one()");
        assert!(c.has_break());
        assert_eq!(c.to_code(), "case 1: {\n    one();\n    break;\n}\n");
    }

    #[test]
    fn case_without_break() {
        let c: SwitchCase = SwitchCase::from(String::from("1"))
            .with_semi("return 1")
            .with_break(false);
        assert!(!c.has_break());
        assert_eq!(c.to_code(), "case 1: {\n    return 1;\n}\n");
    }

    #[test]
    fn case_without_statements() {
        let c: SwitchCase = SwitchCase::from("1");
        assert_eq!(c.to_code(), "case 1: {\n    break;\n}\n");
    }

    #[test]
    fn case_without_statements_or_break() {
        let c: SwitchCase = SwitchCase::from("1").with_break(false);
        assert_eq!(c.to_code(), "case 1: {}\n");
    }

    #[test]
    fn default_case() {
        let c: SwitchCase = SwitchCase::default().with_semi("other()");
        assert_eq!(c.to_code(), "default: {\n    other();\n    break;\n}\n");
    }

    #[test]
    fn case_indented() {
        let c: SwitchCase = SwitchCase::from("1").with_semi("one()");
        let mut b: CodeBuffer = CodeBuffer::default();
        c.write(&mut b, 1);
        assert_eq!(
            b.peek(),
            "    case 1: {\n        one();\n        break;\n    }\n"
        );
    }
}
