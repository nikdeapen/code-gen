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
