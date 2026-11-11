use crate::{CodeBuffer, Expression, Literal, Statement, WithStatements};

/// A `while` loop.
pub struct WhileLoop {
    condition: Box<dyn Expression>,
    statements: Vec<Box<dyn Statement>>,
}

impl<E: 'static + Expression> From<E> for WhileLoop {
    fn from(expression: E) -> Self {
        Self {
            condition: Box::new(expression),
            statements: Vec::default(),
        }
    }
}

impl From<&str> for WhileLoop {
    fn from(literal: &str) -> Self {
        Self::from(Literal::from(literal))
    }
}

impl From<String> for WhileLoop {
    fn from(literal: String) -> Self {
        Self::from(Literal::from(literal))
    }
}

impl WithStatements for WhileLoop {
    fn statements(&self) -> &[Box<dyn Statement>] {
        self.statements.as_slice()
    }

    fn add_boxed_statement(&mut self, statement: Box<dyn Statement>) {
        self.statements.push(statement);
    }
}

impl Statement for WhileLoop {
    fn write(&self, b: &mut CodeBuffer, level: usize) {
        b.indent(level);
        b.write("while (");
        self.condition.write(b);
        b.write(") ");
        self.write_curly_statement_block(b, level);
        b.end_line();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_while_loop() {
        let l: WhileLoop = WhileLoop::from(String::from("next()"));
        assert_eq!(l.to_code(), "while (next()) {}\n");
    }

    #[test]
    fn while_loop_with_body() {
        let l: WhileLoop = WhileLoop::from("offset < len").with_semi("offset += step()");
        assert_eq!(
            l.to_code(),
            "while (offset < len) {\n    offset += step();\n}\n"
        );
    }

    #[test]
    fn while_loop_indented() {
        let l: WhileLoop = WhileLoop::from("offset < len").with_semi("offset += step()");
        let mut b: CodeBuffer = CodeBuffer::default();
        l.write(&mut b, 1);
        assert_eq!(
            b.peek(),
            "    while (offset < len) {\n        offset += step();\n    }\n"
        );
    }
}
