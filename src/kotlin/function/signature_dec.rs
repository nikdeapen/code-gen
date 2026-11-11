use crate::kotlin::{CommentType, Modifier, Signature, WithComments, WithModifiers, WithSignature};
use crate::{CodeBuffer, Statement};
use std::collections::BTreeSet;

/// A function signature declaration. (ex: `suspend fun greet(name: String): String`)
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
        b.write("fun ");
        self.write_signature(b);
        b.end_line();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::kotlin::{WithResult, WithVarParams};

    #[test]
    fn simple_signature_dec() {
        let s = SignatureDec::from("reset");
        assert_eq!(s.to_code(), "fun reset()\n");
    }

    #[test]
    fn signature_dec_with_comment_and_modifier() {
        let s = SignatureDec::from(
            Signature::from("greet")
                .with_param(("name", "String"))
                .with_result("String"),
        )
        .with_comment(" Greets the name.")
        .with_modifier(Modifier::Suspend);
        let mut b = CodeBuffer::default();
        s.write(&mut b, 1);
        assert_eq!(
            b.peek(),
            "    /**\n     * Greets the name.\n     */\n    suspend fun greet(name: String): String\n"
        );
    }
}
