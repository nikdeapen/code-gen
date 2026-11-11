use crate::python::{Signature, WithComments, WithDecorators, WithSignature};
use crate::{CodeBuffer, Statement};

/// A function signature declaration.
///
/// # Note
/// - Its body is `...`, which is how a protocol or an abstract class declares a function.
#[derive(Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub struct SignatureDec {
    decorators: Vec<String>,
    signature: Signature,
    comments: Vec<String>,
}

impl<S: Into<Signature>> From<S> for SignatureDec {
    fn from(signature: S) -> Self {
        Self {
            decorators: Vec::default(),
            signature: signature.into(),
            comments: Vec::default(),
        }
    }
}

impl WithDecorators for SignatureDec {
    fn decorators(&self) -> &[String] {
        self.decorators.as_slice()
    }

    fn add_decorator<S>(&mut self, decorator: S)
    where
        S: Into<String>,
    {
        self.decorators.push(decorator.into());
    }
}

impl WithSignature for SignatureDec {
    fn signature(&self) -> &Signature {
        &self.signature
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

impl Statement for SignatureDec {
    fn write(&self, b: &mut CodeBuffer, level: usize) {
        self.write_decorators(b, level);
        b.indent(level);
        self.write_signature(b);
        if self.comments.is_empty() {
            b.write(": ...");
            b.end_line();
        } else {
            b.push(':');
            b.end_line();
            self.write_comments(b, level + 1);
            b.line(level + 1, "...");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::python::Receiver;

    #[test]
    fn simple_signature_dec() {
        let s = SignatureDec::from(
            Signature::from("area")
                .with_receiver(Receiver::Instance)
                .with_result("float"),
        );
        assert_eq!(s.to_code(), "def area(self) -> float: ...\n");
    }

    #[test]
    fn signature_dec_with_decorator() {
        let s = SignatureDec::from("area").with_decorator("abstractmethod");
        assert_eq!(s.to_code(), "@abstractmethod\ndef area(): ...\n");
    }

    #[test]
    fn signature_dec_with_comment() {
        let s = SignatureDec::from("area").with_comment("Gets the area.");
        assert_eq!(
            s.to_code(),
            "def area():\n    \"\"\"Gets the area.\"\"\"\n    ...\n"
        );
    }

    #[test]
    fn signature_dec_indented() {
        let s = SignatureDec::from("area")
            .with_decorator("property")
            .with_comment("Gets the area.");
        let mut b = CodeBuffer::default();
        s.write(&mut b, 1);
        assert_eq!(
            b.peek(),
            "    @property\n    def area():\n        \"\"\"Gets the area.\"\"\"\n        ...\n"
        );
    }
}
