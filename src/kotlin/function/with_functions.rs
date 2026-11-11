use crate::kotlin::Function;

/// An element with functions.
pub trait WithFunctions: Sized {
    /// Gets the functions.
    fn functions(&self) -> &[Function];

    /// Adds the function.
    fn add_function<F>(&mut self, function: F)
    where
        F: Into<Function>;

    /// Adds the function.
    #[must_use]
    fn with_function<F>(mut self, function: F) -> Self
    where
        F: Into<Function>,
    {
        self.add_function(function);
        self
    }
}
