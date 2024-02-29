use crate::rust::{
    Access, CommentType, Signature, WithAccess, WithComments, WithSignature, WithUnsafeFlag,
};
use crate::{CodeBuffer, Statement, WithStatements};

/// A function declaration.
pub struct Function {
    comments: Vec<String>,
    access: Access,
    is_unsafe: bool,
    signature: Signature,
    statements: Vec<Box<dyn Statement>>,
}

impl<S: Into<Signature>> From<S> for Function {
    fn from(signature: S) -> Self {
        Self {
            comments: Vec::default(),
            access: Access::default(),
            is_unsafe: false,
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

impl WithUnsafeFlag for Function {
    fn is_unsafe(&self) -> bool {
        self.is_unsafe
    }

    fn set_unsafe(&mut self) {
        self.is_unsafe = true;
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
        self.write_comments(CommentType::OuterLineDoc, b, level);
        b.indent(level);
        self.write_access(b);
        if self.is_unsafe {
            b.write("unsafe ");
        }
        b.write("fn ");
        self.write_signature(b);
        b.space();
        self.write_curly_statement_block(b, level);
        b.end_line();
    }
}

#[cfg(test)]
mod tests {
    use crate::rust::Access::PublicInCrate;
    use crate::rust::{Function, WithAccess, WithComments, WithUnsafeFlag};
    use crate::{CodeBuffer, Literal, WithStatements};

    #[test]
    fn write_empty() {
        let function: Function = "myFn".into();
        let result: String = CodeBuffer::display_statement(&function);
        assert_eq!(result, "fn myFn() {}\n");
    }

    #[test]
    fn write_comments() {
        let function: Function = Function::from("myFn")
            .with_comment("The first comment line.")
            .with_comment("The second comment line.");
        let result: String = CodeBuffer::display_statement(&function);
        assert_eq!(
            result,
            "/// The first comment line.\n/// The second comment line.\nfn myFn() {}\n"
        );
    }

    #[test]
    fn write_access() {
        let function: Function = Function::from("myFn").with_access(PublicInCrate);
        let result: String = CodeBuffer::display_statement(&function);
        assert_eq!(result, "pub(crate) fn myFn() {}\n");
    }

    #[test]
    fn write_unsafe() {
        let function: Function = Function::from("myFn").with_unsafe();
        let result: String = CodeBuffer::display_statement(&function);
        assert_eq!(result, "unsafe fn myFn() {}\n");
    }

    #[test]
    fn write_statements() {
        let function: Function =
            Function::from("myFn").with_expression_statement(Literal::from("one"));
        let result: String = CodeBuffer::display_statement(&function);
        assert_eq!(result, "fn myFn() {\n\tone\n}\n");

        let function: Function = function.with_expression_statement(Literal::from("two"));
        let result: String = CodeBuffer::display_statement(&function);
        assert_eq!(result, "fn myFn() {\n\tone\n\ttwo\n}\n");
    }
}
