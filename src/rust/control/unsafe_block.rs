use crate::CodeBuffer;
use crate::{Statement, WithStatements};

/// An `unsafe` code block.
#[derive(Default)]
pub struct UnsafeBlock {
    statements: Vec<Box<dyn Statement>>,
}

impl WithStatements for UnsafeBlock {
    fn statements(&self) -> &[Box<dyn Statement>] {
        self.statements.as_slice()
    }

    fn add_boxed_statement(&mut self, statement: Box<dyn Statement>) {
        self.statements.push(statement);
    }
}

impl Statement for UnsafeBlock {
    fn write(&self, b: &mut CodeBuffer, level: usize) {
        b.line(level, "unsafe {");
        self.write_statements(b, level + 1);
        b.line(level, "}");
    }
}
