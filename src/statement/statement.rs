use crate::CodeBuffer;

/// Code that spans one or more lines.
pub trait Statement {
    /// Writes the code to the buffer `b` at the indent `level`.
    fn write(&self, b: &mut CodeBuffer, level: usize);
}
