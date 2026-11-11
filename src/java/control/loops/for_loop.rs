use crate::{CodeBuffer, Expression, Literal, Statement, WithStatements};

/// A `for` loop.
///
/// # Note
/// - The header is the code in the parentheses. (ex: `int score : scores`)
pub struct ForLoop {
    header: Box<dyn Expression>,
    statements: Vec<Box<dyn Statement>>,
}

impl<E: 'static + Expression> From<E> for ForLoop {
    fn from(header: E) -> Self {
        Self {
            header: Box::new(header),
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
        b.write("for (");
        self.header.write(b);
        b.write(") ");
        self.write_curly_statement_block(b, level);
        b.end_line();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn for_each_loop() {
        let l: ForLoop = ForLoop::from("int score : scores").with_semi("out.putInt(score)");
        assert_eq!(
            l.to_code(),
            "for (int score : scores) {\n    out.putInt(score);\n}\n"
        );
    }

    #[test]
    fn counting_for_loop() {
        let l: ForLoop =
            ForLoop::from(String::from("int i = 0; i < 4; i++")).with_semi("out.putInt(i)");
        assert_eq!(
            l.to_code(),
            "for (int i = 0; i < 4; i++) {\n    out.putInt(i);\n}\n"
        );
    }

    #[test]
    fn for_loop_from_an_expression() {
        let l: ForLoop =
            ForLoop::from(Literal::from("int score : scores")).with_semi("out.putInt(score)");
        assert_eq!(
            l.to_code(),
            "for (int score : scores) {\n    out.putInt(score);\n}\n"
        );
    }

    #[test]
    fn empty_for_loop() {
        let l: ForLoop = ForLoop::from("int score : scores");
        assert_eq!(l.to_code(), "for (int score : scores) {}\n");
    }
}
