use crate::kotlin::Object;

/// An element with an optional companion object.
pub trait WithCompanion: Sized {
    /// Gets the optional companion object.
    fn companion(&self) -> Option<&Object>;

    /// Sets the companion object.
    fn set_companion(&mut self, companion: Object);

    /// Sets the companion object.
    #[must_use]
    fn with_companion(mut self, companion: Object) -> Self {
        self.set_companion(companion);
        self
    }
}
