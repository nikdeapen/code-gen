use crate::csharp::Field;

/// An element with fields.
pub trait WithFields: Sized {
    /// Gets the fields.
    fn fields(&self) -> &[Field];

    /// Adds the field.
    fn add_field<F>(&mut self, field: F)
    where
        F: Into<Field>;

    /// Adds the field.
    #[must_use]
    fn with_field<F>(mut self, field: F) -> Self
    where
        F: Into<Field>,
    {
        self.add_field(field);
        self
    }
}
