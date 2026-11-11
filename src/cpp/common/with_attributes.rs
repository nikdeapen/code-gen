use crate::CodeBuffer;

/// An element with attributes.
pub trait WithAttributes: Sized {
    /// Gets the attributes.
    fn attributes(&self) -> &[String];

    /// Adds the attribute, without its brackets. (ex: `nodiscard`)
    fn add_attribute<S>(&mut self, attribute: S)
    where
        S: Into<String>;

    /// Adds the attribute, without its brackets. (ex: `nodiscard`)
    #[must_use]
    fn with_attribute<S>(mut self, attribute: S) -> Self
    where
        S: Into<String>,
    {
        self.add_attribute(attribute);
        self
    }

    /// Writes the attributes, each followed by a space. (ex: `[[nodiscard]] `)
    fn write_attributes(&self, b: &mut CodeBuffer) {
        for attribute in self.attributes() {
            b.write("[[");
            b.write(attribute);
            b.write("]] ");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cpp::SignatureDec;

    #[test]
    fn write_attributes() {
        let dec = SignatureDec::from("foo")
            .with_attribute("nodiscard")
            .with_attribute("deprecated(\"Use bar.\")");
        let mut b = CodeBuffer::default();
        dec.write_attributes(&mut b);
        assert_eq!(b.peek(), "[[nodiscard]] [[deprecated(\"Use bar.\")]] ");
    }

    #[test]
    fn write_no_attributes() {
        let dec = SignatureDec::from("foo");
        let mut b = CodeBuffer::default();
        dec.write_attributes(&mut b);
        assert_eq!(b.peek(), "");
    }
}
