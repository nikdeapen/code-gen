use crate::c::{CommentType, Modifier, Signature, WithComments, WithModifiers, WithSignature};
use crate::{CodeBuffer, Statement};
use std::collections::BTreeSet;

/// A function prototype.
#[derive(Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub struct SignatureDec {
    comments: Vec<String>,
    modifiers: BTreeSet<Modifier>,
    signature: Signature,
}

impl<S: Into<Signature>> From<S> for SignatureDec {
    fn from(signature: S) -> Self {
        Self {
            comments: Vec::default(),
            modifiers: BTreeSet::default(),
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

impl WithModifiers for SignatureDec {
    fn modifiers(&self) -> &BTreeSet<Modifier> {
        &self.modifiers
    }

    fn add_modifier(&mut self, modifier: Modifier) {
        self.modifiers.insert(modifier);
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
        self.write_modifiers(b);
        self.write_signature(b);
        b.push(';');
        b.end_line();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::c::WithVarParams;

    #[test]
    fn simple_signature_dec() {
        let dec: SignatureDec = SignatureDec::from("foo");
        assert_eq!(dec.to_code(), "void foo(void);\n");
    }

    #[test]
    fn signature_dec_with_signature() {
        let dec: SignatureDec = SignatureDec::from(
            Signature::from("foo")
                .with_result("bool")
                .with_param(("x", "int32_t")),
        );
        assert_eq!(dec.to_code(), "bool foo(int32_t x);\n");
    }

    #[test]
    fn signature_dec_with_modifier() {
        let dec: SignatureDec = SignatureDec::from("foo").with_modifier(Modifier::Static);
        assert_eq!(dec.to_code(), "static void foo(void);\n");
    }

    #[test]
    fn signature_dec_with_comment() {
        let dec: SignatureDec = SignatureDec::from("foo").with_comment(" Does foo.");
        assert_eq!(dec.to_code(), "/** Does foo. */\nvoid foo(void);\n");
    }

    #[test]
    fn signature_dec_indented() {
        let dec: SignatureDec = SignatureDec::from("foo").with_comment(" Does foo.");
        let mut b: CodeBuffer = CodeBuffer::default();
        dec.write(&mut b, 1);
        assert_eq!(b.peek(), "    /** Does foo. */\n    void foo(void);\n");
    }
}
