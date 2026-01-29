use crate::rust::CommentType::OuterLineDoc;
use crate::rust::{Signature, WithComments, WithSignature};
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
        self.write_comments(OuterLineDoc, b, level);
        b.indent(level);
        b.write("fn ");
        self.write_signature(b);
        b.write(";");
        b.end_line();
    }
}
