use crate::c::{CommentType, Modifier, Signature, WithComments, WithModifiers, WithSignature};
use crate::{CodeBuffer, Statement, WithStatements};
use std::collections::BTreeSet;

/// A function definition.
pub struct Function {
    comments: Vec<String>,
    modifiers: BTreeSet<Modifier>,
    signature: Signature,
    statements: Vec<Box<dyn Statement>>,
}

impl<S: Into<Signature>> From<S> for Function {
    fn from(signature: S) -> Self {
        Self {
            comments: Vec::default(),
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
        b.indent(level);
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
    use crate::c::{CType, WithVarParams};

    #[test]
    fn empty_function() {
        let f: Function = Function::from("foo");
        assert_eq!(f.to_code(), "void foo(void) {}\n");
    }

    #[test]
    fn function_with_body() {
        let f: Function = Function::from(
            Signature::from("point_x")
                .with_result("int32_t")
                .with_param(("self", CType::from("point_t").to_const().to_pointer())),
        )
        .with_semi("return self->x");
        assert_eq!(
            f.to_code(),
            "int32_t point_x(const point_t *self) {\n    return self->x;\n}\n"
        );
    }

    #[test]
    fn function_with_modifiers() {
        let f: Function = Function::from("foo")
            .with_modifier(Modifier::Static)
            .with_modifier(Modifier::Inline);
        assert_eq!(f.to_code(), "static inline void foo(void) {}\n");
    }

    #[test]
    fn function_with_comment() {
        let f: Function = Function::from("foo").with_comment(" Does foo.");
        assert_eq!(f.to_code(), "/** Does foo. */\nvoid foo(void) {}\n");
    }

    #[test]
    fn function_indented() {
        let f: Function = Function::from("foo")
            .with_comment(" Does foo.")
            .with_semi("bar()");
        let mut b: CodeBuffer = CodeBuffer::default();
        f.write(&mut b, 1);
        assert_eq!(
            b.peek(),
            "    /** Does foo. */\n    void foo(void) {\n        bar();\n    }\n"
        );
    }
}
