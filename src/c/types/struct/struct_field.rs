use crate::c::{CType, CommentType, Var, WithCType, WithComments, WithVar};
use crate::{CodeBuffer, Statement, WithName};

/// A field of a struct or union declaration.
#[derive(Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub struct StructField {
    comments: Vec<String>,
    var: Var,
}

impl<V: Into<Var>> From<V> for StructField {
    fn from(var: V) -> Self {
        Self {
            comments: Vec::default(),
            var: var.into(),
        }
    }
}

impl WithComments for StructField {
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

impl WithName for StructField {
    fn name(&self) -> &str {
        self.var.name()
    }
}

impl WithCType for StructField {
    fn c_type(&self) -> &CType {
        self.var.c_type()
    }
}

impl WithVar for StructField {
    fn var(&self) -> &Var {
        &self.var
    }
}

impl Statement for StructField {
    fn write(&self, b: &mut CodeBuffer, level: usize) {
        self.write_comments(CommentType::Doc, b, level);
        b.indent(level);
        self.write_var(b);
        b.push(';');
        b.end_line();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_field() {
        let f: StructField = StructField::from(("x", "int32_t"));
        assert_eq!(f.to_code(), "int32_t x;\n");
    }

    #[test]
    fn array_field() {
        let f: StructField = StructField::from(("id", CType::from("uint8_t").to_array("16")));
        assert_eq!(f.to_code(), "uint8_t id[16];\n");
    }

    #[test]
    fn field_with_comment() {
        let f: StructField = StructField::from(("x", "int32_t")).with_comment(" The x.");
        assert_eq!(f.to_code(), "/** The x. */\nint32_t x;\n");
    }

    #[test]
    fn field_indented() {
        let f: StructField = StructField::from(("x", "int32_t")).with_comment(" The x.");
        let mut b: CodeBuffer = CodeBuffer::default();
        f.write(&mut b, 1);
        assert_eq!(b.peek(), "    /** The x. */\n    int32_t x;\n");
    }
}
