use crate::{Block, CodeBuffer, Statement, WithStatements};

/// An `unsafe` block.
#[derive(Default)]
pub struct UnsafeBlock {
    block: Block,
}

impl From<Block> for UnsafeBlock {
    fn from(block: Block) -> Self {
        Self { block }
    }
}

impl WithStatements for UnsafeBlock {
    fn statements(&self) -> &[Box<dyn Statement>] {
        self.block.statements()
    }

    fn add_boxed_statement(&mut self, statement: Box<dyn Statement>) {
        self.block.add_boxed_statement(statement);
    }
}

impl Statement for UnsafeBlock {
    fn write(&self, b: &mut CodeBuffer, level: usize) {
        b.indent(level);
        b.write("unsafe ");
        self.write_curly_statement_block(b, level);
        b.end_line();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_unsafe_block() {
        let u: UnsafeBlock = UnsafeBlock::default();
        assert_eq!(u.to_code(), "unsafe {}\n");
    }

    #[test]
    fn unsafe_block_with_body() {
        let u: UnsafeBlock = UnsafeBlock::default().with_semi("ptr.write(value)");
        assert_eq!(u.to_code(), "unsafe {\n    ptr.write(value);\n}\n");
    }

    #[test]
    fn unsafe_block_from_block() {
        let block: Block = Block::default().with_semi("a()").with_semi("b()");
        let u: UnsafeBlock = UnsafeBlock::from(block);
        assert_eq!(u.to_code(), "unsafe {\n    a();\n    b();\n}\n");
    }

    #[test]
    fn unsafe_block_indented() {
        let u: UnsafeBlock = UnsafeBlock::default().with_semi("x");
        let mut b: CodeBuffer = CodeBuffer::default();
        u.write(&mut b, 1);
        assert_eq!(b.peek(), "    unsafe {\n        x;\n    }\n");
    }
}
