use crate::{CodeBuffer, Expression, Literal, Statement, WithStatements};

/// A `while` loop.
pub struct WhileLoop {
    condition: Box<dyn Expression>,
    statements: Vec<Box<dyn Statement>>,
}

impl<E: 'static + Expression> From<E> for WhileLoop {
    fn from(condition: E) -> Self {
        Self {
            condition: Box::new(condition),
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
    fn simple_while_loop() {
        let l: WhileLoop = WhileLoop::from("in.hasRemaining()").with_semi("read(in)");
        assert_eq!(
            l.to_code(),
            "while (in.hasRemaining()) {\n    read(in);\n}\n"
        );
    }

    #[test]
    fn while_loop_from_a_string() {
        let l: WhileLoop = WhileLoop::from(String::from("in.hasRemaining()")).with_semi("read(in)");
        assert_eq!(
            l.to_code(),
            "while (in.hasRemaining()) {\n    read(in);\n}\n"
        );
    }

    #[test]
    fn while_loop_from_an_expression() {
        let l: WhileLoop =
            WhileLoop::from(Literal::from("in.hasRemaining()")).with_semi("read(in)");
        assert_eq!(
            l.to_code(),
            "while (in.hasRemaining()) {\n    read(in);\n}\n"
        );
    }

    #[test]
    fn empty_while_loop() {
        let l: WhileLoop = WhileLoop::from("step()");
        assert_eq!(l.to_code(), "while (step()) {}\n");
    }
}
