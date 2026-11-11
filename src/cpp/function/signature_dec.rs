use crate::cpp::{
    Access, CommentType, Generic, Signature, WithAccess, WithAttributes, WithComments,
    WithGenerics, WithSignature,
};
use crate::{CodeBuffer, Statement};

/// A function signature declaration. (ex: `int x() const;`)
///
/// # Note
/// - A pure virtual function is a declaration. So is a defaulted function.
#[derive(Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub struct SignatureDec {
    comments: Vec<String>,
    generics: Vec<Generic>,
    attributes: Vec<String>,
    access: Access,
    signature: Signature,
}

impl<S: Into<Signature>> From<S> for SignatureDec {
    fn from(signature: S) -> Self {
        Self {
            comments: Vec::default(),
            generics: Vec::default(),
            attributes: Vec::default(),
            access: Access::default(),
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

impl WithGenerics for SignatureDec {
    fn generics(&self) -> &[Generic] {
        self.generics.as_slice()
    }

    fn add_generic<G>(&mut self, generic: G)
    where
        G: Into<Generic>,
    {
        self.generics.push(generic.into());
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

impl WithAccess for SignatureDec {
    fn access(&self) -> Access {
        self.access
    }

    fn set_access(&mut self, access: Access) {
        self.access = access;
    }
}

impl WithSignature for SignatureDec {
    fn signature(&self) -> &Signature {
        &self.signature
    }
}

impl Statement for SignatureDec {
    fn write(&self, b: &mut CodeBuffer, level: usize) {
        self.write_comments(CommentType::LineDoc, b, level);
        self.write_generics(b, level);
        b.indent(level);
        self.write_attributes(b);
        self.write_signature(b);
        b.push(';');
        b.end_line();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cpp::{CppType, Modifier, WithModifiers, WithVarParams};

    #[test]
    fn simple_signature_dec() {
        let d = SignatureDec::from("reset");
        assert_eq!(d.to_code(), "void reset();\n");
    }

    #[test]
    fn signature_dec_with_comment_and_attribute() {
        let d = SignatureDec::from(
            Signature::from("x")
                .with_result("std::uint32_t")
                .with_modifier(Modifier::Const),
        )
        .with_comment(" Gets the field: `x`.")
        .with_attribute("nodiscard")
        .with_access(Access::Public);
        assert_eq!(d.access(), Access::Public);
        let mut b = CodeBuffer::default();
        d.write(&mut b, 1);
        assert_eq!(
            b.peek(),
            "    /// Gets the field: `x`.\n    [[nodiscard]] std::uint32_t x() const;\n"
        );
    }

    #[test]
    fn signature_dec_with_generics() {
        let d = SignatureDec::from(
            Signature::from("write").with_param(("value", CppType::from("T").to_const().to_ref())),
        )
        .with_generic("T");
        assert_eq!(
            d.to_code(),
            "template <typename T>\nvoid write(const T& value);\n"
        );
    }

    #[test]
    fn defaulted_signature_dec() {
        let d = SignatureDec::from(
            Signature::from("operator==")
                .with_param(("other", CppType::from("Point").to_const().to_ref()))
                .with_result("bool")
                .with_modifier(Modifier::Const)
                .with_modifier(Modifier::Default),
        );
        assert_eq!(
            d.to_code(),
            "bool operator==(const Point& other) const = default;\n"
        );
    }
}
