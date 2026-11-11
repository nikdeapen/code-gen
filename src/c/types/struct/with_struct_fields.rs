use crate::c::{StructField, WithComments};
use crate::{CodeBuffer, Statement};

/// An element with struct fields.
pub trait WithStructFields: Sized {
    /// Gets the struct fields.
    fn fields(&self) -> &[StructField];

    /// Adds a struct field.
    fn add_field<F>(&mut self, field: F)
    where
        F: Into<StructField>;

    /// Adds a struct field.
    #[must_use]
    fn with_field<F>(mut self, field: F) -> Self
    where
        F: Into<StructField>,
    {
        self.add_field(field);
        self
    }

    /// Writes the struct fields.
    ///
    /// # Note
    /// - A blank line is written between two fields if either has comments.
    fn write_fields(&self, b: &mut CodeBuffer, level: usize) {
        if let Some((first, rest)) = self.fields().split_first() {
            first.write(b, level);
            for (previous, field) in self.fields().iter().zip(rest) {
                if !previous.comments().is_empty() || !field.comments().is_empty() {
                    b.end_line();
                }
                field.write(b, level);
            }
        }
    }

    /// Writes the curly-bracketed struct fields. (`level` is the outer level)
    fn write_curly_field_block(&self, b: &mut CodeBuffer, level: usize) {
        b.push('{');
        if !self.fields().is_empty() {
            b.end_line();
            self.write_fields(b, level + 1);
            b.indent(level);
        }
        b.push('}');
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::c::Struct;

    /// Writes the struct's curly-bracketed fields to a string.
    fn write(structure: &Struct, level: usize) -> String {
        let mut b: CodeBuffer = CodeBuffer::default();
        structure.write_curly_field_block(&mut b, level);
        b.into()
    }

    #[test]
    fn no_fields() {
        assert_eq!(write(&Struct::from("empty"), 0), "{}");
    }

    #[test]
    fn fields_without_comments() {
        let s: Struct = Struct::from("point")
            .with_field(("x", "int32_t"))
            .with_field(("y", "int32_t"));
        assert_eq!(write(&s, 0), "{\n    int32_t x;\n    int32_t y;\n}");
    }

    #[test]
    fn fields_with_comments() {
        let s: Struct = Struct::from("point")
            .with_field(StructField::from(("x", "int32_t")).with_comment(" The x."))
            .with_field(StructField::from(("y", "int32_t")).with_comment(" The y."));
        assert_eq!(
            write(&s, 0),
            "{\n    /** The x. */\n    int32_t x;\n\n    /** The y. */\n    int32_t y;\n}"
        );
    }

    #[test]
    fn fields_with_some_comments() {
        let s: Struct = Struct::from("point")
            .with_field(("a", "int32_t"))
            .with_field(("b", "int32_t"))
            .with_field(StructField::from(("c", "int32_t")).with_comment(" The c."))
            .with_field(("d", "int32_t"));
        assert_eq!(
            write(&s, 0),
            "{\n    int32_t a;\n    int32_t b;\n\n    /** The c. */\n    int32_t c;\n\n    int32_t d;\n}"
        );
    }

    #[test]
    fn fields_indented() {
        let s: Struct = Struct::from("point").with_field(("x", "int32_t"));
        assert_eq!(write(&s, 1), "{\n        int32_t x;\n    }");
    }
}
