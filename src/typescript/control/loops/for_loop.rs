use crate::{CodeBuffer, Expression, Literal, Statement, WithStatements};

/// A `for` loop.
///
/// # Note
/// - The header is the code in the parentheses. (ex: `const x of xs` or `let i = 0; i < n; i++`)
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
    fn for_of_loop() {
        let l = ForLoop::from("const x of xs").with_semi("visit(x)");
        assert_eq!(l.to_code(), "for (const x of xs) {\n    visit(x);\n}\n");
    }

    #[test]
    fn classic_for_loop() {
        let l = ForLoop::from(String::from("let i = 0; i < n; i++"));
        assert_eq!(l.to_code(), "for (let i = 0; i < n; i++) {}\n");
    }
}
