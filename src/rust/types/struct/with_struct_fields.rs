use crate::rust::StructField;
use crate::{CodeBuffer, Statement};

/// An element with struct fields.
pub trait WithStructFields: Sized {
    /// Gets the struct fields.
    fn fields(&self) -> &[StructField];

    /// Adds the struct `field`.
    fn add_field<F>(&mut self, field: F)
    where
        F: Into<StructField>;

    /// Adds the struct `field`.
    fn with_field<F>(mut self, field: F) -> Self
    where
        F: Into<StructField>,
    {
        self.add_field(field);
        self
    }

    /// Writes the struct fields.
    fn write_fields(&self, b: &mut CodeBuffer, level: usize) {
        for field in self.fields() {
            field.write(b, level);
        }
    }
}
