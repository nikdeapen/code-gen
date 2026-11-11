use crate::java::Access;
use crate::{CodeBuffer, Expression};

/// An element with an access level.
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

    /// Writes the access level.
    fn write_access(&self, b: &mut CodeBuffer) {
        self.access().write(b);
    }
}
