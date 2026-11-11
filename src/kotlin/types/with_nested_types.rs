use crate::kotlin::NestedType;

/// An element with nested types.
pub trait WithNestedTypes: Sized {
    /// Gets the nested types.
    fn nested_types(&self) -> &[NestedType];

    /// Adds the nested type.
    fn add_nested_type<T>(&mut self, nested_type: T)
    where
        T: Into<NestedType>;

    /// Adds the nested type.
    #[must_use]
    fn with_nested_type<T>(mut self, nested_type: T) -> Self
    where
        T: Into<NestedType>,
    {
        self.add_nested_type(nested_type);
        self
    }
}
