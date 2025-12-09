use crate::rust::RustType;
use crate::{CodeBuffer, Expression};

/// An element with an optional result.
pub trait WithResult: Sized {
    /// Gets the optional result.
    fn result(&self) -> Option<&RustType>;

    /// Sets the `result`.
    fn set_result<T>(&mut self, result: T)
    where
        T: Into<RustType>;

    /// Sets the `result`.
    fn with_result<T>(mut self, result: T) -> Self
    where
        T: Into<RustType>,
    {
        self.set_result(result);
        self
    }

    /// Writes the optional result. (includes the ` -> ` if the result is not `None`)
    fn write_result(&self, b: &mut CodeBuffer) {
        if let Some(result) = self.result() {
            b.write(" -> ");
            result.write(b);
        }
    }
}
