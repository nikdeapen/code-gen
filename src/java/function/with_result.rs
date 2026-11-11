use crate::java::JavaType;
use crate::{CodeBuffer, Expression};

/// An element with an optional result.
pub trait WithResult: Sized {
    /// Gets the optional result.
    fn result(&self) -> Option<&JavaType>;

    /// Sets the result.
    fn set_result<T>(&mut self, result: T)
    where
        T: Into<JavaType>;

    /// Sets the result.
    #[must_use]
    fn with_result<T>(mut self, result: T) -> Self
    where
        T: Into<JavaType>,
    {
        self.set_result(result);
        self
    }

    /// Writes the result, which is `void` when there is none.
    fn write_result(&self, b: &mut CodeBuffer) {
        match self.result() {
            Some(result) => result.write(b),
            None => b.write("void"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::java::Signature;

    /// Writes the result of the `signature`.
    fn write(signature: Signature) -> String {
        let mut b: CodeBuffer = CodeBuffer::default();
        signature.write_result(&mut b);
        b.into()
    }

    #[test]
    fn no_result() {
        assert_eq!(write(Signature::from("f")), "void");
    }

    #[test]
    fn result() {
        assert_eq!(write(Signature::from("f").with_result("String")), "String");
    }
}
