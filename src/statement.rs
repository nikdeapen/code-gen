use crate::CodeBuffer;

/// A code statement. (code that spans one or more lines)
pub trait Statement {
    /// Writes the statement to the code buffer at the indent level.
    fn write(&self, b: &mut CodeBuffer, level: usize);
}
