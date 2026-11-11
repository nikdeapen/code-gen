use crate::Statement;

/// An element with nested types.
///
/// # Note
/// - A nested type is any statement, since each kind of type is its own element.
pub trait WithNestedTypes: Sized {
    /// Gets the nested types.
    fn nested_types(&self) -> &[Box<dyn Statement>];

    /// Adds the nested type.
    fn add_nested_type<S>(&mut self, nested_type: S)
    where
        S: 'static + Statement;

    /// Adds the nested type.
    #[must_use]
    fn with_nested_type<S>(mut self, nested_type: S) -> Self
    where
        S: 'static + Statement,
    {
        self.add_nested_type(nested_type);
        self
    }
}
