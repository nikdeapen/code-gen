use crate::{CodeBuffer, Expression, Literal, Statement, WithStatements};

/// A `case` of a `switch` statement.
///
/// # Note
/// - The default value is the `default` case.
#[derive(Default)]
pub struct SwitchCase {
    expression: Option<Box<dyn Expression>>,
    statements: Vec<Box<dyn Statement>>,
}

impl<E: 'static + Expression> From<E> for SwitchCase {
    fn from(expression: E) -> Self {
        Self {
            expression: Some(Box::new(expression)),
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
        b.push(':');
        b.end_line();
        self.write_statements(b, level + 1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn switch_case() {
        let c = SwitchCase::from("ColorRed").with_literal("return \"Red\"");
        assert_eq!(c.to_code(), "case ColorRed:\n    return \"Red\"\n");
    }

    #[test]
    fn switch_case_without_statements() {
        let c = SwitchCase::from(String::from("1, 2"));
        assert_eq!(c.to_code(), "case 1, 2:\n");
    }

    #[test]
    fn default_case() {
        let c = SwitchCase::default().with_literal("return \"\"");
        assert_eq!(c.to_code(), "default:\n    return \"\"\n");
    }

    #[test]
    fn default_case_without_statements() {
        let c = SwitchCase::default();
        let mut b = CodeBuffer::default();
        c.write(&mut b, 1);
        assert_eq!(b.peek(), "    default:\n");
    }
}
