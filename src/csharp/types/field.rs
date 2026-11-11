use crate::csharp::{
    Access, CommentType, Modifier, Var, WithAccess, WithComments, WithModifiers, WithVar,
};
use crate::{CodeBuffer, Statement};
use std::collections::BTreeSet;

/// A field declaration.
///
/// # Note
/// - A constant is a field with the `const` modifier & an initializer.
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

impl Field {
    //! Initializer

    /// Gets the optional initializer.
    #[must_use]
    pub fn initializer(&self) -> Option<&str> {
        self.initializer.as_deref()
    }

    /// Sets the initializer.
    pub fn set_initializer<S>(&mut self, initializer: S)
    where
        S: Into<String>,
    {
        self.initializer = Some(initializer.into());
    }

    /// Sets the initializer.
    #[must_use]
    pub fn with_initializer<S>(mut self, initializer: S) -> Self
    where
        S: Into<String>,
    {
        self.set_initializer(initializer);
        self
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

impl WithVar for Field {
    fn var(&self) -> &Var {
        &self.var
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
            b.write(initializer.as_str());
        }
        b.push(';');
        b.end_line();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_field() {
        let f: Field = Field::from(("_x", "int"));
        assert_eq!(f.to_code(), "int _x;\n");
    }

    #[test]
    fn field_with_access_and_modifiers() {
        let f: Field = Field::from(("_x", "int"))
            .with_access(Access::Private)
            .with_modifier(Modifier::Readonly);
        let mut b: CodeBuffer = CodeBuffer::default();
        f.write(&mut b, 1);
        assert_eq!(b.peek(), "    private readonly int _x;\n");
    }

    #[test]
    fn constant_with_comment() {
        let f: Field = Field::from(("MaxDepth", "int"))
            .with_comment(" The most levels of nesting.")
            .with_access(Access::Public)
            .with_modifier(Modifier::Const)
            .with_initializer("64");
        assert_eq!(
            f.to_code(),
            "/// <summary>\n/// The most levels of nesting.\n/// </summary>\npublic const int MaxDepth = 64;\n"
        );
    }
}
