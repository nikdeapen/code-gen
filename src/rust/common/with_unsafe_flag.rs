use crate::CodeBuffer;

/// An element with an unsafe flag.
pub trait WithUnsafeFlag: Sized {
    /// Gets the unsafe flag.
    fn is_unsafe(&self) -> bool;

    /// Sets the unsafe flag.
    fn set_unsafe(&mut self, is_unsafe: bool);

    /// Sets the unsafe flag.
    #[must_use]
    fn with_unsafe(mut self, is_unsafe: bool) -> Self {
        self.set_unsafe(is_unsafe);
        self
    }

    /// Writes the optional unsafe flag.
    fn write_unsafe(&self, b: &mut CodeBuffer) {
        if self.is_unsafe() {
            b.write("unsafe ");
        }
    }
}
