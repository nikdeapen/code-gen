use crate::typescript::{
    Access, CommentType, Modifier, TypeScriptType, Var, WithAccess, WithComments, WithModifiers,
    WithTypeScriptType, WithVar,
};
use crate::{CodeBuffer, Statement, WithName};
use std::collections::BTreeSet;

/// A field of a class or an interface. (ex: `readonly x: number;`)
#[derive(Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub struct Field {
    comments: Vec<String>,
    access: Access,
    modifiers: BTreeSet<Modifier>,
    var: Var,
    initializer: Option<String>,
}

impl<V: Into<Var>> From<V> for Field {
    fn from(var: V) -> Self {
        Self {
            comments: Vec::default(),
            access: Access::default(),
            modifiers: BTreeSet::default(),
            var: var.into(),
            initializer: None,
        }
    }
}

impl WithComments for Field {
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

impl WithAccess for Field {
    fn access(&self) -> Access {
        self.access
    }

    fn set_access(&mut self, access: Access) {
        self.access = access;
    }
}

impl WithModifiers for Field {
    fn modifiers(&self) -> &BTreeSet<Modifier> {
        &self.modifiers
    }

    fn add_modifier(&mut self, modifier: Modifier) {
        self.modifiers.insert(modifier);
    }
}

impl WithName for Field {
    fn name(&self) -> &str {
        self.var.name()
    }
}

impl WithTypeScriptType for Field {
    fn typescript_type(&self) -> Option<&TypeScriptType> {
        self.var.typescript_type()
    }
}

impl WithVar for Field {
    fn var(&self) -> &Var {
        &self.var
    }
}

impl Field {
    //! Initializer

    /// Gets the optional initializer.
    #[must_use]
    pub fn initializer(&self) -> Option<&str> {
        self.initializer.as_deref()
    }

    /// Sets the `initializer`.
    pub fn set_initializer<S>(&mut self, initializer: S)
    where
        S: Into<String>,
    {
        self.initializer = Some(initializer.into());
    }

    /// Sets the `initializer`.
    #[must_use]
    pub fn with_initializer<S>(mut self, initializer: S) -> Self
    where
        S: Into<String>,
    {
        self.set_initializer(initializer);
        self
    }
}

impl Statement for Field {
    fn write(&self, b: &mut CodeBuffer, level: usize) {
        self.write_comments(CommentType::Doc, b, level);
        b.indent(level);
        self.write_access(b);
        self.write_modifiers(b);
        self.write_var(b);
        if let Some(initializer) = &self.initializer {
            b.write(" = ");
            b.write(initializer);
        }
        b.push(';');
        b.end_line();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn field_without_type() {
        let f = Field::from("x");
        assert_eq!(f.to_code(), "x;\n");
    }

    #[test]
    fn field_with_type() {
        let f = Field::from(("x", "number"));
        assert_eq!(f.to_code(), "x: number;\n");
    }

    #[test]
    fn optional_field() {
        let f = Field::from(Var::from(("name", "string")).with_optional(true));
        assert_eq!(f.to_code(), "name?: string;\n");
    }

    #[test]
    fn field_with_access_and_modifier() {
        let f = Field::from(("x", "number"))
            .with_access(Access::Private)
            .with_modifier(Modifier::Readonly);
        assert_eq!(f.to_code(), "private readonly x: number;\n");
    }

    #[test]
    fn field_with_initializer() {
        let f = Field::from("ZERO")
            .with_modifier(Modifier::Static)
            .with_modifier(Modifier::Readonly)
            .with_initializer("new Point(0, 0)");
        assert_eq!(f.to_code(), "static readonly ZERO = new Point(0, 0);\n");
        assert_eq!(f.initializer(), Some("new Point(0, 0)"));
    }

    #[test]
    fn field_with_comment() {
        let f = Field::from(("x", "number")).with_comment(" The x coordinate.");
        let mut b = CodeBuffer::default();
        f.write(&mut b, 1);
        assert_eq!(b.peek(), "    /** The x coordinate. */\n    x: number;\n");
    }
}
