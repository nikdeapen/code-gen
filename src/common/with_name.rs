use crate::CodeBuffer;

/// An element with a name.
pub trait WithName {
    /// Gets the name.
    fn name(&self) -> &str;

    /// Writes the name.
    fn write_name(&self, b: &mut CodeBuffer) {
        b.write(self.name());
    }
}
