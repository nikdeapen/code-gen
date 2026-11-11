use crate::csharp::CSharpType;
use crate::{CodeBuffer, Expression};

/// An element with base types, which are its base class & its interfaces.
///
/// # Note
/// - The base types are written as they are added. C# wants the base class first.
pub trait WithBaseTypes: Sized {
    /// Gets the base types.
    fn base_types(&self) -> &[CSharpType];

    /// Adds the base type.
    fn add_base_type<T>(&mut self, base_type: T)
    where
        T: Into<CSharpType>;

    /// Adds the base type.
    #[must_use]
    fn with_base_type<T>(mut self, base_type: T) -> Self
    where
        T: Into<CSharpType>,
    {
        self.add_base_type(base_type);
        self
    }

    /// Writes the base types, with a space before them. (ex: ` : Shape, IDisposable`)
    fn write_base_types(&self, b: &mut CodeBuffer) {
        if let Some((first, rest)) = self.base_types().split_first() {
            b.write(" : ");
            first.write(b);
            for base_type in rest {
                b.write(", ");
                base_type.write(b);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::CodeBuffer;
    use crate::csharp::{Interface, WithBaseTypes};

    #[test]
    fn no_base_types() {
        let mut b: CodeBuffer = CodeBuffer::default();
        Interface::from("IShape").write_base_types(&mut b);
        assert_eq!(b.peek(), "");
    }

    #[test]
    fn base_types() {
        let i: Interface = Interface::from("IShape")
            .with_base_type("IView")
            .with_base_type("IDisposable");
        let mut b: CodeBuffer = CodeBuffer::default();
        i.write_base_types(&mut b);
        assert_eq!(b.peek(), " : IView, IDisposable");
    }
}
