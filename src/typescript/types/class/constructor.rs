use crate::typescript::{Access, CommentType, Var, WithAccess, WithComments, WithVarParams};
use crate::{CodeBuffer, Statement, WithStatements};

/// A constructor of a class.
#[derive(Default)]
pub struct Constructor {
    comments: Vec<String>,
    access: Access,
    params: Vec<Var>,
    statements: Vec<Box<dyn Statement>>,
}

impl WithComments for Constructor {
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

impl WithAccess for Constructor {
    fn access(&self) -> Access {
        self.access
    }

    fn set_access(&mut self, access: Access) {
        self.access = access;
    }
}

impl WithVarParams for Constructor {
    fn params(&self) -> &[Var] {
        self.params.as_slice()
    }

    fn add_param<V>(&mut self, param: V)
    where
        V: Into<Var>,
    {
        self.params.push(param.into());
    }
}

impl WithStatements for Constructor {
    fn statements(&self) -> &[Box<dyn Statement>] {
        self.statements.as_slice()
    }

    fn add_boxed_statement(&mut self, statement: Box<dyn Statement>) {
        self.statements.push(statement);
    }
}

impl Statement for Constructor {
    fn write(&self, b: &mut CodeBuffer, level: usize) {
        self.write_comments(CommentType::Doc, b, level);
        b.indent(level);
        self.write_access(b);
        b.write("constructor(");
        self.write_params(b);
        b.write(") ");
        self.write_curly_statement_block(b, level);
        b.end_line();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_constructor() {
        let c = Constructor::default();
        assert_eq!(c.to_code(), "constructor() {}\n");
    }

    #[test]
    fn constructor_with_body() {
        let c = Constructor::default()
            .with_param(("x", "number"))
            .with_param(("y", "number"))
            .with_semi("this.x = x")
            .with_semi("this.y = y");
        assert_eq!(
            c.to_code(),
            "constructor(x: number, y: number) {\n    this.x = x;\n    this.y = y;\n}\n"
        );
    }

    #[test]
    fn private_constructor_with_comment() {
        let c = Constructor::default()
            .with_comment(" Creates the point.")
            .with_access(Access::Private);
        assert_eq!(
            c.to_code(),
            "/** Creates the point. */\nprivate constructor() {}\n"
        );
    }
}
