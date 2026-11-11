use crate::python::WithIndentedBlock;
use crate::{CodeBuffer, Expression, Literal, Statement, WithStatements};

/// A `case` of a `match` statement.
pub struct MatchCase {
    pattern: Box<dyn Expression>,
    statements: Vec<Box<dyn Statement>>,
}

impl<E: 'static + Expression> From<E> for MatchCase {
    fn from(pattern: E) -> Self {
        Self {
            pattern: Box::new(pattern),
            statements: Vec::default(),
        }
    }
}

impl From<&str> for MatchCase {
    fn from(pattern: &str) -> Self {
        Self::from(Literal::from(pattern))
    }
}

impl From<String> for MatchCase {
    fn from(pattern: String) -> Self {
        Self::from(Literal::from(pattern))
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
        b.write("case ");
        self.pattern.write(b);
        b.push(':');
        b.end_line();
        self.write_indented_statement_block(b, level);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_case() {
        let c = MatchCase::from("Circle(radius)").with_literal("return radius");
        assert_eq!(c.to_code(), "case Circle(radius):\n    return radius\n");
    }

    #[test]
    fn empty_case() {
        let c = MatchCase::from(String::from("_"));
        assert_eq!(c.to_code(), "case _:\n    pass\n");
    }

    #[test]
    fn case_indented() {
        let c = MatchCase::from("0").with_literal("zero()");
        let mut b = CodeBuffer::default();
        c.write(&mut b, 1);
        assert_eq!(b.peek(), "    case 0:\n        zero()\n");
    }
}
