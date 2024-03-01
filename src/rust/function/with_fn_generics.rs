use crate::rust::Var;

/// An element with generic type parameters.
pub trait WithFnGenerics: Sized {
    /// Gets the generics.
    fn generics(&self) -> &[Var];

    /// Adds the generic type parameter.
    fn with_generic<V>(mut self, generic: V) -> Self
    where
        V: Into<Var>,
    {
        self.add_generic(generic);
        self
    }

    /// Adds the generic type parameter.
    fn add_generic<V>(&mut self, generic: V)
    where
        V: Into<Var>;
}
