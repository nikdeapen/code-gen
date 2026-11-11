use crate::CodeBuffer;

/// An element with attributes.
pub trait WithAttributes: Sized {
    /// Gets the attributes.
    fn attributes(&self) -> &[String];

    /// Adds the attribute. (ex: `Serializable` or `Obsolete("Use the view.")`)
    fn add_attribute<S>(&mut self, attribute: S)
    where
        S: Into<String>;

    /// Adds the attribute. (ex: `Serializable` or `Obsolete("Use the view.")`)
    #[must_use]
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
            b.push('[');
            b.write(attribute);
            b.push(']');
            b.end_line();
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::CodeBuffer;
    use crate::csharp::{Enum, WithAttributes};

    #[test]
    fn no_attributes() {
        let mut b: CodeBuffer = CodeBuffer::default();
        Enum::from("Color").write_attributes(&mut b, 1);
        assert_eq!(b.peek(), "");
    }

    #[test]
    fn attributes() {
        let e: Enum = Enum::from("Color")
            .with_attribute("Flags")
            .with_attribute("Obsolete(\"Use the shade.\")");
        let mut b: CodeBuffer = CodeBuffer::default();
        e.write_attributes(&mut b, 1);
        assert_eq!(
            b.peek(),
            "    [Flags]\n    [Obsolete(\"Use the shade.\")]\n"
        );
    }
}
