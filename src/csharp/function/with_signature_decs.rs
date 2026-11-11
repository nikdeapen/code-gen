use crate::csharp::SignatureDec;

/// An element with function signature declarations.
pub trait WithSignatureDecs: Sized {
    /// Gets the function signature declarations.
    fn signature_decs(&self) -> &[SignatureDec];

    /// Adds the function signature declaration.
    fn add_signature_dec<S>(&mut self, signature_dec: S)
    where
        S: Into<SignatureDec>;

    /// Adds the function signature declaration.
    #[must_use]
    fn with_signature_dec<S>(mut self, signature_dec: S) -> Self
    where
        S: Into<SignatureDec>,
    {
        self.add_signature_dec(signature_dec);
        self
    }
}
