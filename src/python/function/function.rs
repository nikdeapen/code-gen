use crate::python::{Signature, WithComments, WithDecorators, WithIndentedBlock, WithSignature};
use crate::{CodeBuffer, Statement, WithStatements};

/// A function declaration.
pub struct Function {
    decorators: Vec<String>,
    signature: Signature,
    comments: Vec<String>,
    statements: Vec<Box<dyn Statement>>,
}

impl<S: Into<Signature>> From<S> for Function {
    fn from(signature: S) -> Self {
        Self {
            decorators: Vec::default(),
            signature: signature.into(),
            comments: Vec::default(),
            statements: Vec::default(),
        }
    }
}

impl WithDecorators for Function {
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

impl WithSignature for Function {
    fn signature(&self) -> &Signature {
        &self.signature
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
        self.write_decorators(b, level);
        b.indent(level);
        self.write_signature(b);
        b.push(':');
        b.end_line();
        self.write_comments(b, level + 1);
        // A docstring is a statement. A body with one needs no `pass`.
        if self.comments.is_empty() || !self.statements.is_empty() {
            self.write_indented_statement_block(b, level);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::python::Receiver;

    #[test]
    fn empty_function() {
        let f = Function::from("foo");
        assert_eq!(f.to_code(), "def foo():\n    pass\n");
    }

    #[test]
    fn function_with_body() {
        let f = Function::from(
            Signature::from("area")
                .with_receiver(Receiver::Instance)
                .with_result("int"),
        )
        .with_literal("return self.width * self.height");
        assert_eq!(
            f.to_code(),
            "def area(self) -> int:\n    return self.width * self.height\n"
        );
    }

    #[test]
    fn function_with_comment() {
        let f = Function::from("foo").with_comment("Does foo.");
        assert_eq!(f.to_code(), "def foo():\n    \"\"\"Does foo.\"\"\"\n");
    }

    #[test]
    fn function_with_comment_and_body() {
        let f = Function::from("foo")
            .with_comment("Does foo.")
            .with_literal("bar()");
        assert_eq!(
            f.to_code(),
            "def foo():\n    \"\"\"Does foo.\"\"\"\n    bar()\n"
        );
    }

    #[test]
    fn function_with_decorators() {
        let f = Function::from("foo")
            .with_decorator("staticmethod")
            .with_decorator("cache");
        assert_eq!(f.to_code(), "@staticmethod\n@cache\ndef foo():\n    pass\n");
    }

    #[test]
    fn async_function() {
        let f =
            Function::from(Signature::from("fetch").with_async(true)).with_literal("await bar()");
        assert_eq!(f.to_code(), "async def fetch():\n    await bar()\n");
    }

    #[test]
    fn function_indented() {
        let f = Function::from("foo")
            .with_decorator("staticmethod")
            .with_comment("Does foo.")
            .with_literal("bar()");
        let mut b = CodeBuffer::default();
        f.write(&mut b, 1);
        assert_eq!(
            b.peek(),
            "    @staticmethod\n    def foo():\n        \"\"\"Does foo.\"\"\"\n        bar()\n"
        );
    }
}
