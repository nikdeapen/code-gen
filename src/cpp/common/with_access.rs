use crate::cpp::Access;

/// An element with an access level.
///
/// # Note
/// - The access level picks the section of the class that the element is written in.
/// - An element outside a class ignores it.
pub trait WithAccess: Sized {
    /// Gets the access level.
    fn access(&self) -> Access;

    /// Sets the access level.
    fn set_access(&mut self, access: Access);

    /// Sets the access level.
    #[must_use]
    fn with_access(mut self, access: Access) -> Self {
        self.set_access(access);
        self
    }
}
