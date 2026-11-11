use crate::{CodeBuffer, Statement, WithName, WithStatements};

/// An include guard around the statements of a header.
///
/// # Note
/// - The name is the guard's macro. (ex: `POINT_H`)
pub struct IncludeGuard {
    name: String,
    statements: Vec<Box<dyn Statement>>,
}

impl<S: Into<String>> From<S> for IncludeGuard {
    fn from(name: S) -> Self {
        Self {
            name: name.into(),
            statements: Vec::default(),
        }
    }
}

impl WithName for IncludeGuard {
    fn name(&self) -> &str {
        self.name.as_str()
    }
}

impl WithStatements for IncludeGuard {
    fn statements(&self) -> &[Box<dyn Statement>] {
        self.statements.as_slice()
    }

    fn add_boxed_statement(&mut self, statement: Box<dyn Statement>) {
        self.statements.push(statement);
    }
}

impl Statement for IncludeGuard {
    fn write(&self, b: &mut CodeBuffer, level: usize) {
        b.indent(level);
        b.write("#ifndef ");
        self.write_name(b);
        b.end_line();
        b.indent(level);
        b.write("#define ");
        self.write_name(b);
        b.end_line();
        b.end_line();
        if !self.statements.is_empty() {
            self.write_statements(b, level);
            b.end_line();
        }
        b.indent(level);
        b.write("#endif // ");
        self.write_name(b);
        b.end_line();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_include_guard() {
        let g: IncludeGuard = IncludeGuard::from("POINT_H");
        assert_eq!(
            g.to_code(),
            "#ifndef POINT_H\n#define POINT_H\n\n#endif // POINT_H\n"
        );
    }

    #[test]
    fn include_guard_with_statements() {
        let g: IncludeGuard = IncludeGuard::from("POINT_H")
            .with_literal("#include <stdint.h>")
            .with_empty_line()
            .with_semi("struct point");
        assert_eq!(
            g.to_code(),
            "#ifndef POINT_H\n#define POINT_H\n\n#include <stdint.h>\n\nstruct point;\n\n#endif // POINT_H\n"
        );
    }

    #[test]
    fn include_guard_indented() {
        let g: IncludeGuard = IncludeGuard::from("POINT_H").with_semi("struct point");
        let mut b: CodeBuffer = CodeBuffer::default();
        g.write(&mut b, 1);
        assert_eq!(
            b.peek(),
            "    #ifndef POINT_H\n    #define POINT_H\n\n    struct point;\n\n    #endif // POINT_H\n"
        );
    }
}
