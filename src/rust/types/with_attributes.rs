use crate::CodeBuffer;

/// An element with attributes.
pub trait WithAttributes: Sized {
    /// Gets the attributes.
    fn attributes(&self) -> &[String];

    /// Adds the attribute.
    fn add_attribute<S>(&mut self, attribute: S)
    where
        S: Into<String>;

    /// Adds the attribute.
    fn with_attribute<S>(mut self, attribute: S) -> Self
    where
        S: Into<String>,
    {
        self.add_attribute(attribute);
        self
    }

    /// Writes the attributes.
    fn write_attributes(&self, b: &mut CodeBuffer, level: usize) {
        for attribute in self.attributes() {
            b.indent(level);
            b.write("#[");
            b.write(attribute);
            b.write("]");
            b.end_line();
        }
    }
}
