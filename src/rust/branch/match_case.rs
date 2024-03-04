use crate::{CodeBuffer, Statement, WithStatements};

/// A match statement case.
pub struct MatchCase {
    case: String,
    statements: Vec<Box<dyn Statement>>,
}

impl<S: Into<String>> From<S> for MatchCase {
    fn from(case: S) -> Self {
        Self {
            case: case.into(),
            statements: Vec::default(),
        }
    }
}

impl WithStatements for MatchCase {
    fn statements(&self) -> &[Box<dyn Statement>] {
        self.statements.as_slice()
    }

    fn add_boxed_statement(&mut self, statement: Box<dyn Statement>) {
        self.statements.push(statement);
    }
}

impl Statement for MatchCase {
    fn write(&self, b: &mut CodeBuffer, level: usize) {
        b.indent(level);
        b.write(self.case.as_str());
        b.write(" => ");
        self.write_curly_statement_block(b, level);
        b.end_line();
    }
}

#[cfg(test)]
mod tests {
    use crate::rust::MatchCase;
    use crate::{CodeBuffer, Literal, WithStatements};

    #[test]
    fn write() {
        let match_case: MatchCase = MatchCase::from("Some(1)");
        let result: String = CodeBuffer::display_statement(&match_case);
        assert_eq!(result, "Some(1) -> {}\n");

        let match_case: MatchCase = match_case.with_expression_statement(Literal::from("one"));
        let result: String = CodeBuffer::display_statement(&match_case);
        assert_eq!(result, "Some(1) -> {\n\tone\n}\n");

        let match_case: MatchCase = match_case.with_expression_statement(Literal::from("two"));
        let result: String = CodeBuffer::display_statement(&match_case);
        assert_eq!(result, "Some(1) -> {\n\tone\n\ttwo\n}\n");
    }
}
