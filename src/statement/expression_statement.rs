use crate::{CodeBuffer, Expression, Statement};

/// A statement that wraps an expression.
pub struct ExpressionStatement<E: Expression> {
    expression: E,
}

impl<E: Expression> From<E> for ExpressionStatement<E> {
    fn from(expression: E) -> Self {
        Self { expression }
    }
}

impl<E: Expression> Statement for ExpressionStatement<E> {
    fn write(&self, b: &mut CodeBuffer, level: usize) {
        b.indent(level);
        self.expression.write(b);
        b.end_line();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Literal;

    #[test]
    fn expression_statement() {
        let stmt = ExpressionStatement::from(Literal::from("hello"));
        assert_eq!(stmt.to_code(), "hello\n");
    }

    #[test]
    fn expression_statement_indented() {
        let stmt = ExpressionStatement::from(Literal::from("hello"));
        let mut b = CodeBuffer::default();
        stmt.write(&mut b, 1);
        assert_eq!(b.peek(), "    hello\n");
    }
}
