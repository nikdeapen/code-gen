use crate::java::Function;

/// An element with functions.
pub trait WithFunctions: Sized {
    /// Gets the functions.
    fn functions(&self) -> &[Function];

    /// Adds a function.
    fn add_function<F>(&mut self, function: F)
    where
        F: Into<Function>;

    /// Adds a function.
    #[must_use]
    fn with_function<F>(mut self, function: F) -> Self
    where
        F: Into<Function>,
    {
        self.add_function(function);
        self
    }
}
