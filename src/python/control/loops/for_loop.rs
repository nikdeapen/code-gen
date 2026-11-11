use crate::python::WithIndentedBlock;
use crate::{CodeBuffer, Statement, WithStatements};

/// A `for` loop. (ex: `for item in items`)
pub struct ForLoop {
    target: String,
    iterable: String,
    statements: Vec<Box<dyn Statement>>,
}

impl<T: Into<String>, I: Into<String>> From<(T, I)> for ForLoop {
    fn from(tuple: (T, I)) -> Self {
        Self {
            target: tuple.0.into(),
            iterable: tuple.1.into(),
            statements: Vec::default(),
        }
    }
}

impl WithStatements for ForLoop {
    fn statements(&self) -> &[Box<dyn Statement>] {
        self.statements.as_slice()
    }

    fn add_boxed_statement(&mut self, statement: Box<dyn Statement>) {
        self.statements.push(statement);
    }
}

impl Statement for ForLoop {
    fn write(&self, b: &mut CodeBuffer, level: usize) {
        b.indent(level);
        b.write("for ");
        b.write(self.target.as_str());
        b.write(" in ");
        b.write(self.iterable.as_str());
        b.push(':');
        b.end_line();
        self.write_indented_statement_block(b, level);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_for_loop() {
        let l = ForLoop::from(("item", "items")).with_literal("process(item)");
        assert_eq!(l.to_code(), "for item in items:\n    process(item)\n");
    }

    #[test]
    fn empty_for_loop() {
        let l = ForLoop::from(("item", "items"));
        assert_eq!(l.to_code(), "for item in items:\n    pass\n");
    }

    #[test]
    fn for_loop_indented() {
        let l = ForLoop::from(("key, value", "items.items()")).with_literal("process(key, value)");
        let mut b = CodeBuffer::default();
        l.write(&mut b, 1);
        assert_eq!(
            b.peek(),
            "    for key, value in items.items():\n        process(key, value)\n"
        );
    }
}
