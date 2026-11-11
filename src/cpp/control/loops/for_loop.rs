use crate::{CodeBuffer, Expression, Literal, Statement, WithStatements};

/// A `for` loop.
///
/// # Note
/// - The expression is the code in the parentheses.
/// - It can be a range. (ex: `const auto& item : items`)
/// - It can be the three clauses. (ex: `std::size_t i = 0; i < size; ++i`)
pub struct ForLoop {
    expression: Box<dyn Expression>,
    statements: Vec<Box<dyn Statement>>,
}

impl<E: 'static + Expression> From<E> for ForLoop {
    fn from(expression: E) -> Self {
        Self {
            expression: Box::new(expression),
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
        self.expression.write(b);
        b.write(") ");
        self.write_curly_statement_block(b, level);
        b.end_line();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn range_for_loop() {
        let l = ForLoop::from("const auto& item : items").with_semi("write(item)");
        assert_eq!(
            l.to_code(),
            "for (const auto& item : items) {\n    write(item);\n}\n"
        );
    }

    #[test]
    fn classic_for_loop() {
        let l = ForLoop::from(String::from("std::size_t i = 0; i < size; ++i"))
            .with_semi("write(items[i])");
        let mut b = CodeBuffer::default();
        l.write(&mut b, 1);
        assert_eq!(
            b.peek(),
            "    for (std::size_t i = 0; i < size; ++i) {\n        write(items[i]);\n    }\n"
        );
    }

    #[test]
    fn empty_for_loop() {
        let l = ForLoop::from("const auto& item : items");
        assert_eq!(l.to_code(), "for (const auto& item : items) {}\n");
    }
}
