use crate::typescript::{CommentType, Signature, WithComments, WithSignature};
use crate::{CodeBuffer, Statement};

/// A function signature declaration. (ex: `area(): number;`)
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
        self.write_comments(CommentType::Doc, b, level);
        b.indent(level);
        self.write_signature(b);
        b.push(';');
        b.end_line();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn signature_dec() {
        let s = SignatureDec::from(Signature::from("area").with_result("number"));
        assert_eq!(s.to_code(), "area(): number;\n");
    }

    #[test]
    fn signature_dec_with_comment() {
        let s = SignatureDec::from("close").with_comment(" Closes the shape.");
        let mut b = CodeBuffer::default();
        s.write(&mut b, 1);
        assert_eq!(b.peek(), "    /** Closes the shape. */\n    close();\n");
    }
}
