use crate::CodeBuffer;

/// Code that spans one or more lines.
pub trait Statement {
    /// Writes the code to the buffer `b` at the indent `level`.
    fn write(&self, b: &mut CodeBuffer, level: usize);

    /// Renders the statement to a string at indent level 0.
    fn to_code(&self) -> String {
        let mut b = CodeBuffer::default();
        self.write(&mut b, 0);
        b.into()
    }
}
