use crate::{CodeBuffer, Expression, Literal, Statement, WithStatements};

/// A case of a `switch` statement.
///
/// # Default
/// - The default case is the `default` case of the statement.
///
/// # Note
/// - A case ends in a `break`, unless it is set not to.
/// - A case that ends in a `return` or a `throw` needs no `break`.
/// - A case with no statements & no `break` shares the statements of the next case.
pub struct SwitchCase {
    label: Option<Box<dyn Expression>>,
    statements: Vec<Box<dyn Statement>>,
    has_break: bool,
}

impl<E: 'static + Expression> From<E> for SwitchCase {
    fn from(expression: E) -> Self {
        Self {
            label: Some(Box::new(expression)),
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

impl Default for SwitchCase {
    fn default() -> Self {
        Self {
            label: None,
            statements: Vec::default(),
            has_break: true,
        }
    }
}

impl SwitchCase {
    //! Break

    /// Checks if the case ends in a `break`.
    #[must_use]
    pub fn has_break(&self) -> bool {
        self.has_break
    }

    /// Sets if the case ends in a `break`.
    pub fn set_break(&mut self, has_break: bool) {
        self.has_break = has_break;
    }

    /// Sets if the case ends in a `break`.
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
        match &self.label {
            Some(label) => {
                b.write("case ");
                label.write(b);
            }
            None => b.write("default"),
        }
        b.push(':');
        b.end_line();
        self.write_statements(b, level + 1);
        if self.has_break {
            b.line(level + 1, "break;");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::csharp::Var;

    #[test]
    fn case_with_break() {
        let c: SwitchCase = SwitchCase::from("1").with_semi("Run()");
        assert!(c.has_break());
        assert_eq!(c.to_code(), "case 1:\n    Run();\n    break;\n");
    }

    #[test]
    fn case_without_break() {
        let c: SwitchCase = SwitchCase::from(String::from("Shape.Circle circle"))
            .with_semi("return circle.Value")
            .with_break(false);
        assert!(!c.has_break());
        assert_eq!(
            c.to_code(),
            "case Shape.Circle circle:\n    return circle.Value;\n"
        );
    }

    #[test]
    fn case_from_an_expression() {
        let c: SwitchCase = SwitchCase::from(Literal::from("Color.Red"));
        assert_eq!(c.to_code(), "case Color.Red:\n    break;\n");

        let c: SwitchCase = SwitchCase::from(Var::from(("circle", "Shape.Circle")));
        assert_eq!(c.to_code(), "case Shape.Circle circle:\n    break;\n");
    }

    #[test]
    fn default_case() {
        let c: SwitchCase = SwitchCase::default();
        let mut b: CodeBuffer = CodeBuffer::default();
        c.write(&mut b, 1);
        assert_eq!(b.peek(), "    default:\n        break;\n");
    }

    #[test]
    fn default_case_without_break() {
        let c: SwitchCase = SwitchCase::default()
            .with_semi("return null")
            .with_break(false);
        assert_eq!(c.to_code(), "default:\n    return null;\n");
    }
}
