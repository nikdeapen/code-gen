use crate::c::{CommentType, StructField, WithComments, WithStructFields, WithTypedefName};
use crate::{CodeBuffer, Statement, WithName};

/// A struct declaration.
///
/// # Note
/// - The name is the struct's tag. (ex: the `point` of `struct point`)
/// - A struct with no fields is not standard C. GCC & Clang allow it.
#[derive(Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub struct Struct {
    comments: Vec<String>,
    name: String,
    typedef_name: Option<String>,
    fields: Vec<StructField>,
}

impl<S: Into<String>> From<S> for Struct {
    fn from(name: S) -> Self {
        Self {
            comments: Vec::default(),
            name: name.into(),
            typedef_name: None,
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

impl WithTypedefName for Struct {
    fn typedef_name(&self) -> Option<&str> {
        self.typedef_name.as_deref()
    }

    fn set_typedef_name<S>(&mut self, typedef_name: S)
    where
        S: Into<String>,
    {
        self.typedef_name = Some(typedef_name.into());
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
        self.write_comments(CommentType::Doc, b, level);
        b.indent(level);
        self.write_typedef_keyword(b);
        b.write("struct ");
        self.write_name(b);
        b.space();
        self.write_curly_field_block(b, level);
        self.write_typedef_name(b);
        b.push(';');
        b.end_line();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_struct() {
        let s: Struct = Struct::from("empty");
        assert_eq!(s.to_code(), "struct empty {};\n");
    }

    #[test]
    fn struct_with_fields() {
        let s: Struct = Struct::from("point")
            .with_field(("x", "int32_t"))
            .with_field(("y", "int32_t"));
        assert_eq!(
            s.to_code(),
            "struct point {\n    int32_t x;\n    int32_t y;\n};\n"
        );
    }

    #[test]
    fn struct_with_typedef_name() {
        let s: Struct = Struct::from("point")
            .with_typedef_name("point_t")
            .with_field(("x", "int32_t"));
        assert_eq!(s.typedef_name(), Some("point_t"));
        assert_eq!(
            s.to_code(),
            "typedef struct point {\n    int32_t x;\n} point_t;\n"
        );
    }

    #[test]
    fn struct_with_comment() {
        let s: Struct = Struct::from("empty").with_comment(" An empty struct.");
        assert_eq!(s.to_code(), "/** An empty struct. */\nstruct empty {};\n");
    }

    #[test]
    fn struct_indented() {
        let s: Struct = Struct::from("point")
            .with_comment(" A point.")
            .with_typedef_name("point_t")
            .with_field(("x", "int32_t"));
        let mut b: CodeBuffer = CodeBuffer::default();
        s.write(&mut b, 1);
        assert_eq!(
            b.peek(),
            "    /** A point. */\n    typedef struct point {\n        int32_t x;\n    } point_t;\n"
        );
    }
}
