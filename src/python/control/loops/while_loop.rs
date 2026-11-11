use crate::python::WithIndentedBlock;
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
    fn from(condition: &str) -> Self {
        Self::from(Literal::from(condition))
    }
}

impl From<String> for WhileLoop {
    fn from(condition: String) -> Self {
        Self::from(Literal::from(condition))
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
        b.write("while ");
        self.condition.write(b);
        b.push(':');
        b.end_line();
        self.write_indented_statement_block(b, level);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_while_loop() {
        let l = WhileLoop::from("running").with_literal("step()");
        assert_eq!(l.to_code(), "while running:\n    step()\n");
    }

    #[test]
    fn empty_while_loop() {
        let l = WhileLoop::from(String::from("True"));
        assert_eq!(l.to_code(), "while True:\n    pass\n");
    }

    #[test]
    fn while_loop_indented() {
        let l = WhileLoop::from("running").with_literal("step()");
        let mut b = CodeBuffer::default();
        l.write(&mut b, 1);
        assert_eq!(b.peek(), "    while running:\n        step()\n");
    }
}
