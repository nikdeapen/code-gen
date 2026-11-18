use crate::rust::Var;
use crate::{Block, CodeBuffer, Expression, Statement, WithStatements};

/// An `unsafe` block of statements, with an optional assignment.
#[derive(Default)]
pub struct UnsafeBlock {
    assignment: Option<Var>,
    block: Block,
}

impl From<Block> for UnsafeBlock {
    fn from(block: Block) -> Self {
        Self {
            assignment: None,
            block,
        }
    }
}

impl UnsafeBlock {
    //! Assignment

    /// Gets the variable the block is assigned to.
    #[must_use]
    pub fn assignment(&self) -> Option<&Var> {
        self.assignment.as_ref()
    }

    /// Sets the `assignment`, the variable the block is assigned to.
    pub fn set_assignment<V>(&mut self, assignment: V)
    where
        V: Into<Var>,
    {
        self.assignment = Some(assignment.into());
    }

    /// Sets the `assignment`, the variable the block is assigned to.
    #[must_use]
    pub fn with_assignment<V>(mut self, assignment: V) -> Self
    where
        V: Into<Var>,
    {
        self.set_assignment(assignment);
        self
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
        if let Some(assignment) = &self.assignment {
            b.write("let ");
            assignment.write(b);
            b.write(" = ");
        }
        b.write("unsafe ");
        self.write_curly_statement_block(b, level);
        if self.assignment.is_some() {
            b.push(';');
        }
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

    #[test]
    fn unsafe_block_with_assignment() {
        let u: UnsafeBlock = UnsafeBlock::default()
            .with_assignment(("value", "u32"))
            .with_semi("let ptr = ptr.add(1)")
            .with_literal("ptr.read()");
        assert_eq!(
            u.to_code(),
            "let value: u32 = unsafe {\n    let ptr = ptr.add(1);\n    ptr.read()\n};\n"
        );
    }

    #[test]
    fn unsafe_block_with_assignment_indented() {
        let u: UnsafeBlock = UnsafeBlock::default()
            .with_assignment(("value", "u32"))
            .with_literal("ptr.read()");
        let mut b: CodeBuffer = CodeBuffer::default();
        u.write(&mut b, 1);
        assert_eq!(
            b.peek(),
            "    let value: u32 = unsafe {\n        ptr.read()\n    };\n"
        );
    }
}
