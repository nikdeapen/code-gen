use crate::csharp::{
    Access, CommentType, Modifier, Signature, WithAccess, WithComments, WithModifiers,
    WithSignature,
};
use crate::{CodeBuffer, Statement};
use std::collections::BTreeSet;

/// A function signature declaration, which is a function with no body.
///
/// # Note
/// - In an interface it needs no access level & no modifier.
/// - In a class it needs the `abstract` modifier.
#[derive(Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub struct SignatureDec {
    comments: Vec<String>,
    access: Access,
    modifiers: BTreeSet<Modifier>,
    signature: Signature,
}

impl<S: Into<Signature>> From<S> for SignatureDec {
    fn from(signature: S) -> Self {
        Self {
            comments: Vec::default(),
            access: Access::default(),
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

impl WithAccess for SignatureDec {
    fn access(&self) -> Access {
        self.access
    }

    fn set_access(&mut self, access: Access) {
        self.access = access;
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
        self.write_access(b);
        self.write_modifiers(b);
        self.write_signature(b);
        b.push(';');
        b.end_line();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn interface_signature_dec() {
        let s: SignatureDec = SignatureDec::from(Signature::from("Count").with_result("int"));
        assert_eq!(s.to_code(), "int Count();\n");
    }

    #[test]
    fn abstract_signature_dec() {
        let s: SignatureDec = SignatureDec::from("Run")
            .with_access(Access::Public)
            .with_modifier(Modifier::Abstract);
        assert_eq!(s.to_code(), "public abstract void Run();\n");
    }

    #[test]
    fn signature_dec_with_comment() {
        let s: SignatureDec = SignatureDec::from("Run").with_comment(" Runs the task.");
        let mut b: CodeBuffer = CodeBuffer::default();
        s.write(&mut b, 1);
        assert_eq!(
            b.peek(),
            "    /// <summary>\n    /// Runs the task.\n    /// </summary>\n    void Run();\n"
        );
    }
}
