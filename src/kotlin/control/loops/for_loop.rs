use crate::{CodeBuffer, Statement, WithStatements};

/// A `for` loop over the elements of an iterable. (ex: `for (x in xs)`)
pub struct ForLoop {
    variable: String,
    iterable: String,
    statements: Vec<Box<dyn Statement>>,
}

impl<S0: Into<String>, S1: Into<String>> From<(S0, S1)> for ForLoop {
    fn from(tuple: (S0, S1)) -> Self {
        Self {
            variable: tuple.0.into(),
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
        b.write("for (");
        b.write(self.variable.as_str());
        b.write(" in ");
        b.write(self.iterable.as_str());
        b.write(") ");
        self.write_curly_statement_block(b, level);
        b.end_line();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_for_loop() {
        let l = ForLoop::from(("x", "xs"));
        assert_eq!(l.to_code(), "for (x in xs) {}\n");
    }

    #[test]
    fn for_loop_with_body() {
        let l = ForLoop::from(("(key, value)", String::from("entries")))
            .with_literal("out.write(key)")
            .with_literal("out.write(value)");
        let mut b = CodeBuffer::default();
        l.write(&mut b, 1);
        assert_eq!(
            b.peek(),
            "    for ((key, value) in entries) {\n        out.write(key)\n        out.write(value)\n    }\n"
        );
    }
}
