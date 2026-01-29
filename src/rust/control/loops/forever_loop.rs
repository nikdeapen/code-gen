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

impl Statement for ForeverLoop {
    fn write(&self, b: &mut CodeBuffer, level: usize) {
        b.indent(level);
        b.write("loops ");
        self.write_curly_statement_block(b, level);
        b.end_line();
    }
}
