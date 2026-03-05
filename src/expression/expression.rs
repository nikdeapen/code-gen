use crate::CodeBuffer;

/// Code within a single line.
pub trait Expression {
    /// Writes the code to the buffer `b`.
    fn write(&self, b: &mut CodeBuffer);

    /// Renders the expression to a string.
    fn to_code(&self) -> String {
        let mut b = CodeBuffer::default();
        self.write(&mut b);
        b.into()
    }
}
