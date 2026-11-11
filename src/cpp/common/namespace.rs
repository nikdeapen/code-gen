use crate::{CodeBuffer, Statement, WithName, WithStatements};

/// A namespace block. (ex: `namespace shapes::flat { ... }`)
///
/// # Note
/// - The statements are written at the indent level of the namespace.
pub struct Namespace {
    name: String,
    statements: Vec<Box<dyn Statement>>,
}

impl<S: Into<String>> From<S> for Namespace {
    fn from(name: S) -> Self {
        Self {
            name: name.into(),
            statements: Vec::default(),
        }
    }
}

impl WithName for Namespace {
    fn name(&self) -> &str {
        self.name.as_str()
    }
}

impl WithStatements for Namespace {
    fn statements(&self) -> &[Box<dyn Statement>] {
        self.statements.as_slice()
    }

    fn add_boxed_statement(&mut self, statement: Box<dyn Statement>) {
        self.statements.push(statement);
    }
}

impl Statement for Namespace {
    fn write(&self, b: &mut CodeBuffer, level: usize) {
        b.indent(level);
        b.write("namespace ");
        self.write_name(b);
        b.write(" {");
        if self.statements.is_empty() {
            b.push('}');
            b.end_line();
        } else {
            b.end_line();
            b.end_line();
            self.write_statements(b, level);
            b.end_line();
            b.indent(level);
            b.write("} // namespace ");
            self.write_name(b);
            b.end_line();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_namespace() {
        let n = Namespace::from("shapes");
        assert_eq!(n.to_code(), "namespace shapes {}\n");
    }

    #[test]
    fn namespace_with_statements() {
        let n = Namespace::from("shapes::flat")
            .with_semi("class Point")
            .with_semi("class Line");
        assert_eq!(
            n.to_code(),
            "namespace shapes::flat {\n\nclass Point;\nclass Line;\n\n} // namespace shapes::flat\n"
        );
    }

    #[test]
    fn namespace_indented() {
        let n = Namespace::from("shapes").with_semi("class Point");
        let mut b = CodeBuffer::default();
        n.write(&mut b, 1);
        assert_eq!(
            b.peek(),
            "    namespace shapes {\n\n    class Point;\n\n    } // namespace shapes\n"
        );
    }
}
