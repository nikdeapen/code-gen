use crate::cpp::{
    Access, CommentType, Modifier, Var, WithAccess, WithComments, WithModifiers, WithVar,
};
use crate::{CodeBuffer, Expression, Statement};
use std::collections::BTreeSet;

/// A `constexpr` constant initialization statement. (ex: `constexpr int MAX = 100;`)
///
/// # Note
/// - A constant of a class needs the `static` modifier.
pub struct ConstInit {
    comments: Vec<String>,
    access: Access,
    modifiers: BTreeSet<Modifier>,
    var: Var,
    expression: Box<dyn Expression>,
}

impl<V: Into<Var>, E: 'static + Expression> From<(V, E)> for ConstInit {
    fn from(t: (V, E)) -> Self {
        Self {
            comments: Vec::default(),
            access: Access::default(),
            modifiers: BTreeSet::from([Modifier::Constexpr]),
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
    fn access(&self) -> Access {
        self.access
    }

    fn set_access(&mut self, access: Access) {
        self.access = access;
    }
}

impl WithModifiers for ConstInit {
    fn modifiers(&self) -> &BTreeSet<Modifier> {
        &self.modifiers
    }

    fn add_modifier(&mut self, modifier: Modifier) {
        self.modifiers.insert(modifier);
    }
}

impl WithVar for ConstInit {
    fn var(&self) -> &Var {
        &self.var
    }
}

impl Statement for ConstInit {
    fn write(&self, b: &mut CodeBuffer, level: usize) {
        self.write_comments(CommentType::LineDoc, b, level);
        b.indent(level);
        self.write_modifiers(b);
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

    #[test]
    fn const_init_statement() {
        let c = ConstInit::from((("MAX", "std::uint32_t"), Literal::from("100")));
        assert_eq!(c.to_code(), "constexpr std::uint32_t MAX = 100;\n");
    }

    #[test]
    fn const_init_static() {
        let c = ConstInit::from((("MAX", "std::uint32_t"), Literal::from("100")))
            .with_modifier(Modifier::Static)
            .with_access(Access::Public);
        assert_eq!(c.access(), Access::Public);
        assert_eq!(c.to_code(), "static constexpr std::uint32_t MAX = 100;\n");
    }

    #[test]
    fn const_init_inline() {
        let c = ConstInit::from((("MAX", "std::uint32_t"), Literal::from("100")))
            .with_modifier(Modifier::Constexpr)
            .with_modifier(Modifier::Inline);
        assert_eq!(c.to_code(), "inline constexpr std::uint32_t MAX = 100;\n");
    }

    #[test]
    fn const_init_with_comment() {
        let c = ConstInit::from((("MAX", "std::uint32_t"), Literal::from("100")))
            .with_comment(" The maximum value.");
        let mut b = CodeBuffer::default();
        c.write(&mut b, 1);
        assert_eq!(
            b.peek(),
            "    /// The maximum value.\n    constexpr std::uint32_t MAX = 100;\n"
        );
    }
}
