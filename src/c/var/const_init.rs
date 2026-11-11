use crate::c::{CommentType, Var, WithComments, WithVar};
use crate::{CodeBuffer, Expression, Statement};

/// A `static const` initialization statement.
///
/// # Note
/// - The variable is made `const`. (ex: a `const char *` is written `const char *const`)
pub struct ConstInit {
    comments: Vec<String>,
    var: Var,
    expression: Box<dyn Expression>,
}

impl<V: Into<Var>, E: 'static + Expression> From<(V, E)> for ConstInit {
    fn from(t: (V, E)) -> Self {
        let var: Var = t.0.into();
        Self {
            comments: Vec::default(),
            var: var.into_const(),
            expression: Box::new(t.1),
        }
    }
}

impl WithComments for ConstInit {
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

impl WithVar for ConstInit {
    fn var(&self) -> &Var {
        &self.var
    }
}

impl Statement for ConstInit {
    fn write(&self, b: &mut CodeBuffer, level: usize) {
        self.write_comments(CommentType::Doc, b, level);
        b.indent(level);
        b.write("static ");
        self.write_var(b);
        b.write(" = ");
        self.expression.write(b);
        b.push(';');
        b.end_line();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Literal;
    use crate::c::CType;

    #[test]
    fn const_init_statement() {
        let c: ConstInit = ConstInit::from((("MAX", "uint32_t"), Literal::from("100")));
        assert_eq!(c.to_code(), "static const uint32_t MAX = 100;\n");
    }

    #[test]
    fn const_init_pointer() {
        let name: CType = CType::from("char").to_const().to_pointer();
        let c: ConstInit = ConstInit::from((("NAME", name), Literal::from("\"point\"")));
        assert_eq!(c.to_code(), "static const char *const NAME = \"point\";\n");
    }

    #[test]
    fn const_init_with_comment() {
        let c: ConstInit = ConstInit::from((("MAX", "uint32_t"), Literal::from("100")))
            .with_comment(" The maximum value.");
        assert_eq!(
            c.to_code(),
            "/** The maximum value. */\nstatic const uint32_t MAX = 100;\n"
        );
    }

    #[test]
    fn const_init_indented() {
        let c: ConstInit = ConstInit::from((("MAX", "uint32_t"), Literal::from("100")));
        let mut b: CodeBuffer = CodeBuffer::default();
        c.write(&mut b, 1);
        assert_eq!(b.peek(), "    static const uint32_t MAX = 100;\n");
    }
}
