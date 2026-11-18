use crate::CodeBuffer;

/// An element with derive macros.
///
/// # Note
/// - The derives are written below the attributes, so an attribute like `must_use` stays above
///   them.
/// - A helper attribute of a derive macro must be below its derive, or the code doesn't compile.
///   (ex: `serde`) Add that derive as an attribute, right above the helper attribute.
pub trait WithDerives: Sized {
    /// Gets the derives.
    fn derives(&self) -> &[String];

    /// Adds the derivation.
    fn add_derive<S>(&mut self, derive: S)
    where
        S: Into<String>;

    /// Adds the derivation.
    #[must_use]
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
