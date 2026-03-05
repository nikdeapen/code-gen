use crate::rust::{Access, CommentType, Var, WithAccess, WithComments, WithVar};
use crate::{CodeBuffer, Expression, Statement};

/// A const initialization statement.
pub struct ConstInit {
    comments: Vec<String>,
    access: Access,
    var: Var,
    expression: Box<dyn Expression>,
}

impl<V: Into<Var>, E: 'static + Expression> From<(V, E)> for ConstInit {
    fn from(t: (V, E)) -> Self {
        Self {
            comments: Vec::default(),
            access: Access::default(),
            var: t.0.into(),
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

impl WithAccess for ConstInit {
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

impl WithVar for ConstInit {
    fn var(&self) -> &Var {
        &self.var
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Literal;

    #[test]
    fn const_init_statement() {
        let c = ConstInit::from((("MAX", "u32"), Literal::from("100")));
        assert_eq!(c.to_code(), "const MAX: u32 = 100;\n");
    }

    #[test]
    fn const_init_public() {
        let mut c = ConstInit::from((("MAX", "u32"), Literal::from("100")));
        c.set_access(Access::Public);
        assert_eq!(c.to_code(), "pub const MAX: u32 = 100;\n");
    }

    #[test]
    fn const_init_with_comment() {
        let mut c = ConstInit::from((("MAX", "u32"), Literal::from("100")));
        c.add_comment("The maximum value.");
        assert_eq!(
            c.to_code(),
            "///The maximum value.\nconst MAX: u32 = 100;\n"
        );
    }
}

impl Statement for ConstInit {
    fn write(&self, b: &mut CodeBuffer, level: usize) {
        self.write_comments(CommentType::OuterLineDoc, b, level);
        b.indent(level);
        self.write_access(b);
        b.write("const ");
        self.write_var(b);
        b.write(" = ");
        self.expression.write(b);
        b.push(';');
        b.end_line();
    }
}
