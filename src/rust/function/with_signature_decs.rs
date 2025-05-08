use crate::rust::SignatureDec;
use crate::{CodeBuffer, Statement};

/// An element with function signature declarations.
pub trait WithTraitFunctions: Sized {
    /// Gets the function signature declarations.
    fn signature_decs(&self) -> &[SignatureDec];

    /// Adds the function `signature` declaration.
    fn add_signature_dec<S>(&mut self, signature: S)
    where
        S: Into<SignatureDec>;

    /// Adds the function `signature` declaration.
    fn with_signature_dec<S>(mut self, signature: S) -> Self
    where
        S: Into<SignatureDec>,
    {
        self.add_signature_dec(signature);
        self
    }

    /// Writes the function signature declarations.
    fn write_signature_decs(&self, b: &mut CodeBuffer, level: usize) {
        if let Some((first, rest)) = self.signature_decs().split_first() {
            first.write(b, level);
            for function in rest {
                b.end_line();
                function.write(b, level);
            }
        }
    }
}
