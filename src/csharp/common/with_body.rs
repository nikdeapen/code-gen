use crate::{CodeBuffer, WithStatements};

/// An element with statements, written as a body in curly brackets.
///
/// # Note
/// - C# puts each bracket on its own line, which the core's block does not.
pub trait WithBody: WithStatements {
    /// Writes the statements in curly brackets on their own lines. (`level` is the outer level)
    fn write_body(&self, b: &mut CodeBuffer, level: usize) {
        b.line(level, "{");
        self.write_statements(b, level + 1);
        b.line(level, "}");
    }
}

impl<T: WithStatements> WithBody for T {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Block;

    #[test]
    fn empty_body() {
        let mut b: CodeBuffer = CodeBuffer::default();
        Block::default().write_body(&mut b, 0);
        assert_eq!(b.peek(), "{\n}\n");
    }

    #[test]
    fn body_with_statements() {
        let mut b: CodeBuffer = CodeBuffer::default();
        Block::default().with_semi("Run()").write_body(&mut b, 1);
        assert_eq!(b.peek(), "    {\n        Run();\n    }\n");
    }
}
