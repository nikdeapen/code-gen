use crate::kotlin::KotlinType;
use crate::{CodeBuffer, Expression};

/// An element with a Kotlin type.
pub trait WithKotlinType {
    /// Gets the Kotlin type.
    fn kotlin_type(&self) -> &KotlinType;

    /// Writes the Kotlin type.
    fn write_kotlin_type(&self, b: &mut CodeBuffer) {
        self.kotlin_type().write(b);
    }
}
