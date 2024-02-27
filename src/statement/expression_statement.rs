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
    use crate::{CodeBuffer, ExpressionStatement, Literal};

    #[test]
    fn write() {
        let exp: ExpressionStatement<Literal> = Literal::from("expression").into();
        let result: String = CodeBuffer::display_statement(&exp);
        assert_eq!(result, "expression\n");
    }
}
