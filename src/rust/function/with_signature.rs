use crate::rust::Signature;
use crate::{CodeBuffer, Expression};

/// An element with a function signature.
pub trait WithSignature {
    /// Gets the function signature.
    fn signature(&self) -> &Signature;

    /// Writes the function signature.
    fn write_signature(&self, b: &mut CodeBuffer) {
        self.signature().write(b);
    }
}
