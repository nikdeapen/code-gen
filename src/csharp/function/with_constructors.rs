use crate::csharp::Constructor;

/// An element with constructors.
pub trait WithConstructors: Sized {
    /// Gets the constructors.
    fn constructors(&self) -> &[Constructor];

    /// Adds the constructor.
    fn add_constructor<C>(&mut self, constructor: C)
    where
        C: Into<Constructor>;

    /// Adds the constructor.
    #[must_use]
    fn with_constructor<C>(mut self, constructor: C) -> Self
    where
        C: Into<Constructor>,
    {
        self.add_constructor(constructor);
        self
    }
}
