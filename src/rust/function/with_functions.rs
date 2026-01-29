use crate::rust::Function;
use crate::{CodeBuffer, Statement};

/// An element with functions.
pub trait WithFunctions: Sized {
    /// Gets the functions.
    fn functions(&self) -> &[Function];

    /// Adds the `function`.
    fn add_function<F>(&mut self, function: F)
    where
        F: Into<Function>;

    /// Adds the `function`.
    fn with_function<F>(mut self, function: F) -> Self
    where
        F: Into<Function>,
    {
        self.add_function(function);
        self
    }

    /// Writes the functions.
    fn write_functions(&self, b: &mut CodeBuffer, level: usize) {
        if let Some((first, rest)) = self.functions().split_first() {
            first.write(b, level);
            for function in rest {
                b.end_line();
                function.write(b, level);
            }
        }
    }
}
