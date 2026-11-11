use crate::CodeBuffer;

/// An element with an optional `typedef` name.
///
/// # Note
/// - The name is written after the declaration. (ex: `typedef struct point { ... } point_t;`)
pub trait WithTypedefName: Sized {
    /// Gets the optional `typedef` name.
    fn typedef_name(&self) -> Option<&str>;

    /// Sets the `typedef` name.
    fn set_typedef_name<S>(&mut self, typedef_name: S)
    where
        S: Into<String>;

    /// Sets the `typedef` name.
    #[must_use]
    fn with_typedef_name<S>(mut self, typedef_name: S) -> Self
    where
        S: Into<String>,
    {
        self.set_typedef_name(typedef_name);
        self
    }

    /// Writes the `typedef` keyword with a space after it, if there is a `typedef` name.
    fn write_typedef_keyword(&self, b: &mut CodeBuffer) {
        if self.typedef_name().is_some() {
            b.write("typedef ");
        }
    }

    /// Writes the `typedef` name with a space before it, if there is one.
    fn write_typedef_name(&self, b: &mut CodeBuffer) {
        if let Some(typedef_name) = self.typedef_name() {
            b.space();
            b.write(typedef_name);
        }
    }
}
