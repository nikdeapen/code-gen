use crate::rust::{Access, CommentType, StructField, WithAccess, WithComments, WithStructFields};
use crate::{CodeBuffer, Statement, WithName};

/// A struct declaration.
#[derive(Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub struct Struct {
    comments: Vec<String>,
    access: Access,
    name: String,
    fields: Vec<StructField>,
}

impl<S: Into<String>> From<S> for Struct {
    fn from(name: S) -> Self {
        Self {
            comments: Vec::default(),
            access: Access::default(),
            name: name.into(),
            fields: Vec::default(),
        }
    }
}

impl WithComments for Struct {
    fn comments(&self) -> &[String] {
        self.comments.as_slice()
    }

    fn add_comment<S>(&mut self, comment: S)
    where
        S: Into<String>,
    {
        self.comments.push(comment.into())
    }
}

impl WithAccess for Struct {
    fn access(&self) -> &Access {
        &self.access
    }

    fn set_access<A>(&mut self, access: A)
    where
        A: Into<Access>,
    {
        self.access = access.into()
    }
}

impl WithName for Struct {
    fn name(&self) -> &str {
        self.name.as_str()
    }
}

impl WithStructFields for Struct {
    fn fields(&self) -> &[StructField] {
        self.fields.as_slice()
    }

    fn add_field<F>(&mut self, field: F)
    where
        F: Into<StructField>,
    {
        self.fields.push(field.into());
    }
}

impl Statement for Struct {
    fn write(&self, b: &mut CodeBuffer, level: usize) {
        self.write_comments(CommentType::OuterLineDoc, b, level);
        b.indent(level);
        self.write_access(b);
        b.write("struct ");
        self.write_name(b);
        b.write(" {");
        if self.fields.is_empty() {
            b.write("}");
            b.end_line();
        } else {
            b.end_line();
            self.write_fields(b, level + 1);
            b.line(level, "}");
        }
    }
}
