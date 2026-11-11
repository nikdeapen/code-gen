use crate::cpp::{
    Access, CommentType, Generic, Signature, WithAccess, WithAttributes, WithComments,
    WithGenerics, WithSignature,
};
use crate::{CodeBuffer, Statement, WithStatements};

/// A function definition.
///
/// # Note
/// - A function defined outside its class takes its signature from [Signature::to_scoped].
pub struct Function {
    comments: Vec<String>,
    generics: Vec<Generic>,
    attributes: Vec<String>,
    access: Access,
    signature: Signature,
    statements: Vec<Box<dyn Statement>>,
}

impl<S: Into<Signature>> From<S> for Function {
    fn from(signature: S) -> Self {
        Self {
            comments: Vec::default(),
            generics: Vec::default(),
            attributes: Vec::default(),
            access: Access::default(),
            signature: signature.into(),
            statements: Vec::default(),
        }
    }
}

impl WithComments for Function {
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

impl WithGenerics for Function {
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

impl WithAttributes for Function {
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

impl WithAccess for Function {
    fn access(&self) -> Access {
        self.access
    }

    fn set_access(&mut self, access: Access) {
        self.access = access;
    }
}

impl WithSignature for Function {
    fn signature(&self) -> &Signature {
        &self.signature
    }
}

impl WithStatements for Function {
    fn statements(&self) -> &[Box<dyn Statement>] {
        self.statements.as_slice()
    }

    fn add_boxed_statement(&mut self, statement: Box<dyn Statement>) {
        self.statements.push(statement);
    }
}

impl Statement for Function {
    fn write(&self, b: &mut CodeBuffer, level: usize) {
        self.write_comments(CommentType::LineDoc, b, level);
        self.write_generics(b, level);
        b.indent(level);
        self.write_attributes(b);
        self.write_signature(b);
        b.space();
        self.write_curly_statement_block(b, level);
        b.end_line();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cpp::{CppType, Modifier, Var, WithModifiers, WithVarParams};

    #[test]
    fn empty_function() {
        let f = Function::from("reset");
        assert_eq!(f.to_code(), "void reset() {}\n");
    }

    #[test]
    fn function_with_body() {
        let f = Function::from(
            Signature::from("x")
                .with_result("std::uint32_t")
                .with_modifier(Modifier::Const),
        )
        .with_access(Access::Public)
        .with_semi("return x_");
        assert_eq!(f.access(), Access::Public);
        assert_eq!(
            f.to_code(),
            "std::uint32_t x() const {\n    return x_;\n}\n"
        );
    }

    #[test]
    fn function_with_comment_and_attribute() {
        let f = Function::from(Signature::from("size").with_result("std::size_t"))
            .with_comment(" Gets the size.")
            .with_attribute("nodiscard")
            .with_semi("return 8");
        let mut b = CodeBuffer::default();
        f.write(&mut b, 1);
        assert_eq!(
            b.peek(),
            "    /// Gets the size.\n    [[nodiscard]] std::size_t size() {\n        return 8;\n    }\n"
        );
    }

    #[test]
    fn function_with_generics() {
        let f = Function::from(
            Signature::from("write")
                .with_param(("out", "O"))
                .with_result("O"),
        )
        .with_generic(Var::from(("O", "std::output_iterator<std::uint8_t>")))
        .with_semi("return out");
        assert_eq!(
            f.to_code(),
            "template <std::output_iterator<std::uint8_t> O>\nO write(O out) {\n    return out;\n}\n"
        );
    }

    #[test]
    fn function_defined_outside_its_class() {
        let signature = Signature::from("decode")
            .with_param(("bytes", CppType::from("Bytes").to_const().to_ref()))
            .with_result("Point")
            .with_modifier(Modifier::Static);
        let f = Function::from(signature.to_scoped("Point")).with_semi("return Point(bytes)");
        assert_eq!(
            f.to_code(),
            "Point Point::decode(const Bytes& bytes) {\n    return Point(bytes);\n}\n"
        );
    }
}
