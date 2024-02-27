use crate::CodeBuffer;

/// An element with derived traits.
pub trait WithDerives: Sized {
    /// Gets the derives.
    fn derives(&self) -> &[String];

    /// Adds the derive.
    fn with_derive<S>(mut self, derive: S) -> Self
    where
        S: Into<String>,
    {
        self.add_derive(derive);
        self
    }

    /// Adds the derive.
    fn add_derive<S>(&mut self, derive: S)
    where
        S: Into<String>;

    /// Writes the derives.
    fn write_derives(&self, b: &mut CodeBuffer, level: usize) {
        if let Some((first, rest)) = self.derives().split_first() {
            b.indent(level);
            b.write("#[derive(");
            b.write(first.as_str());
            for derive in rest {
                b.write(", ");
                b.write(derive.as_str());
            }
            b.write(")]");
            b.end_line();
        }
    }
}
