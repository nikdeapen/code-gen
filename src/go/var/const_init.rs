use crate::go::{Var, WithComments, WithVar};
use crate::{CodeBuffer, Expression, Statement};

/// A constant declaration. (ex: `const PointSize int = 8`)
pub struct ConstInit {
    comments: Vec<String>,
    var: Var,
    expression: Box<dyn Expression>,
}

impl<V: Into<Var>, E: 'static + Expression> From<(V, E)> for ConstInit {
    fn from(tuple: (V, E)) -> Self {
        Self {
            comments: Vec::default(),
            var: tuple.0.into(),
            expression: Box::new(tuple.1),
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

impl ConstInit {
    //! Write

    /// Writes the constant without the `const` keyword.
    pub(super) fn write_spec(&self, b: &mut CodeBuffer, level: usize) {
        self.write_after(b, level, "");
    }

    /// Writes the comments, then the constant after the `keyword`.
    fn write_after(&self, b: &mut CodeBuffer, level: usize, keyword: &str) {
        self.write_comments(b, level);
        b.indent(level);
        b.write(keyword);
        self.write_var(b);
        b.write(" = ");
        self.expression.write(b);
        b.end_line();
    }
}

impl Statement for ConstInit {
    fn write(&self, b: &mut CodeBuffer, level: usize) {
        self.write_after(b, level, "const ");
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Literal;

    #[test]
    fn const_init_statement() {
        let c = ConstInit::from((("PointSize", "int"), Literal::from("8")));
        assert_eq!(c.to_code(), "const PointSize int = 8\n");
    }

    #[test]
    fn const_init_with_comment() {
        let c = ConstInit::from((("PointSize", "int"), Literal::from("8")))
            .with_comment(" PointSize is the length of an encoded Point.");
        assert_eq!(
            c.to_code(),
            "// PointSize is the length of an encoded Point.\nconst PointSize int = 8\n"
        );
    }

    #[test]
    fn const_init_spec() {
        let c = ConstInit::from((("ColorRed", "Color"), Literal::from("1")))
            .with_comment(" ColorRed is red.");
        let mut b = CodeBuffer::default();
        c.write_spec(&mut b, 1);
        assert_eq!(
            b.peek(),
            "    // ColorRed is red.\n    ColorRed Color = 1\n"
        );
    }
}
