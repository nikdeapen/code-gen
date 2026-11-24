use crate::rust::{StructField, WithComments};
use crate::{CodeBuffer, EmptyLine, Statement};

/// An element with struct fields.
pub trait WithStructFields: Sized {
    /// Gets the struct fields.
    fn fields(&self) -> &[StructField];

    /// Adds the struct `field`.
    fn add_field<F>(&mut self, field: F)
    where
        F: Into<StructField>;

    /// Adds the struct `field`.
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
    /// - A field with comments has a blank line above it, unless it is the first field.
    fn write_fields(&self, b: &mut CodeBuffer, level: usize) {
        for (index, field) in self.fields().iter().enumerate() {
            if index > 0 && !field.comments().is_empty() {
                EmptyLine::default().write(b, level);
            }
            field.write(b, level);
        }
    }
}
