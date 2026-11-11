use crate::java::{
    Access, CommentType, Modifier, Signature, WithAccess, WithAnnotations, WithComments,
    WithModifiers, WithSignature,
};
use crate::{CodeBuffer, Statement, WithStatements};
use std::collections::BTreeSet;

/// A function declaration.
pub struct Function {
    comments: Vec<String>,
    annotations: Vec<String>,
    access: Access,
    modifiers: BTreeSet<Modifier>,
    signature: Signature,
    statements: Vec<Box<dyn Statement>>,
}

impl<S: Into<Signature>> From<S> for Function {
    fn from(signature: S) -> Self {
        Self {
            comments: Vec::default(),
            annotations: Vec::default(),
            access: Access::default(),
            modifiers: BTreeSet::default(),
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

impl WithAnnotations for Function {
    fn annotations(&self) -> &[String] {
        self.annotations.as_slice()
    }

    fn add_annotation<S>(&mut self, annotation: S)
    where
        S: Into<String>,
    {
        self.annotations.push(annotation.into());
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

impl WithModifiers for Function {
    fn modifiers(&self) -> &BTreeSet<Modifier> {
        &self.modifiers
    }

    fn add_modifier(&mut self, modifier: Modifier) {
        self.modifiers.insert(modifier);
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
        self.write_comments(CommentType::Doc, b, level);
        self.write_annotations(b, level);
        b.indent(level);
        self.write_access(b);
        self.write_modifiers(b);
        self.write_signature(b);
        b.space();
        self.write_curly_statement_block(b, level);
        b.end_line();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::java::{JavaPrimitive, WithResult, WithVarParams};

    #[test]
    fn empty_function() {
        let f: Function = Function::from("reset");
        assert_eq!(f.to_code(), "void reset() {}\n");
    }

    #[test]
    fn function_with_body() {
        let f: Function = Function::from(
            Signature::from("setX")
                .with_param(("x", JavaPrimitive::Int))
                .with_result("Point"),
        )
        .with_access(Access::Public)
        .with_semi("this.x = x")
        .with_semi("return this");
        assert_eq!(
            f.to_code(),
            "public Point setX(int x) {\n    this.x = x;\n    return this;\n}\n"
        );
    }

    #[test]
    fn function_with_modifiers() {
        let f: Function = Function::from("reset")
            .with_access(Access::Private)
            .with_modifier(Modifier::Final)
            .with_modifier(Modifier::Static);
        assert_eq!(f.to_code(), "private static final void reset() {}\n");
    }

    #[test]
    fn function_with_comment_and_annotation() {
        let f: Function = Function::from(Signature::from("x").with_result(JavaPrimitive::Int))
            .with_comment(" Gets the x.")
            .with_annotation("Override")
            .with_access(Access::Public)
            .with_semi("return this.x");
        assert_eq!(
            f.to_code(),
            "/**\n * Gets the x.\n */\n@Override\npublic int x() {\n    return this.x;\n}\n"
        );
    }

    #[test]
    fn function_indented() {
        let f: Function = Function::from("reset")
            .with_comment(" Resets the point.")
            .with_semi("this.x = 0");
        let mut b: CodeBuffer = CodeBuffer::default();
        f.write(&mut b, 1);
        assert_eq!(
            b.peek(),
            "    /**\n     * Resets the point.\n     */\n    void reset() {\n        this.x = 0;\n    }\n"
        );
    }
}
