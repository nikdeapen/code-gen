use crate::rust::Access;
use crate::{CodeBuffer, Expression};

/// An element with an access level.
pub trait WithAccess: Sized {
    /// Gets the access level.
    fn access(&self) -> &Access;

    /// Sets the `access` level.
    fn set_access<A>(&mut self, access: A)
    where
        A: Into<Access>;

    /// Sets the `access` level.
    fn with_access<A>(mut self, access: A) -> Self
    where
        A: Into<Access>,
    {
        self.set_access(access);
        self
    }

    /// Writes the access level.
    fn write_access(&self, b: &mut CodeBuffer) {
        self.access().write(b);
    }
}
