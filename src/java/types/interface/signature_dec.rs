use crate::java::{CommentType, Signature, WithComments, WithSignature};
use crate::{CodeBuffer, Statement};

/// A function signature declaration.
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
    use crate::java::{JavaPrimitive, WithResult};

    #[test]
    fn simple_signature_dec() {
        let d: SignatureDec = SignatureDec::from("reset");
        assert_eq!(d.to_code(), "void reset();\n");
    }

    #[test]
    fn signature_dec_with_comment() {
        let d: SignatureDec =
            SignatureDec::from(Signature::from("x").with_result(JavaPrimitive::Int))
                .with_comment(" Gets the x.");
        let mut b: CodeBuffer = CodeBuffer::default();
        d.write(&mut b, 1);
        assert_eq!(
            b.peek(),
            "    /**\n     * Gets the x.\n     */\n    int x();\n"
        );
    }
}
