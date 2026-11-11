use crate::go::{Signature, WithComments, WithSignature};
use crate::{CodeBuffer, Statement};

/// A function signature declaration. (ex: `X() uint32`)
#[derive(Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub struct SignatureDec {
    comments: Vec<String>,
    signature: Signature,
}

impl<S: Into<Signature>> From<S> for SignatureDec {
    fn from(signature: S) -> Self {
        Self {
            comments: Vec::default(),
            signature: signature.into(),
        }
    }
}

impl WithComments for SignatureDec {
    fn comments(&self) -> &[String] {
        self.comments.as_slice()
    }

    fn add_comment<S>(&mut self, comment: S)
    where
        S: Into<String>,
    {
        self.comments.push(comment.into());
    }
}

impl WithSignature for SignatureDec {
    fn signature(&self) -> &Signature {
        &self.signature
    }
}

impl Statement for SignatureDec {
    fn write(&self, b: &mut CodeBuffer, level: usize) {
        self.write_comments(b, level);
        b.indent(level);
        self.write_signature(b);
        b.end_line();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::go::WithVarParams;

    #[test]
    fn signature_dec() {
        let s = SignatureDec::from(Signature::from("SetX").with_param(("x", "uint32")));
        assert_eq!(s.to_code(), "SetX(x uint32)\n");
    }

    #[test]
    fn signature_dec_with_comment() {
        let s = SignatureDec::from(Signature::from("X").with_result("uint32"))
            .with_comment(" X gets the field: x.");
        assert_eq!(s.to_code(), "// X gets the field: x.\nX() uint32\n");
    }
}
