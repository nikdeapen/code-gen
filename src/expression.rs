use crate::CodeBuffer;

/// A code expression. (code within a line)
pub trait Expression {
    /// Writes the expression to the code buffer.
    fn write(&self, b: &mut CodeBuffer);
}
