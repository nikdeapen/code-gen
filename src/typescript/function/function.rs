use crate::typescript::{
    Access, CommentType, Modifier, Signature, WithAccess, WithComments, WithModifiers,
    WithSignature,
};
use crate::{CodeBuffer, Statement, WithStatements};
use std::collections::BTreeSet;

/// A function declaration.
///
/// # Note
/// - As a statement it is a top-level `function`. A class writes it as a method.
pub struct Function {
    comments: Vec<String>,
    access: Access,
    modifiers: BTreeSet<Modifier>,
    signature: Signature,
    statements: Vec<Box<dyn Statement>>,
}

impl<S: Into<Signature>> From<S> for Function {
    fn from(signature: S) -> Self {
        Self {
            comments: Vec::default(),
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

impl Function {
    //! Write

    /// Writes the function as a method of a class.
    pub fn write_method(&self, b: &mut CodeBuffer, level: usize) {
        self.write_declaration(b, level, "");
    }

    /// Writes the declaration, with the `keyword` before its signature.
    fn write_declaration(&self, b: &mut CodeBuffer, level: usize, keyword: &str) {
        self.write_comments(CommentType::Doc, b, level);
        b.indent(level);
        self.write_access(b);
        self.write_modifiers(b);
        b.write(keyword);
        self.write_signature(b);
        b.space();
        self.write_curly_statement_block(b, level);
        b.end_line();
    }
}

impl Statement for Function {
    fn write(&self, b: &mut CodeBuffer, level: usize) {
        self.write_declaration(b, level, "function ");
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::typescript::{TypeScriptType, WithVarParams};

    /// Writes the `function` as a method of a class.
    fn method(function: &Function) -> String {
        let mut b: CodeBuffer = CodeBuffer::default();
        function.write_method(&mut b, 1);
        b.into()
    }

    #[test]
    fn empty_function() {
        let f = Function::from("foo");
        assert_eq!(f.to_code(), "function foo() {}\n");
    }

    #[test]
    fn function_with_body() {
        let f = Function::from(
            Signature::from("add")
                .with_param(("a", "number"))
                .with_param(("b", "number"))
                .with_result("number"),
        )
        .with_semi("return a + b");
        assert_eq!(
            f.to_code(),
            "function add(a: number, b: number): number {\n    return a + b;\n}\n"
        );
    }

    #[test]
    fn function_with_modifiers() {
        let f = Function::from(
            Signature::from("fetch")
                .with_result(TypeScriptType::from("Promise").with_generic("void")),
        )
        .with_modifier(Modifier::Async)
        .with_modifier(Modifier::Export);
        assert_eq!(
            f.to_code(),
            "export async function fetch(): Promise<void> {}\n"
        );
    }

    #[test]
    fn function_with_comment() {
        let f = Function::from("foo").with_comment(" Does foo.");
        assert_eq!(f.to_code(), "/** Does foo. */\nfunction foo() {}\n");
    }

    #[test]
    fn method_with_body() {
        let f = Function::from(Signature::from("length").with_result("number"))
            .with_comment(" Gets the length.")
            .with_semi("return this.values.length");
        assert_eq!(
            method(&f),
            "    /** Gets the length. */\n    length(): number {\n        return this.values.length;\n    }\n"
        );
    }

    #[test]
    fn method_with_access_and_modifiers() {
        let f = Function::from("foo")
            .with_access(Access::Private)
            .with_modifier(Modifier::Async)
            .with_modifier(Modifier::Static);
        assert_eq!(method(&f), "    private static async foo() {}\n");
    }

    #[test]
    fn getter_and_setter() {
        let get = Function::from(Signature::from("x").with_result("number"))
            .with_modifier(Modifier::Get)
            .with_semi("return this.#x");
        assert_eq!(
            method(&get),
            "    get x(): number {\n        return this.#x;\n    }\n"
        );

        let set = Function::from(Signature::from("x").with_param(("x", "number")))
            .with_modifier(Modifier::Set)
            .with_semi("this.#x = x");
        assert_eq!(
            method(&set),
            "    set x(x: number) {\n        this.#x = x;\n    }\n"
        );
    }
}
