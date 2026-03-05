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
        self.comments.push(comment.into());
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
        self.attributes.push(attribute.into());
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
        self.access = access.into();
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
        self.generics.push(generic.into());
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rust::WithComments;

    #[test]
    fn empty_struct() {
        let s = Struct::from("Empty");
        assert_eq!(s.to_code(), "struct Empty {}\n");
    }

    #[test]
    fn public_struct_with_fields() {
        let s = Struct::from("Point")
            .with_access(Access::Public)
            .with_field(StructField::from(("x", "f64")).with_access(Access::Public))
            .with_field(StructField::from(("y", "f64")).with_access(Access::Public));
        assert_eq!(
            s.to_code(),
            "pub struct Point {\n    pub x: f64,\n    pub y: f64,\n}\n"
        );
    }

    #[test]
    fn struct_with_derives() {
        let s = Struct::from("Foo")
            .with_derive("Clone")
            .with_derive("Debug");
        assert_eq!(s.to_code(), "#[derive(Clone, Debug)]\nstruct Foo {}\n");
    }

    #[test]
    fn struct_with_generics() {
        let s = Struct::from("Wrapper")
            .with_generic(Var::from(("T", "Clone")))
            .with_field(StructField::from(("value", "T")));
        assert_eq!(
            s.to_code(),
            "struct Wrapper<T: Clone> {\n    value: T,\n}\n"
        );
    }

    #[test]
    fn struct_with_comment() {
        let s = Struct::from("Foo").with_comment("A foo.");
        assert_eq!(s.to_code(), "///A foo.\nstruct Foo {}\n");
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
            b.push('}');
            b.end_line();
        } else {
            b.end_line();
            self.write_fields(b, level + 1);
            b.line(level, "}");
        }
    }
}
