use crate::csharp::CSharpType;
use crate::{CodeBuffer, Expression};

/// An element with a C# type.
pub trait WithCSharpType {
    /// Gets the C# type.
    fn csharp_type(&self) -> &CSharpType;

    /// Writes the C# type.
    fn write_csharp_type(&self, b: &mut CodeBuffer) {
        self.csharp_type().write(b);
    }
}
