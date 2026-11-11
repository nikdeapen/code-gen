use crate::kotlin::types::Members;
use crate::kotlin::{
    Access, CommentType, Function, Generic, KotlinType, Modifier, NestedType, Object, Property,
    SignatureDec, WithAccess, WithAnnotations, WithComments, WithCompanion, WithFunctions,
    WithGenerics, WithModifiers, WithNestedTypes, WithProperties, WithSupertypes,
};
use crate::{CodeBuffer, Statement, WithName};
use std::collections::BTreeSet;

/// An interface declaration.
pub struct Interface {
    comments: Vec<String>,
    annotations: Vec<String>,
    access: Access,
    modifiers: BTreeSet<Modifier>,
    name: String,
    generics: Vec<Generic>,
    supertypes: Vec<KotlinType>,
    properties: Vec<Property>,
    signature_decs: Vec<SignatureDec>,
    functions: Vec<Function>,
    companion: Option<Object>,
    nested_types: Vec<NestedType>,
}

impl<S: Into<String>> From<S> for Interface {
    fn from(name: S) -> Self {
        Self {
            comments: Vec::default(),
            annotations: Vec::default(),
            access: Access::default(),
            modifiers: BTreeSet::default(),
            name: name.into(),
            generics: Vec::default(),
            supertypes: Vec::default(),
            properties: Vec::default(),
            signature_decs: Vec::default(),
            functions: Vec::default(),
            companion: None,
            nested_types: Vec::default(),
        }
    }
}

impl Interface {
    //! Properties

    /// Gets the function signature declarations.
    #[must_use]
    pub fn signature_decs(&self) -> &[SignatureDec] {
        self.signature_decs.as_slice()
    }
}

impl Interface {
    //! Signature Decs

    /// Adds the function signature declaration.
    pub fn add_signature_dec<S>(&mut self, signature_dec: S)
    where
        S: Into<SignatureDec>,
    {
        self.signature_decs.push(signature_dec.into());
    }

    /// Adds the function signature declaration.
    #[must_use]
    pub fn with_signature_dec<S>(mut self, signature_dec: S) -> Self
    where
        S: Into<SignatureDec>,
    {
        self.add_signature_dec(signature_dec);
        self
    }
}

impl WithComments for Interface {
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

impl WithAnnotations for Interface {
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

impl WithAccess for Interface {
    fn access(&self) -> Access {
        self.access
    }

    fn set_access(&mut self, access: Access) {
        self.access = access;
    }
}

impl WithModifiers for Interface {
    fn modifiers(&self) -> &BTreeSet<Modifier> {
        &self.modifiers
    }

    fn add_modifier(&mut self, modifier: Modifier) {
        self.modifiers.insert(modifier);
    }
}

impl WithName for Interface {
    fn name(&self) -> &str {
        self.name.as_str()
    }
}

impl WithGenerics for Interface {
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

impl WithSupertypes for Interface {
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

impl WithProperties for Interface {
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

impl WithFunctions for Interface {
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

impl WithCompanion for Interface {
    fn companion(&self) -> Option<&Object> {
        self.companion.as_ref()
    }

    fn set_companion(&mut self, companion: Object) {
        self.companion = Some(companion);
    }
}

impl WithNestedTypes for Interface {
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

impl Statement for Interface {
    fn write(&self, b: &mut CodeBuffer, level: usize) {
        self.write_comments(CommentType::Doc, b, level);
        self.write_annotations(b, level);
        b.indent(level);
        self.write_access(b);
        self.write_modifiers(b);
        b.write("interface ");
        self.write_name(b);
        self.write_generic_brackets(b);
        self.write_supertypes(b);
        let members: Members = Members {
            properties: &self.properties,
            signature_decs: &self.signature_decs,
            functions: &self.functions,
            companion: self.companion.as_ref(),
            nested_types: &self.nested_types,
        };
        members.write(b, level);
        b.end_line();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::kotlin::{Class, Signature, WithExpression, WithResult};

    #[test]
    fn empty_interface() {
        let i = Interface::from("Shape");
        assert_eq!(i.to_code(), "interface Shape\n");
    }

    #[test]
    fn interface_with_header() {
        let i = Interface::from("Source")
            .with_comment(" A source of values.")
            .with_annotation("Deprecated(\"Use a list.\")")
            .with_access(Access::Internal)
            .with_modifier(Modifier::Sealed)
            .with_generic("out T")
            .with_supertype(KotlinType::from("Iterable").with_generic("T"))
            .with_supertype("AutoCloseable");
        assert_eq!(
            i.to_code(),
            "/**\n * A source of values.\n */\n@Deprecated(\"Use a list.\")\ninternal sealed interface Source<out T> : Iterable<T>, AutoCloseable\n"
        );
    }

    #[test]
    fn interface_with_members() {
        let i = Interface::from("Shape")
            .with_modifier(Modifier::Sealed)
            .with_property(("name", "String"))
            .with_signature_dec(Signature::from("area").with_result("Double"))
            .with_function(
                Function::from(Signature::from("isEmpty").with_result("Boolean"))
                    .with_expression("area() == 0.0"),
            )
            .with_companion(
                Object::default().with_property(
                    Property::from(("SIDES", "Int"))
                        .with_modifier(Modifier::Const)
                        .with_initializer("4"),
                ),
            )
            .with_nested_type(Class::from("Square").with_supertype("Shape"));
        let mut b = CodeBuffer::default();
        i.write(&mut b, 1);
        assert_eq!(
            b.peek(),
            "    sealed interface Shape {\n        val name: String\n\n        fun area(): Double\n\n        fun isEmpty(): Boolean = area() == 0.0\n\n        companion object {\n            const val SIDES: Int = 4\n        }\n\n        class Square : Shape\n    }\n"
        );
    }
}
