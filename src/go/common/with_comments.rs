use crate::CodeBuffer;

/// An element with comment lines.
pub trait WithComments: Sized {
    /// Gets the comment lines.
    fn comments(&self) -> &[String];

    /// Adds the comment line.
    fn add_comment<S>(&mut self, comment: S)
    where
        S: Into<String>;

    /// Adds the comment line.
    #[must_use]
    fn with_comment<S>(mut self, comment: S) -> Self
    where
        S: Into<String>,
    {
        self.add_comment(comment);
        self
    }

    /// Writes the comment lines.
    ///
    /// # Note
    /// - A line is written right after its `//`. Start it with a space.
    fn write_comments(&self, b: &mut CodeBuffer, level: usize) {
        for line in self.comments() {
            b.indent(level);
            b.write("//");
            b.write(line.as_str());
            b.end_line();
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::go::{TypeDef, WithComments};
    use crate::{CodeBuffer, Statement};

    #[test]
    fn no_comments() {
        let t = TypeDef::from(("Color", "uint16"));
        assert_eq!(t.to_code(), "type Color uint16\n");
    }

    #[test]
    fn comment_lines() {
        let t = TypeDef::from(("Color", "uint16"))
            .with_comment(" Color is a color.")
            .with_comment("")
            .with_comment(" Its cases are tag numbers.");
        assert_eq!(
            t.to_code(),
            "// Color is a color.\n//\n// Its cases are tag numbers.\ntype Color uint16\n"
        );
    }

    #[test]
    fn comment_lines_indented() {
        let t = TypeDef::from(("Color", "uint16")).with_comment(" Color is a color.");
        let mut b = CodeBuffer::default();
        t.write(&mut b, 1);
        assert_eq!(
            b.peek(),
            "    // Color is a color.\n    type Color uint16\n"
        );
    }
}
