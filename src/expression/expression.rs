use crate::CodeBuffer;

/// Code within a single line.
pub trait Expression {
    /// Writes the code to the buffer `b`.
    fn write(&self, b: &mut CodeBuffer);
}
