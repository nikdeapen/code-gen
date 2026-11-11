use crate::kotlin::{
    Access, CommentType, KotlinType, Modifier, Var, WithAccess, WithAnnotations, WithComments,
    WithKotlinType, WithModifiers, WithVar,
};
use crate::{CodeBuffer, Statement, WithName};
use std::collections::BTreeSet;

/// A property declaration. (ex: `val x: Int = 1`)
#[derive(Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub struct Property {
    comments: Vec<String>,
    annotations: Vec<String>,
    access: Access,
    modifiers: BTreeSet<Modifier>,
    mutable: bool,
    var: Var,
    initializer: Option<String>,

    /// The expression the getter returns.
    getter: Option<String>,
}

impl<V: Into<Var>> From<V> for Property {
    fn from(var: V) -> Self {
        Self {
            comments: Vec::default(),
            annotations: Vec::default(),
            access: Access::default(),
            modifiers: BTreeSet::default(),
            mutable: false,
            var: var.into(),
            initializer: None,
            getter: None,
        }
    }
}

impl Property {
    //! Properties

    /// Checks if the property is a `var`.
    #[must_use]
    pub fn is_mutable(&self) -> bool {
        self.mutable
    }

    /// Gets the optional initializer.
    #[must_use]
    pub fn initializer(&self) -> Option<&str> {
        self.initializer.as_deref()
    }

    /// Gets the optional expression the getter returns.
    #[must_use]
    pub fn getter(&self) -> Option<&str> {
        self.getter.as_deref()
    }
}

impl Property {
    //! Mutable

    /// Makes the property a `var`.
    pub fn set_mutable(&mut self) {
        self.mutable = true;
    }

    /// Makes the property a `var`.
    #[must_use]
    pub fn with_mutable(mut self) -> Self {
        self.set_mutable();
        self
    }
}

impl Property {
    //! Initializer

    /// Sets the initializer. (ex: the `1` of `val x: Int = 1`)
    pub fn set_initializer<S>(&mut self, initializer: S)
    where
        S: Into<String>,
    {
        self.initializer = Some(initializer.into());
    }

    /// Sets the initializer. (ex: the `1` of `val x: Int = 1`)
    #[must_use]
    pub fn with_initializer<S>(mut self, initializer: S) -> Self
    where
        S: Into<String>,
    {
        self.set_initializer(initializer);
        self
    }
}

impl Property {
    //! Getter

    /// Sets the expression the getter returns. (ex: the `size == 0` of `get() = size == 0`)
    pub fn set_getter<S>(&mut self, getter: S)
    where
        S: Into<String>,
    {
        self.getter = Some(getter.into());
    }

    /// Sets the expression the getter returns. (ex: the `size == 0` of `get() = size == 0`)
    #[must_use]
    pub fn with_getter<S>(mut self, getter: S) -> Self
    where
        S: Into<String>,
    {
        self.set_getter(getter);
        self
    }
}

impl Property {
    //! Write

    /// Checks if the property is written on more than one line.
    pub(crate) fn spans_lines(&self) -> bool {
        !self.comments.is_empty() || !self.annotations.is_empty() || self.getter.is_some()
    }

    /// Writes the property as a parameter of a primary constructor.
    ///
    /// # Note
    /// - The getter is not written, since a parameter can't have one.
    pub(crate) fn write_param(&self, b: &mut CodeBuffer, level: usize) {
        self.write_declaration(b, level);
        b.push(',');
        b.end_line();
    }

    /// Writes the declaration through its initializer, without a line-ending.
    fn write_declaration(&self, b: &mut CodeBuffer, level: usize) {
        self.write_comments(CommentType::Doc, b, level);
        self.write_annotations(b, level);
        b.indent(level);
        self.write_access(b);
        self.write_modifiers(b);
        if self.mutable {
            b.write("var ");
        } else {
            b.write("val ");
        }
        self.write_var(b);
        if let Some(initializer) = &self.initializer {
            b.write(" = ");
            b.write(initializer);
        }
    }
}

