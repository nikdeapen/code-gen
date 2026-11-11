use crate::csharp::{Access, CommentType, Var, WithAccess, WithBody, WithComments, WithVarParams};
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
        b.push(')');
        b.end_line();
        self.write_body(b, level);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_constructor() {
        let c: Constructor = Constructor::from("Shape").with_access(Access::Private);
        assert_eq!(c.to_code(), "private Shape()\n{\n}\n");
    }

    #[test]
    fn constructor_with_params() {
        let c: Constructor = Constructor::from("Point")
            .with_comment(" Creates the point.")
            .with_access(Access::Public)
            .with_param(("x", "int"))
            .with_param(("y", "int"))
            .with_semi("_x = x")
            .with_semi("_y = y");
        let mut b: CodeBuffer = CodeBuffer::default();
        c.write(&mut b, 1);
        assert_eq!(
            b.peek(),
            "    /// <summary>\n    /// Creates the point.\n    /// </summary>\n    public Point(int x, int y)\n    {\n        _x = x;\n        _y = y;\n    }\n"
        );
    }
}
