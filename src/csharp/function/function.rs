use crate::csharp::{
    Access, CommentType, Modifier, Signature, WithAccess, WithAttributes, WithBody, WithComments,
    WithModifiers, WithSignature,
};
use crate::{CodeBuffer, Statement, WithStatements};
use std::collections::BTreeSet;

/// A function declaration.
pub struct Function {
    comments: Vec<String>,
    attributes: Vec<String>,
    access: Access,
    modifiers: BTreeSet<Modifier>,
    signature: Signature,
    statements: Vec<Box<dyn Statement>>,
}

impl<S: Into<Signature>> From<S> for Function {
    fn from(signature: S) -> Self {
        Self {
            comments: Vec::default(),
            attributes: Vec::default(),
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
        self.write_attributes(b, level);
        b.indent(level);
        self.write_access(b);
        self.write_modifiers(b);
        self.write_signature(b);
        b.end_line();
        self.write_body(b, level);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::csharp::WithVarParams;

    #[test]
    fn empty_function() {
        let f: Function = Function::from("Run");
        assert_eq!(f.to_code(), "void Run()\n{\n}\n");
    }

    #[test]
    fn function_with_body() {
        let f: Function = Function::from(
            Signature::from("Add")
                .with_param(("a", "int"))
                .with_param(("b", "int"))
                .with_result("int"),
        )
        .with_access(Access::Public)
        .with_modifier(Modifier::Static)
        .with_semi("return a + b");
        assert_eq!(
            f.to_code(),
            "public static int Add(int a, int b)\n{\n    return a + b;\n}\n"
        );
    }

    #[test]
    fn function_with_comment_and_attribute() {
        let f: Function = Function::from("Run")
            .with_comment(" Runs the task.")
            .with_attribute("Obsolete");
        assert_eq!(
            f.to_code(),
            "/// <summary>\n/// Runs the task.\n/// </summary>\n[Obsolete]\nvoid Run()\n{\n}\n"
        );
    }

    #[test]
    fn function_indented() {
        let f: Function = Function::from(Signature::from("ToString").with_result("string"))
            .with_access(Access::Public)
            .with_modifier(Modifier::Override)
            .with_semi("return _name");
        let mut b: CodeBuffer = CodeBuffer::default();
        f.write(&mut b, 1);
        assert_eq!(
            b.peek(),
            "    public override string ToString()\n    {\n        return _name;\n    }\n"
        );
    }
}
