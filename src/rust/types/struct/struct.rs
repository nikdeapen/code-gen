use crate::rust::{
    Access, CommentType, StructField, Var, WithAccess, WithAttributes, WithComments, WithDerives,
    WithGenerics, WithStructFields,
};
use crate::{CodeBuffer, Statement, WithName};

/// A struct declaration.
#[derive(Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub struct Struct {
    comments: Vec<String>,
    derives: Vec<String>,
    attributes: Vec<String>,
    access: Access,
    name: String,
    generics: Vec<Var>,
    fields: Vec<StructField>,
}

impl<S: Into<String>> From<S> for Struct {
    fn from(name: S) -> Self {
        Self {
            comments: Vec::default(),
            derives: Vec::default(),
            attributes: Vec::default(),
            access: Access::default(),
            name: name.into(),
            generics: Vec::default(),
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

impl WithDerives for Struct {
    fn derives(&self) -> &[String] {
        self.derives.as_slice()
    }

    fn add_derive<S>(&mut self, derive: S)
    where
        S: Into<String>,
    {
        self.derives.push(derive.into());
    }
}

impl WithAttributes for Struct {
    fn attributes(&self) -> &[String] {
        self.attributes.as_slice()
    }

    fn add_attribute<S>(&mut self, attribute: S)
    where
        S: Into<String>,
    {
        self.attributes.push(attribute.into())
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

impl WithGenerics for Struct {
    fn generics(&self) -> &[Var] {
        self.generics.as_slice()
    }

    fn add_generic<V>(&mut self, generic: V)
    where
        V: Into<Var>,
    {
        self.generics.push(generic.into())
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
        self.write_derives(b, level);
        self.write_attributes(b, level);
        b.indent(level);
        self.write_access(b);
        b.write("struct ");
        self.write_name(b);
        self.write_generic_brackets(b);
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
