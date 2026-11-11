use crate::{CodeBuffer, WithStatements};

/// An element with an indented block of statements.
pub trait WithIndentedBlock: WithStatements {
    /// Writes the indented statement block. (`level` is the outer level)
    ///
    /// # Note
    /// - A block with no statements is written as `pass`, since a block needs a statement.
    fn write_indented_statement_block(&self, b: &mut CodeBuffer, level: usize) {
        if self.statements().is_empty() {
            b.line(level + 1, "pass");
        } else {
            self.write_statements(b, level + 1);
        }
    }
}

impl<T: WithStatements> WithIndentedBlock for T {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Block;

    #[test]
    fn empty_block() {
        let mut b = CodeBuffer::default();
        Block::default().write_indented_statement_block(&mut b, 0);
        assert_eq!(b.peek(), "    pass\n");
    }

    #[test]
    fn block_with_statements() {
        let block = Block::default().with_literal("a = 1").with_literal("b = 2");
        let mut b = CodeBuffer::default();
        block.write_indented_statement_block(&mut b, 1);
        assert_eq!(b.peek(), "        a = 1\n        b = 2\n");
    }
}
