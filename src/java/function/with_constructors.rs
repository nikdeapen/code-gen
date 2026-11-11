use crate::java::Constructor;

/// An element with constructors.
pub trait WithConstructors: Sized {
    /// Gets the constructors.
    fn constructors(&self) -> &[Constructor];

    /// Adds a constructor.
    fn add_constructor(&mut self, constructor: Constructor);

    /// Adds a constructor.
    #[must_use]
    fn with_constructor(mut self, constructor: Constructor) -> Self {
        self.add_constructor(constructor);
        self
    }
}
