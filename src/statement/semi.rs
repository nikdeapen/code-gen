use crate::{CodeBuffer, Expression, Statement};

/// A semi-colon ended expression statement.
pub struct Semi<E: Expression> {
    expression: E,
}

impl<E: Expression> From<E> for Semi<E> {
    fn from(expression: E) -> Self {
        Self { expression }
    }
}

impl<E: Expression> Statement for Semi<E> {
    fn write(&self, b: &mut CodeBuffer, level: usize) {
        b.indent(level);
        self.expression.write(b);
        b.write(";");
        b.end_line();
    }
}

#[cfg(test)]
mod tests {
    use crate::{CodeBuffer, Literal, Semi};

    #[test]
    fn write() {
        let semi: Semi<Literal> = Literal::from("expression").into();
        let result: String = CodeBuffer::display_statement(&semi);
        assert_eq!(result, "expression;\n");
    }
}
