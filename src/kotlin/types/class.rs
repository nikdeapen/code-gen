use crate::kotlin::types::Members;
use crate::kotlin::{
    Access, CommentType, Function, Generic, KotlinType, Modifier, NestedType, Object, Property,
    WithAccess, WithAnnotations, WithComments, WithCompanion, WithFunctions, WithGenerics,
    WithModifiers, WithNestedTypes, WithProperties, WithSupertypes,
};
use crate::{CodeBuffer, Statement, WithName};
use std::collections::BTreeSet;

/// A class declaration.
pub struct Class {
    comments: Vec<String>,
    annotations: Vec<String>,
    access: Access,
    modifiers: BTreeSet<Modifier>,
    name: String,
    generics: Vec<Generic>,

    /// The properties of the primary constructor.
    params: Vec<Property>,
    supertypes: Vec<KotlinType>,
    properties: Vec<Property>,
    functions: Vec<Function>,
    companion: Option<Object>,
    nested_types: Vec<NestedType>,
}

impl<S: Into<String>> From<S> for Class {
    fn from(name: S) -> Self {
        Self {
            comments: Vec::default(),
            annotations: Vec::default(),
            access: Access::default(),
            modifiers: BTreeSet::default(),
            name: name.into(),
            generics: Vec::default(),
            params: Vec::default(),
            supertypes: Vec::default(),
            properties: Vec::default(),
            functions: Vec::default(),
            companion: None,
            nested_types: Vec::default(),
        }
    }
}

impl Class {
    //! Properties

    /// Gets the properties of the primary constructor.
    #[must_use]
    pub fn params(&self) -> &[Property] {
        self.params.as_slice()
    }
}

impl Class {
    //! Params

    /// Adds the property to the primary constructor.
    ///
    /// # Note
    /// - Its getter is not written, since a parameter can't have one.
    pub fn add_param<P>(&mut self, param: P)
    where
        P: Into<Property>,
    {
        self.params.push(param.into());
    }

    /// Adds the property to the primary constructor.
    ///
    /// # Note
    /// - Its getter is not written, since a parameter can't have one.
    #[must_use]
    pub fn with_param<P>(mut self, param: P) -> Self
    where
        P: Into<Property>,
    {
        self.add_param(param);
        self
    }

    /// Writes the primary constructor, with each parameter on its own line.
    fn write_params(&self, b: &mut CodeBuffer, level: usize) {
        if !self.params.is_empty() {
            b.push('(');
            b.end_line();
            for param in &self.params {
                param.write_param(b, level + 1);
            }
            b.indent(level);
            b.push(')');
        }
    }
}

impl WithComments for Class {
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

impl WithAnnotations for Class {
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

impl WithAccess for Class {
    fn access(&self) -> Access {
        self.access
    }

    fn set_access(&mut self, access: Access) {
        self.access = access;
    }
}

impl WithModifiers for Class {
    fn modifiers(&self) -> &BTreeSet<Modifier> {
        &self.modifiers
    }

    fn add_modifier(&mut self, modifier: Modifier) {
        self.modifiers.insert(modifier);
    }
}

impl WithName for Class {
    fn name(&self) -> &str {
        self.name.as_str()
    }
}

impl WithGenerics for Class {
    fn generics(&self) -> &[Generic] {
        self.generics.as_slice()
    }

    fn add_generic<G>(&mut self, generic: G)
    where
        G: Into<Generic>,
    {
        self.generics.push(generic.into());
    }
}

impl WithSupertypes for Class {
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

impl WithProperties for Class {
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

impl WithFunctions for Class {
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

impl WithCompanion for Class {
    fn companion(&self) -> Option<&Object> {
        self.companion.as_ref()
    }

    fn set_companion(&mut self, companion: Object) {
        self.companion = Some(companion);
    }
}

impl WithNestedTypes for Class {
    fn nested_types(&self) -> &[NestedType] {
        self.nested_types.as_slice()
    }

    fn add_nested_type<T>(&mut self, nested_type: T)
    where
        T: Into<NestedType>,
    {
        self.nested_types.push(nested_type.into());
    }
}

impl Statement for Class {
    fn write(&self, b: &mut CodeBuffer, level: usize) {
        self.write_comments(CommentType::Doc, b, level);
        self.write_annotations(b, level);
        b.indent(level);
        self.write_access(b);
        self.write_modifiers(b);
        b.write("class ");
        self.write_name(b);
        self.write_generic_brackets(b);
        self.write_params(b, level);
        self.write_supertypes(b);
        let members: Members = Members {
            properties: &self.properties,
            functions: &self.functions,
            companion: self.companion.as_ref(),
            nested_types: &self.nested_types,
            ..Members::default()
        };
        members.write(b, level);
        b.end_line();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::WithStatements;
    use crate::kotlin::Var;

    #[test]
    fn empty_class() {
        let c = Class::from("Empty");
        assert_eq!(c.to_code(), "class Empty\n");
    }

    #[test]
    fn class_with_header() {
        let c = Class::from("Meters")
            .with_comment(" A length in meters.")
            .with_annotation("JvmInline")
            .with_access(Access::Internal)
            .with_modifier(Modifier::Value)
            .with_param(("value", "Double"))
            .with_supertype(KotlinType::from("Comparable").with_generic("Meters"));
        assert_eq!(
            c.to_code(),
            "/**\n * A length in meters.\n */\n@JvmInline\ninternal value class Meters(\n    val value: Double,\n) : Comparable<Meters>\n"
        );
    }

    #[test]
    fn class_with_params() {
        let c = Class::from("Label")
            .with_modifier(Modifier::Data)
            .with_param(
                Property::from(("id", "Long"))
                    .with_access(Access::Private)
                    .with_getter("0"),
            )
            .with_param(
                Property::from(("text", KotlinType::from("String").to_nullable()))
                    .with_mutable()
                    .with_initializer("null"),
            );
        let mut b = CodeBuffer::default();
        c.write(&mut b, 1);
        assert_eq!(
            b.peek(),
            "    data class Label(\n        private val id: Long,\n        var text: String? = null,\n    )\n"
        );
    }

    #[test]
    fn class_with_generics() {
        let c = Class::from("Range")
            .with_generic(Var::from(("T", "Comparable<T>")))
            .with_param(("start", "T"))
            .with_param(("end", "T"));
        assert_eq!(
            c.to_code(),
            "class Range<T : Comparable<T>>(\n    val start: T,\n    val end: T,\n)\n"
        );
    }

    #[test]
    fn class_with_members() {
        let c = Class::from("Counter")
            .with_supertype("Resettable")
            .with_property(
                Property::from(("count", "Int"))
                    .with_mutable()
                    .with_initializer("0"),
            )
            .with_function(
                Function::from("reset")
                    .with_modifier(Modifier::Override)
                    .with_literal("count = 0"),
            )
            .with_companion(
                Object::default().with_property(
                    Property::from(("START", "Int"))
                        .with_modifier(Modifier::Const)
                        .with_initializer("0"),
                ),
            )
            .with_nested_type(Object::from("None").with_supertype("Resettable"));
        assert_eq!(
            c.to_code(),
            "class Counter : Resettable {\n    var count: Int = 0\n\n    override fun reset() {\n        count = 0\n    }\n\n    companion object {\n        const val START: Int = 0\n    }\n\n    object None : Resettable\n}\n"
        );
    }
}
