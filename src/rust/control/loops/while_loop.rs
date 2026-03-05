use crate::{CodeBuffer, Expression, Statement, WithStatements};

/// A `while` loop.
pub struct WhileLoop {
    expression: Box<dyn Expression>,
    statements: Vec<Box<dyn Statement>>,
}

impl<E: 'static + Expression> From<E> for WhileLoop {
    fn from(expression: E) -> Self {
        Self {
            expression: Box::new(expression),
            statements: Vec::default(),
        }
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
        self.expression.write(b);
        b.space();
        self.write_curly_statement_block(b, level);
        b.end_line();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Literal, WithStatements};

    #[test]
    fn simple_while_loop() {
        let l = WhileLoop::from(Literal::from("running")).with_semi("step()");
        assert_eq!(l.to_code(), "while running {\n    step();\n}\n");
    }
}
