use crate::rust::CommentType::OuterLineDoc;
use crate::rust::{Access, TypeTag, Var, WithAccess, WithComments, WithTypeTag, WithVar};
use crate::{CodeBuffer, Statement, WithName};

/// A type declaration.
#[derive(Debug)]
pub struct TypeDec {
    comments: Vec<String>,
    access: Access,
    var: Var,
}

impl<V: Into<Var>> From<V> for TypeDec {
    fn from(var: V) -> Self {
        Self {
            comments: Vec::default(),
            access: Access::default(),
            var: var.into(),
        }
    }
}

impl WithComments for TypeDec {
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

impl WithAccess for TypeDec {
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

impl WithName for TypeDec {
    fn name(&self) -> &str {
        self.var.name()
    }
}

impl WithTypeTag for TypeDec {
    fn type_tag(&self) -> &TypeTag {
        self.var.type_tag()
    }
}

impl WithVar for TypeDec {
    fn var(&self) -> &Var {
        &self.var
    }
}

impl Statement for TypeDec {
    fn write(&self, b: &mut CodeBuffer, level: usize) {
        self.write_comments(OuterLineDoc, b, level);
        b.indent(level);
        b.write("type ");
        self.write_name(b);
        b.write(" = ");
        self.write_type_tag(b);
        b.write(";");
        b.end_line();
    }
}