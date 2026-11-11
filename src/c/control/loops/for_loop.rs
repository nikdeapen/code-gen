use crate::{CodeBuffer, Statement, WithStatements};

/// A `for` loop.
///
/// # Note
/// - It is created from its three clauses. (ex: `size_t i = 0`, `i < len` & `i++`)
pub struct ForLoop {
    init: String,
    condition: String,
    step: String,
    statements: Vec<Box<dyn Statement>>,
}

impl<S0: Into<String>, S1: Into<String>, S2: Into<String>> From<(S0, S1, S2)> for ForLoop {
    fn from(clauses: (S0, S1, S2)) -> Self {
        Self {
            init: clauses.0.into(),
            condition: clauses.1.into(),
            step: clauses.2.into(),
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
        b.write(self.init.as_str());
        b.write("; ");
        b.write(self.condition.as_str());
        b.write("; ");
        b.write(self.step.as_str());
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
        let l: ForLoop = ForLoop::from(("size_t i = 0", "i < len", "i++"));
        assert_eq!(l.to_code(), "for (size_t i = 0; i < len; i++) {}\n");
    }

    #[test]
    fn for_loop_with_body() {
        let l: ForLoop = ForLoop::from(("size_t i = 0", "i < len", "i++")).with_semi("step(i)");
        assert_eq!(
            l.to_code(),
            "for (size_t i = 0; i < len; i++) {\n    step(i);\n}\n"
        );
    }

    #[test]
    fn for_loop_indented() {
        let l: ForLoop = ForLoop::from(("size_t i = 0", "i < len", "i++")).with_semi("step(i)");
        let mut b: CodeBuffer = CodeBuffer::default();
        l.write(&mut b, 1);
        assert_eq!(
            b.peek(),
            "    for (size_t i = 0; i < len; i++) {\n        step(i);\n    }\n"
        );
    }
}
