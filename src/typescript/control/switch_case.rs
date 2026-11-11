use crate::{CodeBuffer, Expression, Literal, Statement, WithStatements};

/// A `switch` statement case.
///
/// # Note
/// - The default value is the `default` clause.
pub struct SwitchCase {
    expression: Option<Box<dyn Expression>>,
    has_braces: bool,
    has_break: bool,
    statements: Vec<Box<dyn Statement>>,
}

impl Default for SwitchCase {
    fn default() -> Self {
        Self {
            expression: None,
            has_braces: false,
            has_break: true,
            statements: Vec::default(),
        }
    }
}

impl<E: 'static + Expression> From<E> for SwitchCase {
    fn from(expression: E) -> Self {
        Self {
            expression: Some(Box::new(expression)),
            ..Self::default()
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
    //! Braces

    /// Checks if the statements are in braces.
    ///
    /// # Note
    /// - Braces give the statements a scope of their own, for a case that declares variables.
    #[must_use]
    pub fn has_braces(&self) -> bool {
        self.has_braces
    }

    /// Sets the braces flag.
    pub fn set_braces(&mut self, has_braces: bool) {
        self.has_braces = has_braces;
    }

    /// Sets the braces flag.
    #[must_use]
    pub fn with_braces(mut self, has_braces: bool) -> Self {
        self.set_braces(has_braces);
        self
    }
}

impl SwitchCase {
    //! Break

    /// Checks if the case ends with a `break`.
    ///
    /// # Note
    /// - A new case has one. A case that returns or falls through to the next case has none.
    #[must_use]
    pub fn has_break(&self) -> bool {
        self.has_break
    }

    /// Sets the break flag.
    pub fn set_break(&mut self, has_break: bool) {
        self.has_break = has_break;
    }

    /// Sets the break flag.
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

impl SwitchCase {
    //! Write

    /// Writes the statements & the optional `break`.
    fn write_body(&self, b: &mut CodeBuffer, level: usize) {
        self.write_statements(b, level);
        if self.has_break {
            b.line(level, "break;");
        }
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
        b.push(':');
        if !self.has_braces {
            b.end_line();
            self.write_body(b, level + 1);
        } else if self.statements.is_empty() && !self.has_break {
            b.write(" {}");
            b.end_line();
        } else {
            b.write(" {");
            b.end_line();
            self.write_body(b, level + 1);
            b.line(level, "}");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_with_statements() {
        let c = SwitchCase::from("1").with_semi("one()");
        assert_eq!(c.to_code(), "case 1:\n    one();\n    break;\n");
        assert!(c.has_break());
    }

    #[test]
    fn case_without_statements() {
        let c = SwitchCase::from(String::from("\"circle\""));
        assert_eq!(c.to_code(), "case \"circle\":\n    break;\n");
    }

    #[test]
    fn case_without_break() {
        let c = SwitchCase::from("1")
            .with_break(false)
            .with_semi("return one()");
        assert_eq!(c.to_code(), "case 1:\n    return one();\n");
        assert!(!c.has_break());
    }

    #[test]
    fn case_that_falls_through() {
        let c = SwitchCase::from("1").with_break(false);
        assert_eq!(c.to_code(), "case 1:\n");
    }

    #[test]
    fn default_case() {
        let c = SwitchCase::default().with_semi("other()");
        assert_eq!(c.to_code(), "default:\n    other();\n    break;\n");
    }

    #[test]
    fn default_case_without_break() {
        let c = SwitchCase::default()
            .with_break(false)
            .with_semi("return other()");
        assert_eq!(c.to_code(), "default:\n    return other();\n");
    }

    #[test]
    fn case_with_braces() {
        let c = SwitchCase::from("1")
            .with_braces(true)
            .with_semi("const one = 1")
            .with_semi("visit(one)");
        let mut b = CodeBuffer::default();
        c.write(&mut b, 1);
        assert_eq!(
            b.peek(),
            "    case 1: {\n        const one = 1;\n        visit(one);\n        break;\n    }\n"
        );
        assert!(c.has_braces());
    }

    #[test]
    fn case_with_braces_without_break() {
        let c = SwitchCase::from("1")
            .with_braces(true)
            .with_break(false)
            .with_semi("const one = 1")
            .with_semi("return one");
        assert_eq!(
            c.to_code(),
            "case 1: {\n    const one = 1;\n    return one;\n}\n"
        );
    }

    #[test]
    fn case_with_braces_without_statements() {
        let c = SwitchCase::default().with_braces(true);
        assert_eq!(c.to_code(), "default: {\n    break;\n}\n");
        assert_eq!(c.with_break(false).to_code(), "default: {}\n");
    }
}
