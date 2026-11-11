use crate::kotlin::types::Members;
use crate::kotlin::{
    Access, CommentType, Function, KotlinType, Modifier, Property, WithAccess, WithAnnotations,
    WithComments, WithFunctions, WithModifiers, WithProperties, WithSupertypes,
};
use crate::{CodeBuffer, Statement};
use std::collections::BTreeSet;

/// An object declaration.
///
/// # Note
/// - The default object has no name. It is a companion object.
#[derive(Default)]
pub struct Object {
    comments: Vec<String>,
    annotations: Vec<String>,
    access: Access,
    modifiers: BTreeSet<Modifier>,
    name: Option<String>,
    supertypes: Vec<KotlinType>,
    properties: Vec<Property>,
    functions: Vec<Function>,
}

impl<S: Into<String>> From<S> for Object {
    fn from(name: S) -> Self {
        Self {
            name: Some(name.into()),
            ..Self::default()
        }
    }
}

impl Object {
    //! Properties

    /// Gets the optional name.
    #[must_use]
    pub fn name(&self) -> Option<&str> {
        self.name.as_deref()
    }
}

impl Object {
    //! Write

    /// Writes the object as a companion object.
    pub(crate) fn write_companion(&self, b: &mut CodeBuffer, level: usize) {
        self.write_object(b, level, true);
    }

    /// Writes the object, as a companion object if `is_companion` or if it has no name.
    fn write_object(&self, b: &mut CodeBuffer, level: usize, is_companion: bool) {
        self.write_comments(CommentType::Doc, b, level);
        self.write_annotations(b, level);
        b.indent(level);
        self.write_access(b);
        self.write_modifiers(b);
        if is_companion || self.name.is_none() {
            b.write("companion ");
        }
        b.write("object");
        if let Some(name) = &self.name {
            b.space();
            b.write(name);
        }
        self.write_supertypes(b);
        let members: Members = Members {
            properties: &self.properties,
            functions: &self.functions,
            ..Members::default()
        };
        members.write(b, level);
        b.end_line();
    }
}

impl WithComments for Object {
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

impl WithAnnotations for Object {
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

impl WithAccess for Object {
    fn access(&self) -> Access {
        self.access
    }

    fn set_access(&mut self, access: Access) {
        self.access = access;
    }
}

impl WithModifiers for Object {
    fn modifiers(&self) -> &BTreeSet<Modifier> {
        &self.modifiers
    }

    fn add_modifier(&mut self, modifier: Modifier) {
        self.modifiers.insert(modifier);
    }
}

impl WithSupertypes for Object {
    fn supertypes(&self) -> &[KotlinType] {
        self.supertypes.as_slice()
    }

    fn add_supertype<T>(&mut self, supertype: T)
    where
        T: Into<KotlinType>,
    {
        self.supertypes.push(supertype.into());
    }
}

impl WithProperties for Object {
    fn properties(&self) -> &[Property] {
        self.properties.as_slice()
    }

    fn add_property<P>(&mut self, property: P)
    where
        P: Into<Property>,
    {
        self.properties.push(property.into());
    }
}

impl WithFunctions for Object {
    fn functions(&self) -> &[Function] {
        self.functions.as_slice()
    }

    fn add_function<F>(&mut self, function: F)
    where
        F: Into<Function>,
    {
        self.functions.push(function.into());
    }
}

impl Statement for Object {
    fn write(&self, b: &mut CodeBuffer, level: usize) {
        self.write_object(b, level, false);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_object() {
        let o = Object::from("Red");
        assert_eq!(o.to_code(), "object Red\n");
    }

    #[test]
    fn object_with_header() {
        let o = Object::from("Red")
            .with_comment(" The color red.")
            .with_annotation("Serializable")
            .with_access(Access::Internal)
            .with_modifier(Modifier::Data)
            .with_supertype("Color");
        assert_eq!(
            o.to_code(),
            "/**\n * The color red.\n */\n@Serializable\ninternal data object Red : Color\n"
        );
    }

    #[test]
    fn object_with_members() {
        let o = Object::from("Limits")
            .with_property(
                Property::from(("MAX_DEPTH", "Int"))
                    .with_modifier(Modifier::Const)
                    .with_initializer("64"),
            )
            .with_function(Function::from("check"));
        let mut b = CodeBuffer::default();
        o.write(&mut b, 1);
        assert_eq!(
            b.peek(),
            "    object Limits {\n        const val MAX_DEPTH: Int = 64\n\n        fun check() {}\n    }\n"
        );
    }

    #[test]
    fn default_object_is_a_companion() {
        let o = Object::default().with_function(Function::from("create"));
        assert_eq!(o.to_code(), "companion object {\n    fun create() {}\n}\n");
    }

    #[test]
    fn object_as_companion() {
        let mut b = CodeBuffer::default();
        Object::default().write_companion(&mut b, 0);
        Object::from("Factory").write_companion(&mut b, 0);
        assert_eq!(b.peek(), "companion object\ncompanion object Factory\n");
    }
}
