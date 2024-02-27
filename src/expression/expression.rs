use crate::CodeBuffer;

/// Code within a single line.
pub trait Expression {
    /// Writes the code to the buffer.
    fn write(&self, b: &mut CodeBuffer);
}
