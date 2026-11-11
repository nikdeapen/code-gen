use crate::java::types::Members;
use crate::java::{
    Access, CommentType, Function, JavaType, Modifier, NestedType, SignatureDec, WithAccess,
    WithAnnotations, WithComments, WithFunctions, WithModifiers, WithNestedTypes,
};
use crate::{CodeBuffer, Expression, IsEmpty, Statement, WithName};
use std::collections::BTreeSet;

/// An interface declaration.
pub struct Interface {
    comments: Vec<String>,
    annotations: Vec<String>,
    access: Access,
    modifiers: BTreeSet<Modifier>,
    name: String,
    extends: Vec<JavaType>,
    members: Members,
}

impl<S: Into<String>> From<S> for Interface {
    fn from(name: S) -> Self {
        Self {
            comments: Vec::default(),
            annotations: Vec::default(),
            access: Access::default(),
            modifiers: BTreeSet::default(),
            name: name.into(),
            extends: Vec::default(),
            members: Members::default(),
        }
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

impl Interface {
    //! Extends

    /// Gets the interfaces the interface extends.
    #[must_use]
    pub fn extends(&self) -> &[JavaType] {
        self.extends.as_slice()
    }

    /// Adds an interface the interface extends.
    pub fn add_extends<T>(&mut self, interface: T)
    where
        T: Into<JavaType>,
    {
        self.extends.push(interface.into());
    }

    /// Adds an interface the interface extends.
    #[must_use]
    pub fn with_extends<T>(mut self, interface: T) -> Self
    where
        T: Into<JavaType>,
    {
        self.add_extends(interface);
        self
    }

    /// Writes the `extends` clause, after a space.
    fn write_extends(&self, b: &mut CodeBuffer) {
        if let Some((first, rest)) = self.extends.split_first() {
            b.write(" extends ");
            first.write(b);
            for interface in rest {
                b.write(", ");
                interface.write(b);
            }
        }
    }
}

impl Interface {
    //! Signature Decs

    /// Gets the function signature declarations.
    #[must_use]
    pub fn signature_decs(&self) -> &[SignatureDec] {
        self.members.signature_decs.as_slice()
    }

    /// Adds a function signature declaration.
    pub fn add_signature_dec<S>(&mut self, signature_dec: S)
    where
        S: Into<SignatureDec>,
    {
        self.members.signature_decs.push(signature_dec.into());
    }

    /// Adds a function signature declaration.
    #[must_use]
    pub fn with_signature_dec<S>(mut self, signature_dec: S) -> Self
    where
        S: Into<SignatureDec>,
    {
        self.add_signature_dec(signature_dec);
        self
    }
}

impl WithFunctions for Interface {
    fn functions(&self) -> &[Function] {
        self.members.functions.as_slice()
    }

    fn add_function<F>(&mut self, function: F)
    where
        F: Into<Function>,
    {
        self.members.functions.push(function.into());
    }
}

impl WithNestedTypes for Interface {
    fn nested_types(&self) -> &[NestedType] {
        self.members.nested_types.as_slice()
    }

    fn add_nested_type<T>(&mut self, nested_type: T)
    where
        T: Into<NestedType>,
    {
        self.members.nested_types.push(nested_type.into());
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
        self.write_extends(b);
        b.write(" {");
        if self.members.is_empty() {
            b.push('}');
            b.end_line();
        } else {
            b.end_line();
            self.members.write(b, level + 1);
            b.line(level, "}");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::WithStatements;
    use crate::java::{JavaPrimitive, Record, Signature, WithImplements, WithResult};

    #[test]
    fn empty_interface() {
        let i: Interface = Interface::from("Empty");
        assert_eq!(i.to_code(), "interface Empty {}\n");
    }

    #[test]
    fn interface_with_header() {
        let i: Interface = Interface::from("Shape")
            .with_comment(" A shape.")
            .with_annotation("Generated(\"proto-packet\")")
            .with_access(Access::Public)
            .with_modifier(Modifier::Sealed)
            .with_extends(JavaType::from("Comparable").with_generic("Shape"));
        assert_eq!(
            i.to_code(),
            "/**\n * A shape.\n */\n@Generated(\"proto-packet\")\npublic sealed interface Shape extends Comparable<Shape> {}\n"
        );
    }

    #[test]
    fn interface_with_many_extends() {
        let i: Interface = Interface::from("Shape")
            .with_extends("Tagged")
            .with_extends("Encoded");
        assert_eq!(i.to_code(), "interface Shape extends Tagged, Encoded {}\n");
    }

    #[test]
    fn interface_with_members() {
        let i: Interface = Interface::from("Shape")
            .with_signature_dec(Signature::from("tagNumber").with_result(JavaPrimitive::Int))
            .with_function(
                Function::from(Signature::from("isKnown").with_result(JavaPrimitive::Boolean))
                    .with_modifier(Modifier::Default)
                    .with_semi("return this.tagNumber() != 0"),
            )
            .with_nested_type(Record::from("Circle").with_implements("Shape"));
        let mut b: CodeBuffer = CodeBuffer::default();
        i.write(&mut b, 1);
        assert_eq!(
            b.peek(),
            "    interface Shape {\n        int tagNumber();\n\n        default boolean isKnown() {\n            return this.tagNumber() != 0;\n        }\n\n        record Circle() implements Shape {}\n    }\n"
        );
    }
}
