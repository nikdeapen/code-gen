use crate::java::{
    Access, CommentType, JavaType, Modifier, Var, WithAccess, WithComments, WithJavaType,
    WithModifiers,
};
use crate::{CodeBuffer, Expression, Statement, WithName};
use std::collections::BTreeSet;

/// A field of a type declaration.
///
/// # Note
/// - A constant is a `static final` field with an initializer.
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

impl WithJavaType for Field {
    fn java_type(&self) -> &JavaType {
        self.var.java_type()
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

impl Statement for Field {
    fn write(&self, b: &mut CodeBuffer, level: usize) {
        self.write_comments(CommentType::Doc, b, level);
        b.indent(level);
        self.write_access(b);
        self.write_modifiers(b);
        self.var.write(b);
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
    use crate::java::JavaPrimitive;

    #[test]
    fn simple_field() {
        let f: Field = Field::from(("x", JavaPrimitive::Int));
        assert_eq!(f.to_code(), "int x;\n");
    }

    #[test]
    fn private_final_field() {
        let f: Field = Field::from(("name", "String"))
            .with_access(Access::Private)
            .with_modifier(Modifier::Final);
        assert_eq!(f.to_code(), "private final String name;\n");
    }

    #[test]
    fn constant() {
        let f: Field = Field::from(("RED", "Color"))
            .with_comment(" Red.")
            .with_access(Access::Public)
            .with_modifier(Modifier::Static)
            .with_modifier(Modifier::Final)
            .with_initializer("new Color(1)");
        let mut b: CodeBuffer = CodeBuffer::default();
        f.write(&mut b, 1);
        assert_eq!(
            b.peek(),
            "    /**\n     * Red.\n     */\n    public static final Color RED = new Color(1);\n"
        );
    }
}
