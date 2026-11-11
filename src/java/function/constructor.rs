use crate::java::{Access, CommentType, Var, WithAccess, WithComments, WithVarParams};
use crate::{CodeBuffer, Statement, WithName, WithStatements};

/// A constructor declaration.
///
/// # Note
/// - Its name is the name of its type.
pub struct Constructor {
    comments: Vec<String>,
    access: Access,
    name: String,
    params: Vec<Var>,
    statements: Vec<Box<dyn Statement>>,
}

impl<S: Into<String>> From<S> for Constructor {
    fn from(name: S) -> Self {
        Self {
            comments: Vec::default(),
            access: Access::default(),
            name: name.into(),
            params: Vec::default(),
            statements: Vec::default(),
        }
    }
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

impl WithName for Constructor {
    fn name(&self) -> &str {
        self.name.as_str()
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
        self.write_name(b);
        b.push('(');
        self.write_params(b);
        b.write(") ");
        self.write_curly_statement_block(b, level);
        b.end_line();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::java::JavaPrimitive;

    #[test]
    fn empty_constructor() {
        let c: Constructor = Constructor::from("Point");
        assert_eq!(c.to_code(), "Point() {}\n");
    }

    #[test]
    fn constructor_with_params() {
        let c: Constructor = Constructor::from("Point")
            .with_comment(" Creates a point.")
            .with_access(Access::Public)
            .with_param(("x", JavaPrimitive::Int))
            .with_param(("y", JavaPrimitive::Int))
            .with_semi("this.x = x")
            .with_semi("this.y = y");
        assert_eq!(
            c.to_code(),
            "/**\n * Creates a point.\n */\npublic Point(int x, int y) {\n    this.x = x;\n    this.y = y;\n}\n"
        );
    }
}
