use crate::CodeBuffer;
use crate::typescript::Field;
use crate::typescript::types::members::write_members;

/// An element with fields.
pub trait WithFields: Sized {
    /// Gets the fields.
    fn fields(&self) -> &[Field];

    /// Adds the `field`.
    fn add_field<F>(&mut self, field: F)
    where
        F: Into<Field>;

    /// Adds the `field`.
    #[must_use]
    fn with_field<F>(mut self, field: F) -> Self
    where
        F: Into<Field>,
    {
        self.add_field(field);
        self
    }

    /// Writes the fields.
    fn write_fields(&self, b: &mut CodeBuffer, level: usize) {
        write_members(self.fields(), b, level);
    }
}
