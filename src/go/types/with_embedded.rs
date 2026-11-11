use crate::go::GoType;
use crate::{CodeBuffer, Expression};

/// An element with embedded types.
pub trait WithEmbedded: Sized {
    /// Gets the embedded types.
    fn embedded(&self) -> &[GoType];

    /// Adds the embedded type.
    fn add_embedded<T>(&mut self, embedded: T)
    where
        T: Into<GoType>;

    /// Adds the embedded type.
    #[must_use]
    fn with_embedded<T>(mut self, embedded: T) -> Self
    where
        T: Into<GoType>,
    {
        self.add_embedded(embedded);
        self
    }

    /// Writes a line for each embedded type.
    fn write_embedded(&self, b: &mut CodeBuffer, level: usize) {
        for embedded in self.embedded() {
            b.indent(level);
            embedded.write(b);
            b.end_line();
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::Statement;
    use crate::go::{GoType, Struct, WithEmbedded};

    #[test]
    fn no_embedded_types() {
        let s = Struct::from("PointBuf");
        assert_eq!(s.to_code(), "type PointBuf struct{}\n");
    }

    #[test]
    fn embedded_types() {
        let s = Struct::from("PointBuf")
            .with_embedded("PointRef")
            .with_embedded(GoType::from("sync.Mutex").to_pointer());
        assert_eq!(
            s.to_code(),
            "type PointBuf struct {\n    PointRef\n    *sync.Mutex\n}\n"
        );
    }
}
