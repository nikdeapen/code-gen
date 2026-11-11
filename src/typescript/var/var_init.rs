use crate::typescript::{CommentType, Modifier, Var, WithComments, WithModifiers, WithVar};
use crate::{CodeBuffer, Expression, Statement};
use std::collections::BTreeSet;

/// A variable initialization statement. (ex: `const x = 1;` or `let x = 1;`)
pub struct VarInit {
    comments: Vec<String>,
    modifiers: BTreeSet<Modifier>,
    mutable: bool,
    var: Var,
    expression: Box<dyn Expression>,
}

impl<V: Into<Var>, E: 'static + Expression> From<(V, E)> for VarInit {
    fn from(t: (V, E)) -> Self {
        Self {
            comments: Vec::default(),
            modifiers: BTreeSet::default(),
            mutable: false,
            var: t.0.into(),
            expression: Box::new(t.1),
        }
    }
}

impl WithComments for VarInit {
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

impl WithModifiers for VarInit {
    fn modifiers(&self) -> &BTreeSet<Modifier> {
        &self.modifiers
    }

    fn add_modifier(&mut self, modifier: Modifier) {
        self.modifiers.insert(modifier);
    }
}

impl VarInit {
    //! Mutable

    /// Sets the mutable flag.
    pub fn set_mutable(&mut self) {
        self.mutable = true;
    }

    /// Sets the mutable flag.
    #[must_use]
    pub fn with_mutable(mut self) -> Self {
        self.set_mutable();
        self
    }
}

impl WithVar for VarInit {
    fn var(&self) -> &Var {
        &self.var
    }
}

impl Statement for VarInit {
    fn write(&self, b: &mut CodeBuffer, level: usize) {
        self.write_comments(CommentType::Doc, b, level);
        b.indent(level);
        self.write_modifiers(b);
        if self.mutable {
            b.write("let ");
        } else {
            b.write("const ");
        }
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
    fn var_init_statement() {
        let v = VarInit::from(("x", Literal::from("42")));
        assert_eq!(v.to_code(), "const x = 42;\n");
    }

    #[test]
    fn var_init_with_type() {
        let v = VarInit::from((("x", "number"), Literal::from("42")));
        assert_eq!(v.to_code(), "const x: number = 42;\n");
    }

    #[test]
    fn var_init_mutable() {
        let v = VarInit::from((("x", "number"), Literal::from("42"))).with_mutable();
        assert_eq!(v.to_code(), "let x: number = 42;\n");
    }

    #[test]
    fn var_init_exported_with_comment() {
        let v = VarInit::from((("MAX_DEPTH", "number"), Literal::from("64")))
            .with_comment(" The most nested values.")
            .with_modifier(Modifier::Export);
        assert_eq!(
            v.to_code(),
            "/** The most nested values. */\nexport const MAX_DEPTH: number = 64;\n"
        );
    }
}