impl WithComments for Property {
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

impl WithAnnotations for Property {
    fn annotations(&self) -> &[String] {
        self.annotations.as_slice()
    }

    fn add_annotation<S>(&mut self, annotation: S)
    where
        S: Into<String>,
    {
        self.annotations.push(annotation.into());
    }
}

impl WithAccess for Property {
    fn access(&self) -> Access {
        self.access
    }

    fn set_access(&mut self, access: Access) {
        self.access = access;
    }
}

impl WithModifiers for Property {
    fn modifiers(&self) -> &BTreeSet<Modifier> {
        &self.modifiers
    }

    fn add_modifier(&mut self, modifier: Modifier) {
        self.modifiers.insert(modifier);
    }
}

impl WithName for Property {
    fn name(&self) -> &str {
        self.var.name()
    }
}

impl WithKotlinType for Property {
    fn kotlin_type(&self) -> &KotlinType {
        self.var.kotlin_type()
    }
}

impl WithVar for Property {
    fn var(&self) -> &Var {
        &self.var
    }
}

impl Statement for Property {
    fn write(&self, b: &mut CodeBuffer, level: usize) {
        self.write_declaration(b, level);
        b.end_line();
        if let Some(getter) = &self.getter {
            b.indent(level + 1);
            b.write("get() = ");
            b.write(getter);
            b.end_line();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_property() {
        let p = Property::from(("x", "Int"));
        assert_eq!(p.to_code(), "val x: Int\n");
    }

    #[test]
    fn mutable_property() {
        let p = Property::from(("x", "Int")).with_mutable();
        assert_eq!(p.to_code(), "var x: Int\n");
    }

    #[test]
    fn property_with_initializer() {
        let p = Property::from(("count", "Int"))
            .with_access(Access::Private)
            .with_mutable()
            .with_initializer("0");
        assert_eq!(p.to_code(), "private var count: Int = 0\n");
    }

    #[test]
    fn constant() {
        let p = Property::from(("MAX_DEPTH", "Int"))
            .with_comment(" The most levels a type can nest.")
            .with_access(Access::Internal)
            .with_modifier(Modifier::Const)
            .with_initializer("64");
        assert_eq!(
            p.to_code(),
            "/**\n * The most levels a type can nest.\n */\ninternal const val MAX_DEPTH: Int = 64\n"
        );
    }

    #[test]
    fn property_with_annotation() {
        let p = Property::from(("x", "Int")).with_annotation("JvmField");
        assert_eq!(p.to_code(), "@JvmField\nval x: Int\n");
    }

    #[test]
    fn property_with_getter() {
        let p = Property::from(("x", "Int"))
            .with_modifier(Modifier::Override)
            .with_getter("bytes.getInt(0)");
        let mut b = CodeBuffer::default();
        p.write(&mut b, 1);
        assert_eq!(
            b.peek(),
            "    override val x: Int\n        get() = bytes.getInt(0)\n"
        );
    }

    #[test]
    fn property_as_param() {
        let p = Property::from(("text", KotlinType::from("String").to_nullable()))
            .with_mutable()
            .with_initializer("null")
            .with_getter("field");
        let mut b = CodeBuffer::default();
        p.write_param(&mut b, 1);
        assert_eq!(b.peek(), "    var text: String? = null,\n");
    }

    #[test]
    fn spans_lines() {
        let initialized = Property::from(("x", "Int")).with_initializer("0");
        let commented = Property::from(("x", "Int")).with_comment(" The x.");
        let annotated = Property::from(("x", "Int")).with_annotation("JvmField");
        let computed = Property::from(("x", "Int")).with_getter("0");
        assert!(!initialized.spans_lines());
        assert!(commented.spans_lines());
        assert!(annotated.spans_lines());
        assert!(computed.spans_lines());
    }
}
