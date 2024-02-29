/// An element with an unsafe flag.
pub trait WithUnsafeFlag: Sized {
    /// Gets the unsafe flag.
    fn is_unsafe(&self) -> bool;

    /// Sets the unsafe flag.
    fn with_unsafe(mut self) -> Self {
        self.set_unsafe();
        self
    }

    /// Sets the unsafe flag.
    fn set_unsafe(&mut self);
}
