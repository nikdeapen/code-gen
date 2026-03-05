use crate::{CodeBuffer, Expression, Literal, Statement};

/// A semicolon ended expression statement.
pub struct Semi<E: Expression> {
    expression: E,
}

impl<E: Expression> From<E> for Semi<E> {
    fn from(expression: E) -> Self {
        Self { expression }
    }
}

impl From<&str> for Semi<Literal> {
    fn from(value: &str) -> Self {
        Self::from(Literal::from(value))
    }
}

impl From<String> for Semi<Literal> {
    fn from(value: String) -> Self {
        Self::from(Literal::from(value))
    }
}

impl<E: Expression> Statement for Semi<E> {
    fn write(&self, b: &mut CodeBuffer, level: usize) {
        b.indent(level);
        self.expression.write(b);
        b.push(';');
        b.end_line();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn semi_from_str() {
        let semi = Semi::from("let x = 1");
        assert_eq!(semi.to_code(), "let x = 1;\n");
    }

    #[test]
    fn semi_from_string() {
        let semi = Semi::from(String::from("let x = 1"));
        assert_eq!(semi.to_code(), "let x = 1;\n");
    }

    #[test]
    fn semi_indented() {
        let semi = Semi::from("x");
        let mut b = CodeBuffer::default();
        semi.write(&mut b, 2);
        assert_eq!(b.peek(), "        x;\n");
    }
}
