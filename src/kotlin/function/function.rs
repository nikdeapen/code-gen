use crate::kotlin::{
    Access, CommentType, Modifier, Signature, WithAccess, WithAnnotations, WithComments,
    WithExpression, WithModifiers, WithSignature,
};
use crate::{CodeBuffer, Statement, WithStatements};
use std::collections::BTreeSet;

/// A function declaration.
///
/// # Note
/// - A function with an expression has an expression body. (ex: `fun one(): Int = 1`) Its
///   statements are not written.
pub struct Function {
    comments: Vec<String>,
    annotations: Vec<String>,
    access: Access,
    modifiers: BTreeSet<Modifier>,
    signature: Signature,
    expression: Option<String>,
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
            expression: None,
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

impl WithExpression for Function {
    fn expression(&self) -> Option<&str> {
        self.expression.as_deref()
    }

    fn set_expression<S>(&mut self, expression: S)
    where
        S: Into<String>,
    {
        self.expression = Some(expression.into());
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
        b.write("fun ");
        self.write_signature(b);
        if let Some(expression) = &self.expression {
            b.write(" = ");
            b.write(expression);
        } else {
            b.space();
            self.write_curly_statement_block(b, level);
        }
        b.end_line();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::kotlin::{WithResult, WithVarParams};

    #[test]
    fn empty_function() {
        let f = Function::from("reset");
        assert_eq!(f.to_code(), "fun reset() {}\n");
    }

    #[test]
    fn function_with_body() {
        let f = Function::from(
            Signature::from("add")
                .with_param(("x", "Int"))
                .with_param(("y", "Int"))
                .with_result("Int"),
        )
        .with_literal("val sum: Int = x + y")
        .with_literal("return sum");
        assert_eq!(
            f.to_code(),
            "fun add(x: Int, y: Int): Int {\n    val sum: Int = x + y\n    return sum\n}\n"
        );
    }

    #[test]
    fn function_with_expression() {
        let f = Function::from(Signature::from("one").with_result("Int"))
            .with_literal("return 2")
            .with_expression("1");
        assert_eq!(f.to_code(), "fun one(): Int = 1\n");
    }

    #[test]
    fn function_with_access_and_modifiers() {
        let f = Function::from("close")
            .with_access(Access::Internal)
            .with_modifier(Modifier::Suspend)
            .with_modifier(Modifier::Override);
        assert_eq!(f.to_code(), "internal override suspend fun close() {}\n");
    }

    #[test]
    fn function_with_comment_and_annotation() {
        let f = Function::from("reset")
            .with_comment(" Resets the count.")
            .with_annotation("JvmStatic")
            .with_literal("count = 0");
        let mut b = CodeBuffer::default();
        f.write(&mut b, 1);
        assert_eq!(
            b.peek(),
            "    /**\n     * Resets the count.\n     */\n    @JvmStatic\n    fun reset() {\n        count = 0\n    }\n"
        );
    }
}
