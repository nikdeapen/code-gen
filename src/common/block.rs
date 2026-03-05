use crate::{CodeBuffer, Statement, WithStatements};

/// A block of statements.
#[derive(Default)]
pub struct Block {
    statements: Vec<Box<dyn Statement>>,
}

impl From<Block> for Vec<Box<dyn Statement>> {
    fn from(block: Block) -> Self {
        block.statements
    }
}

impl WithStatements for Block {
    fn statements(&self) -> &[Box<dyn Statement>] {
        self.statements.as_slice()
    }

    fn add_boxed_statement(&mut self, statement: Box<dyn Statement>) {
        self.statements.push(statement);
    }
}

impl Statement for Block {
    fn write(&self, b: &mut CodeBuffer, level: usize) {
        self.statements.iter().for_each(|s| s.write(b, level));
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::WithStatements;

    #[test]
    fn empty_block() {
        let block = Block::default();
        assert_eq!(block.to_code(), "");
    }

    #[test]
    fn block_with_statements() {
        let mut block = Block::default();
        block.add_semi("let a = 1");
        block.add_semi("let b = 2");
        assert_eq!(block.to_code(), "let a = 1;\nlet b = 2;\n");
    }

    #[test]
    fn block_with_chaining() {
        let block = Block::default()
            .with_semi("let a = 1")
            .with_semi("let b = 2");
        assert_eq!(block.to_code(), "let a = 1;\nlet b = 2;\n");
    }

    #[test]
    fn block_indented() {
        let mut block = Block::default();
        block.add_semi("x");
        let mut b = CodeBuffer::default();
        block.write(&mut b, 1);
        assert_eq!(b.peek(), "    x;\n");
    }

    #[test]
    fn block_into_vec() {
        let mut block = Block::default();
        block.add_semi("x");
        let stmts: Vec<Box<dyn Statement>> = block.into();
        assert_eq!(stmts.len(), 1);
    }
}
