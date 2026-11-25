use crate::rust::CommentType::OuterLineDoc;
use crate::rust::{Signature, WithAttributes, WithComments, WithSignature};
use crate::{CodeBuffer, Statement};

/// A function signature declaration.
pub struct SignatureDec {
    comments: Vec<String>,
    attributes: Vec<String>,
    signature: Signature,
}

impl<S: Into<Signature>> From<S> for SignatureDec {
    fn from(signature: S) -> Self {
        Self {
            comments: Vec::default(),
            attributes: Vec::default(),
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

impl WithAttributes for SignatureDec {
    fn attributes(&self) -> &[String] {
        self.attributes.as_slice()
    }

    fn add_attribute<S>(&mut self, attribute: S)
    where
        S: Into<String>,
    {
        self.attributes.push(attribute.into());
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
        self.write_attributes(b, level);
        b.indent(level);
        b.write("fn ");
        self.write_signature(b);
        b.push(';');
        b.end_line();
    }
}
