use crate::cpp::Signature;
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cpp::SignatureDec;

    #[test]
    fn write_signature() {
        let dec = SignatureDec::from(Signature::from("size").with_result("std::size_t"));
        let mut b = CodeBuffer::default();
        dec.write_signature(&mut b);
        assert_eq!(b.peek(), "std::size_t size()");
    }
}
