use crate::csharp::{Var, WithBody};
use crate::{CodeBuffer, Expression, Statement, WithStatements};

/// A `foreach` loop or a `for` loop.
///
/// # Construction
/// - A variable & a collection make a `foreach` loop.
/// - An initializer, a condition & an iterator make a `for` loop.
pub struct ForLoop {
    header: String,
    statements: Vec<Box<dyn Statement>>,
}

impl<V: Into<Var>, S: Into<String>> From<(V, S)> for ForLoop {
    fn from(tuple: (V, S)) -> Self {
        let var: Var = tuple.0.into();
        Self {
            header: format!("foreach ({} in {})", var.to_code(), tuple.1.into()),
            statements: Vec::default(),
        }
    }
}

impl<S0: Into<String>, S1: Into<String>, S2: Into<String>> From<(S0, S1, S2)> for ForLoop {
    fn from(tuple: (S0, S1, S2)) -> Self {
        Self {
            header: format!(
                "for ({}; {}; {})",
                tuple.0.into(),
                tuple.1.into(),
                tuple.2.into()
            ),
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
        b.line(level, self.header.as_str());
        self.write_body(b, level);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn foreach_loop() {
        let l: ForLoop = ForLoop::from((("item", "var"), "items")).with_semi("Add(item)");
        assert_eq!(
            l.to_code(),
            "foreach (var item in items)\n{\n    Add(item);\n}\n"
        );
    }

    #[test]
    fn for_loop() {
        let l: ForLoop = ForLoop::from(("int i = 0", "i < count", "i++")).with_semi("Add(i)");
        let mut b: CodeBuffer = CodeBuffer::default();
        l.write(&mut b, 1);
        assert_eq!(
            b.peek(),
            "    for (int i = 0; i < count; i++)\n    {\n        Add(i);\n    }\n"
        );
    }
}
