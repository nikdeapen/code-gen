use crate::kotlin::KotlinType;
use crate::{CodeBuffer, Expression};

/// An element with an optional result.
pub trait WithResult: Sized {
    /// Gets the optional result.
    fn result(&self) -> Option<&KotlinType>;

    /// Sets the result.
    fn set_result<T>(&mut self, result: T)
    where
        T: Into<KotlinType>;

    /// Sets the result.
    #[must_use]
    fn with_result<T>(mut self, result: T) -> Self
    where
        T: Into<KotlinType>,
    {
        self.set_result(result);
        self
    }

    /// Writes the optional result, after its colon. (ex: `: Int`)
    fn write_result(&self, b: &mut CodeBuffer) {
        if let Some(result) = self.result() {
            b.write(": ");
            result.write(b);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::kotlin::Signature;

    /// Writes the result of the `signature`.
    fn code(signature: &Signature) -> String {
        let mut b = CodeBuffer::default();
        signature.write_result(&mut b);
        b.into()
    }

    #[test]
    fn no_result() {
        assert_eq!(code(&Signature::from("f")), "");
    }

    #[test]
    fn result() {
        assert_eq!(code(&Signature::from("f").with_result("Int")), ": Int");
    }
}
