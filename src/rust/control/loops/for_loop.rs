use crate::{CodeBuffer, Statement, WithStatements};

/// A `for` loop over the elements of an iterator. (ex: `for x in xs`)
pub struct ForLoop {
    pattern: String,
    iterator: String,
    statements: Vec<Box<dyn Statement>>,
}

impl<S0: Into<String>, S1: Into<String>> From<(S0, S1)> for ForLoop {
    fn from(tuple: (S0, S1)) -> Self {
        Self {
            pattern: tuple.0.into(),
            iterator: tuple.1.into(),
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
        b.write(self.pattern.as_str());
        b.write(" in ");
        b.write(self.iterator.as_str());
        b.space();
        self.write_curly_statement_block(b, level);
        b.end_line();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rust::{MatchCase, MatchStatement};

    #[test]
    fn empty_for_loop() {
        let l: ForLoop = ForLoop::from(("x", "xs"));
        assert_eq!(l.to_code(), "for x in xs {}\n");
    }

    #[test]
    fn for_loop_with_body() {
        let l: ForLoop = ForLoop::from(("(key, value)", String::from("entries.iter()")))
            .with_semi("target.write(key)")
            .with_semi("target.write(value)");
        let mut b: CodeBuffer = CodeBuffer::default();
        l.write(&mut b, 1);
        assert_eq!(
            b.peek(),
            "    for (key, value) in entries.iter() {\n        target.write(key);\n        target.write(value);\n    }\n"
        );
    }

    #[test]
    fn for_loop_with_a_match() {
        let l: ForLoop = ForLoop::from(("field", "fields")).with_statement(
            MatchStatement::from("field.tag()")
                .with_match_case(MatchCase::from("1").with_expression("message.id = field.id()"))
                .with_match_case(MatchCase::from("_").with_expression("message.keep(field)")),
        );
        assert_eq!(
            l.to_code(),
            "for field in fields {\n    match field.tag() {\n        1 => message.id = field.id(),\n        _ => message.keep(field),\n    }\n}\n"
        );
    }
}
