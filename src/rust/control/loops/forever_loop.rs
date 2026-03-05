use crate::{CodeBuffer, Statement, WithStatements};

/// A forever `loop` statement.
#[derive(Default)]
pub struct ForeverLoop {
    statements: Vec<Box<dyn Statement>>,
}

impl WithStatements for ForeverLoop {
    fn statements(&self) -> &[Box<dyn Statement>] {
        self.statements.as_slice()
    }

    fn add_boxed_statement(&mut self, statement: Box<dyn Statement>) {
        self.statements.push(statement);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_forever_loop() {
        let l = ForeverLoop::default();
        assert_eq!(l.to_code(), "loop {}\n");
    }

    #[test]
    fn forever_loop_with_body() {
        let l = ForeverLoop::default().with_semi("process()");
        assert_eq!(l.to_code(), "loop {\n    process();\n}\n");
    }
}

impl Statement for ForeverLoop {
    fn write(&self, b: &mut CodeBuffer, level: usize) {
        b.indent(level);
        b.write("loop ");
        self.write_curly_statement_block(b, level);
        b.end_line();
    }
}
