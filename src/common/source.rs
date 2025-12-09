use crate::{CodeBuffer, Statement, WithStatements};

/// Represents general source code.
#[derive(Default)]
pub struct Source {
    statements: Vec<Box<dyn Statement>>,
}

impl From<Source> for Vec<Box<dyn Statement>> {
    fn from(source: Source) -> Self {
        source.statements
    }
}

impl WithStatements for Source {
    fn statements(&self) -> &[Box<dyn Statement>] {
        self.statements.as_slice()
    }

    fn add_boxed_statement(&mut self, statement: Box<dyn Statement>) {
        self.statements.push(statement)
    }
}

impl Statement for Source {
    fn write(&self, b: &mut CodeBuffer, level: usize) {
        self.statements.iter().for_each(|s| s.write(b, level))
    }
}
