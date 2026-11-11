use crate::kotlin::Property;

/// An element with properties.
pub trait WithProperties: Sized {
    /// Gets the properties.
    fn properties(&self) -> &[Property];

    /// Adds the property.
    fn add_property<P>(&mut self, property: P)
    where
        P: Into<Property>;

    /// Adds the property.
    #[must_use]
    fn with_property<P>(mut self, property: P) -> Self
    where
        P: Into<Property>,
    {
        self.add_property(property);
        self
    }
}
