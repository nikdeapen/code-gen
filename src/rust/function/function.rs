use crate::rust::CommentType::OuterLineDoc;
use crate::rust::{
    Access, Signature, WithAccess, WithAttributes, WithComments, WithSignature, WithUnsafeFlag,
};
use crate::{CodeBuffer, Statement, WithStatements};

/// A function declaration.
pub struct Function {
    comments: Vec<String>,
    attributes: Vec<String>,
    is_async: bool,
    is_const: bool,
    access: Access,
    signature: Signature,
    statements: Vec<Box<dyn Statement>>,
}

impl<S: Into<Signature>> From<S> for Function {
    fn from(signature: S) -> Self {
        Self {
            comments: Vec::default(),
            attributes: Vec::default(),
            is_async: false,
            is_const: false,
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
    fn access(&self) -> &Access {
        &self.access
    }

    fn set_access<A>(&mut self, access: A)
    where
        A: Into<Access>,
    {
        self.access = access.into();
    }
}

impl Function {
    //! Async

    /// Sets the `is_async` flag.
    pub fn set_async(&mut self, is_async: bool) {
        self.is_async = is_async;
    }

    /// Sets the `is_async` flag.
    #[must_use]
    pub fn with_async(mut self, is_async: bool) -> Self {
        self.set_async(is_async);
        self
    }
}

impl Function {
    //! Const

    /// Sets the `is_const` flag.
    pub fn set_const(&mut self, is_const: bool) {
        self.is_const = is_const;
    }

    /// Sets the `is_const` flag.
    #[must_use]
    pub fn with_const(mut self, is_const: bool) -> Self {
        self.set_const(is_const);
        self
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rust::{Receiver, RustType, WithReceiver, WithResult};
    use crate::WithStatements;

    #[test]
    fn empty_function() {
        let f = Function::from("foo");
        assert_eq!(f.to_code(), "fn foo() {}\n");
    }

    #[test]
    fn public_function() {
        let f = Function::from("foo").with_access(Access::Public);
        assert_eq!(f.to_code(), "pub fn foo() {}\n");
    }

    #[test]
    fn function_with_body() {
        let f = Function::from(
            Signature::from("greet")
                .with_receiver(Receiver::Borrowed)
                .with_result(RustType::from("String")),
        )
        .with_access(Access::Public)
        .with_semi("self.name.clone()");
        assert_eq!(
            f.to_code(),
            "pub fn greet(&self) -> String {\n    self.name.clone();\n}\n"
        );
    }

    #[test]
    fn async_function() {
        let f = Function::from("fetch").with_async(true);
        assert_eq!(f.to_code(), "async fn fetch() {}\n");
    }

    #[test]
    fn const_function() {
        let f = Function::from("new").with_const(true);
        assert_eq!(f.to_code(), "const fn new() {}\n");
    }

    #[test]
    fn function_with_comment() {
        let f = Function::from("foo").with_comment("Does foo.");
        assert_eq!(f.to_code(), "///Does foo.\nfn foo() {}\n");
    }

    #[test]
    fn function_with_attribute() {
        let f = Function::from("test_it").with_attribute("test");
        assert_eq!(f.to_code(), "#[test]\nfn test_it() {}\n");
    }
}

impl Statement for Function {
    fn write(&self, b: &mut CodeBuffer, level: usize) {
        self.write_comments(OuterLineDoc, b, level);
        self.write_attributes(b, level);
        b.indent(level);
        self.write_access(b);
        if self.is_async {
            b.write("async ");
        }
        if self.is_const {
            b.write("const ");
        }
        self.signature.write_unsafe(b);
        b.write("fn ");
        self.write_signature(b);
        b.space();
        self.write_curly_statement_block(b, level);
        b.end_line();
    }
}
