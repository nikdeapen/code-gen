use crate::CodeBuffer;

pub trait WithDerives: Sized {
    /// Gets the derives.
    fn derives(&self) -> &[String];

    /// Adds the derivation.
    fn add_derive<S>(&mut self, derive: S)
    where
        S: Into<String>;

    /// Adds the derivation.
    fn with_derive<S>(mut self, derive: S) -> Self
    where
        S: Into<String>,
    {
        self.add_derive(derive);
        self
    }

    /// Writes the derive tag.
    fn write_derives(&self, b: &mut CodeBuffer, level: usize) {
        if let Some((first, rest)) = self.derives().split_first() {
            b.indent(level);
            b.write("#[derive(");
            b.write(first);
            for derive in rest {
                b.write(", ");
                b.write(derive);
            }
            b.write(")]");
            b.end_line();
        }
    }
}
