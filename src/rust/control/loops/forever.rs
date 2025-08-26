use crate::{CodeBuffer, Statement, WithStatements};

/// A forever `loops` statement.
#[derive(Default)]
pub struct Loop {
    statements: Vec<Box<dyn Statement>>,
}

impl WithStatements for Loop {
    fn statements(&self) -> &[Box<dyn Statement>] {
        self.statements.as_slice()
    }

    fn add_boxed_statement(&mut self, statement: Box<dyn Statement>) {
        self.statements.push(statement);
    }
}

impl Statement for Loop {
    fn write(&self, b: &mut CodeBuffer, level: usize) {
        b.indent(level);
        b.write("loops ");
        self.write_curly_statement_block(b, level);
        b.end_line();
    }
}
