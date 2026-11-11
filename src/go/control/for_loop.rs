use crate::{CodeBuffer, Expression, Literal, Statement, WithStatements};

/// A `for` loop.
///
/// # Note
/// - The default loop has no clause. It runs forever.
/// - The clause is written as given. (ex: `len(bytes) > 0` or `_, value := range values`)
#[derive(Default)]
pub struct ForLoop {
    clause: Option<Box<dyn Expression>>,
    statements: Vec<Box<dyn Statement>>,
}

impl<E: 'static + Expression> From<E> for ForLoop {
    fn from(clause: E) -> Self {
        Self {
            clause: Some(Box::new(clause)),
            statements: Vec::default(),
        }
    }
}

impl From<&str> for ForLoop {
    fn from(literal: &str) -> Self {
        Self::from(Literal::from(literal))
    }
}

impl From<String> for ForLoop {
    fn from(literal: String) -> Self {
        Self::from(Literal::from(literal))
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
        if let Some(clause) = &self.clause {
            clause.write(b);
            b.space();
        }
        b.push('{');
        b.end_line();
        self.write_statements(b, level + 1);
        b.line(level, "}");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_forever_loop() {
        let l = ForLoop::default();
        assert_eq!(l.to_code(), "for {\n}\n");
    }

    #[test]
    fn forever_loop_with_body() {
        let l = ForLoop::default().with_literal("step()");
        assert_eq!(l.to_code(), "for {\n    step()\n}\n");
    }

    #[test]
    fn condition_loop() {
        let l = ForLoop::from("len(bytes) > 0").with_literal("bytes = bytes[1:]");
        assert_eq!(
            l.to_code(),
            "for len(bytes) > 0 {\n    bytes = bytes[1:]\n}\n"
        );
    }

    #[test]
    fn range_loop() {
        let l =
            ForLoop::from(String::from("_, value := range values")).with_literal("sum += value");
        assert_eq!(
            l.to_code(),
            "for _, value := range values {\n    sum += value\n}\n"
        );
    }

    #[test]
    fn counting_loop() {
        let l = ForLoop::from("i := 0; i < 4; i++").with_literal("step(i)");
        let mut b = CodeBuffer::default();
        l.write(&mut b, 1);
        assert_eq!(
            b.peek(),
            "    for i := 0; i < 4; i++ {\n        step(i)\n    }\n"
        );
    }
}
