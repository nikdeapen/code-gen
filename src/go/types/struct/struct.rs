use crate::go::{GoType, StructField, WithComments, WithEmbedded, write_members};
use crate::{CodeBuffer, Statement, WithName};

/// A struct declaration.
#[derive(Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub struct Struct {
    comments: Vec<String>,
    name: String,
    embedded: Vec<GoType>,
    fields: Vec<StructField>,
}

impl<S: Into<String>> From<S> for Struct {
    fn from(name: S) -> Self {
        Self {
            comments: Vec::default(),
            name: name.into(),
            embedded: Vec::default(),
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

impl WithName for Struct {
    fn name(&self) -> &str {
        self.name.as_str()
    }
}

impl WithEmbedded for Struct {
    fn embedded(&self) -> &[GoType] {
        self.embedded.as_slice()
    }

    fn add_embedded<T>(&mut self, embedded: T)
    where
        T: Into<GoType>,
    {
        self.embedded.push(embedded.into());
    }
}

impl Struct {
    //! Fields

    /// Gets the fields.
    #[must_use]
    pub fn fields(&self) -> &[StructField] {
        self.fields.as_slice()
    }

    /// Adds the field.
    pub fn add_field<F>(&mut self, field: F)
    where
        F: Into<StructField>,
    {
        self.fields.push(field.into());
    }

    /// Adds the field.
    #[must_use]
    pub fn with_field<F>(mut self, field: F) -> Self
    where
        F: Into<StructField>,
    {
        self.add_field(field);
        self
    }
}

impl Statement for Struct {
    fn write(&self, b: &mut CodeBuffer, level: usize) {
        self.write_comments(b, level);
        b.indent(level);
        b.write("type ");
        self.write_name(b);
        b.write(" struct");
        if self.embedded.is_empty() && self.fields.is_empty() {
            b.write("{}");
            b.end_line();
        } else {
            b.write(" {");
            b.end_line();
            self.write_embedded(b, level + 1);
            if !self.embedded.is_empty() && !self.fields.is_empty() {
                b.end_line();
            }
            write_members(self.fields.as_slice(), StructField::write, b, level + 1);
            b.line(level, "}");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_struct() {
        let s = Struct::from("Empty");
        assert_eq!(s.to_code(), "type Empty struct{}\n");
    }

    #[test]
    fn struct_with_fields() {
        let s = Struct::from("Point")
            .with_field(("x", "uint32"))
            .with_field(("y", "uint32"));
        assert_eq!(
            s.to_code(),
            "type Point struct {\n    x uint32\n    y uint32\n}\n"
        );
    }

    #[test]
    fn struct_with_field_comments() {
        let s = Struct::from("Point")
            .with_field(StructField::from(("x", "uint32")).with_comment(" The x coordinate."))
            .with_field(StructField::from(("y", "uint32")).with_comment(" The y coordinate."));
        assert_eq!(
            s.to_code(),
            "type Point struct {\n    // The x coordinate.\n    x uint32\n\n    // The y coordinate.\n    y uint32\n}\n"
        );
    }

    #[test]
    fn struct_with_embedded_type() {
        let s = Struct::from("PointBuf").with_embedded("PointRef");
        assert_eq!(s.to_code(), "type PointBuf struct {\n    PointRef\n}\n");
    }

    #[test]
    fn struct_with_embedded_type_and_fields() {
        let s = Struct::from("PointMut")
            .with_embedded("PointRef")
            .with_field(("dirty", "bool"));
        assert_eq!(
            s.to_code(),
            "type PointMut struct {\n    PointRef\n\n    dirty bool\n}\n"
        );
    }

    #[test]
    fn struct_with_comment() {
        let s = Struct::from("Empty").with_comment(" Empty is a struct with no fields.");
        assert_eq!(
            s.to_code(),
            "// Empty is a struct with no fields.\ntype Empty struct{}\n"
        );
    }
}
