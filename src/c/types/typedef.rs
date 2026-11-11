use crate::c::{CType, CommentType, Var, WithCType, WithComments, WithVar};
use crate::{CodeBuffer, Statement, WithName};

/// A `typedef` declaration.
#[derive(Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub struct Typedef {
    comments: Vec<String>,
    var: Var,
}

impl<V: Into<Var>> From<V> for Typedef {
    fn from(var: V) -> Self {
        Self {
            comments: Vec::default(),
            var: var.into(),
        }
    }
}

impl WithComments for Typedef {
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

impl WithName for Typedef {
    fn name(&self) -> &str {
        self.var.name()
    }
}

impl WithCType for Typedef {
    fn c_type(&self) -> &CType {
        self.var.c_type()
    }
}

impl WithVar for Typedef {
    fn var(&self) -> &Var {
        &self.var
    }
}

impl Statement for Typedef {
    fn write(&self, b: &mut CodeBuffer, level: usize) {
        self.write_comments(CommentType::Doc, b, level);
        b.indent(level);
        b.write("typedef ");
        self.write_var(b);
        b.push(';');
        b.end_line();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::c::WithVarParams;

    #[test]
    fn simple_typedef() {
        let t: Typedef = Typedef::from(("color_t", "uint16_t"));
        assert_eq!(t.to_code(), "typedef uint16_t color_t;\n");
    }

    #[test]
    fn typedef_of_a_struct() {
        let t: Typedef = Typedef::from(("point_t", "struct point"));
        assert_eq!(t.to_code(), "typedef struct point point_t;\n");
    }

    #[test]
    fn typedef_of_a_function_pointer() {
        let handler: CType = CType::from("void").with_param(("context", "void *"));
        let t: Typedef = Typedef::from(("handler_t", handler.to_pointer()));
        assert_eq!(t.to_code(), "typedef void (*handler_t)(void *context);\n");
    }

    #[test]
    fn typedef_with_comment() {
        let t: Typedef = Typedef::from(("color_t", "uint16_t")).with_comment(" A color.");
        assert_eq!(t.to_code(), "/** A color. */\ntypedef uint16_t color_t;\n");
    }

    #[test]
    fn typedef_indented() {
        let t: Typedef = Typedef::from(("color_t", "uint16_t")).with_comment(" A color.");
        let mut b: CodeBuffer = CodeBuffer::default();
        t.write(&mut b, 1);
        assert_eq!(
            b.peek(),
            "    /** A color. */\n    typedef uint16_t color_t;\n"
        );
    }
}
