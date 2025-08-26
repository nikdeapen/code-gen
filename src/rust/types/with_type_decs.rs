use crate::rust::TypeDec;
use crate::{CodeBuffer, EmptyLine, Statement};

/// An element with type declarations.
pub trait WithTypeDecs: Sized {
    /// Gets the type decs.
    fn type_decs(&self) -> &[TypeDec];

    /// Adds the `type_dec`.
    fn add_type_dec<D>(&mut self, type_dec: D)
    where
        D: Into<TypeDec>;

    /// Adds the `type_dec`.
    fn with_type_dec<D>(mut self, type_dec: D) -> Self
    where
        D: Into<TypeDec>,
    {
        self.add_type_dec(type_dec);
        self
    }

    /// Writes the type declarations.
    fn write_type_decs(&self, b: &mut CodeBuffer, level: usize) {
        for type_dec in self.type_decs() {
            type_dec.write(b, level);
        }
        if !self.type_decs().is_empty() {
            EmptyLine::default().write(b, level);
        }
    }
}
