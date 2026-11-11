use crate::c::{CommentType, StructField, WithComments, WithStructFields, WithTypedefName};
use crate::{CodeBuffer, Statement, WithName};

/// A union declaration.
///
/// # Note
/// - The name is the union's tag. (ex: the `value` of `union value`)
/// - A union with no fields is not standard C. GCC & Clang allow it.
#[derive(Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub struct Union {
    comments: Vec<String>,
    name: String,
    typedef_name: Option<String>,
    fields: Vec<StructField>,
}

impl<S: Into<String>> From<S> for Union {
    fn from(name: S) -> Self {
        Self {
            comments: Vec::default(),
            name: name.into(),
            typedef_name: None,
            fields: Vec::default(),
        }
    }
}

impl WithComments for Union {
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

impl WithName for Union {
    fn name(&self) -> &str {
        self.name.as_str()
    }
}

impl WithTypedefName for Union {
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

impl WithStructFields for Union {
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

impl Statement for Union {
    fn write(&self, b: &mut CodeBuffer, level: usize) {
        self.write_comments(CommentType::Doc, b, level);
        b.indent(level);
        self.write_typedef_keyword(b);
        b.write("union ");
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
    fn empty_union() {
        let u: Union = Union::from("empty");
        assert_eq!(u.to_code(), "union empty {};\n");
    }

    #[test]
    fn union_with_fields() {
        let u: Union = Union::from("value")
            .with_field(("number", "uint64_t"))
            .with_field(("flag", "bool"));
        assert_eq!(
            u.to_code(),
            "union value {\n    uint64_t number;\n    bool flag;\n};\n"
        );
    }

    #[test]
    fn union_with_typedef_name() {
        let u: Union = Union::from("value")
            .with_typedef_name("value_t")
            .with_field(("number", "uint64_t"));
        assert_eq!(u.typedef_name(), Some("value_t"));
        assert_eq!(
            u.to_code(),
            "typedef union value {\n    uint64_t number;\n} value_t;\n"
        );
    }

    #[test]
    fn union_with_comment() {
        let u: Union = Union::from("empty").with_comment(" An empty union.");
        assert_eq!(u.to_code(), "/** An empty union. */\nunion empty {};\n");
    }

    #[test]
    fn union_indented() {
        let u: Union = Union::from("value")
            .with_comment(" A value.")
            .with_typedef_name("value_t")
            .with_field(("number", "uint64_t"));
        let mut b: CodeBuffer = CodeBuffer::default();
        u.write(&mut b, 1);
        assert_eq!(
            b.peek(),
            "    /** A value. */\n    typedef union value {\n        uint64_t number;\n    } value_t;\n"
        );
    }
}
