use crate::typescript::types::members::write_members;
use crate::typescript::{
    CommentType, Field, Generic, Modifier, SignatureDec, TypeScriptType, WithComments, WithFields,
    WithGenerics, WithModifiers,
};
use crate::{CodeBuffer, Expression, IsEmpty, Statement, WithName};
use std::collections::BTreeSet;

/// An interface declaration.
#[derive(Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub struct Interface {
    comments: Vec<String>,
    modifiers: BTreeSet<Modifier>,
    name: String,
    generics: Vec<Generic>,
    extends: Vec<TypeScriptType>,
    fields: Vec<Field>,
    signature_decs: Vec<SignatureDec>,
}

impl<S: Into<String>> From<S> for Interface {
    fn from(name: S) -> Self {
        Self {
            comments: Vec::default(),
            modifiers: BTreeSet::default(),
            name: name.into(),
            generics: Vec::default(),
            extends: Vec::default(),
            fields: Vec::default(),
            signature_decs: Vec::default(),
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

impl Interface {
    //! Extends

    /// Gets the interfaces this one extends.
    #[must_use]
    pub fn extends(&self) -> &[TypeScriptType] {
        self.extends.as_slice()
    }

    /// Adds an interface this one `extends`.
    pub fn add_extends<T>(&mut self, extends: T)
    where
        T: Into<TypeScriptType>,
    {
        self.extends.push(extends.into());
    }

    /// Adds an interface this one `extends`.
    #[must_use]
    pub fn with_extends<T>(mut self, extends: T) -> Self
    where
        T: Into<TypeScriptType>,
    {
        self.add_extends(extends);
        self
    }

    /// Writes the `extends` clause. (ex: ` extends Shape, Comparable<Circle>`)
    fn write_extends(&self, b: &mut CodeBuffer) {
        if let Some((first, rest)) = self.extends.split_first() {
            b.write(" extends ");
            first.write(b);
            for extends in rest {
                b.write(", ");
                extends.write(b);
            }
        }
    }
}

impl WithFields for Interface {
    fn fields(&self) -> &[Field] {
        self.fields.as_slice()
    }

    fn add_field<F>(&mut self, field: F)
    where
        F: Into<Field>,
    {
        self.fields.push(field.into());
    }
}

impl Interface {
    //! Signature Decs

    /// Gets the function signature declarations.
    #[must_use]
    pub fn signature_decs(&self) -> &[SignatureDec] {
        self.signature_decs.as_slice()
    }

    /// Adds the function `signature` declaration.
    pub fn add_signature_dec<S>(&mut self, signature: S)
    where
        S: Into<SignatureDec>,
    {
        self.signature_decs.push(signature.into());
    }

    /// Adds the function `signature` declaration.
    #[must_use]
    pub fn with_signature_dec<S>(mut self, signature: S) -> Self
    where
        S: Into<SignatureDec>,
    {
        self.add_signature_dec(signature);
        self
    }
}

impl IsEmpty for Interface {
    fn is_empty(&self) -> bool {
        self.fields.is_empty() && self.signature_decs.is_empty()
    }
}

impl Statement for Interface {
    fn write(&self, b: &mut CodeBuffer, level: usize) {
        self.write_comments(CommentType::Doc, b, level);
        b.indent(level);
        self.write_modifiers(b);
        b.write("interface ");
        self.write_name(b);
        self.write_generic_brackets(b);
        self.write_extends(b);
        b.write(" {");
        if self.is_empty() {
            b.push('}');
            b.end_line();
        } else {
            b.end_line();
            self.write_fields(b, level + 1);
            if !self.fields.is_empty() && !self.signature_decs.is_empty() {
                b.end_line();
            }
            write_members(self.signature_decs(), b, level + 1);
            b.line(level, "}");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::typescript::Signature;

    #[test]
    fn empty_interface() {
        let i = Interface::from("Empty");
        assert_eq!(i.to_code(), "interface Empty {}\n");
    }

    #[test]
    fn exported_interface_with_comment() {
        let i = Interface::from("Shape")
            .with_comment(" A shape.")
            .with_modifier(Modifier::Export);
        assert_eq!(i.to_code(), "/** A shape. */\nexport interface Shape {}\n");
    }

    #[test]
    fn interface_with_generics() {
        let i = Interface::from("Comparable").with_generic("T");
        assert_eq!(i.to_code(), "interface Comparable<T> {}\n");
    }

    #[test]
    fn interface_with_extends() {
        let i = Interface::from("Circle")
            .with_extends("Shape")
            .with_extends(TypeScriptType::from("Comparable").with_generic("Circle"));
        assert_eq!(
            i.to_code(),
            "interface Circle extends Shape, Comparable<Circle> {}\n"
        );
    }

    #[test]
    fn interface_with_fields() {
        let i = Interface::from("Point")
            .with_field(Field::from(("x", "number")).with_modifier(Modifier::Readonly))
            .with_field(Field::from(("y", "number")).with_modifier(Modifier::Readonly));
        assert_eq!(
            i.to_code(),
            "interface Point {\n    readonly x: number;\n    readonly y: number;\n}\n"
        );
    }

    #[test]
    fn interface_with_signature_decs() {
        let i = Interface::from("Shape")
            .with_signature_dec(Signature::from("area").with_result("number"))
            .with_signature_dec(
                SignatureDec::from(Signature::from("close").with_result("void"))
                    .with_comment(" Closes the shape."),
            );
        assert_eq!(
            i.to_code(),
            "interface Shape {\n    area(): number;\n\n    /** Closes the shape. */\n    close(): void;\n}\n"
        );
    }

    #[test]
    fn interface_with_all_members() {
        let i = Interface::from("Shape")
            .with_signature_dec(Signature::from("area").with_result("number"))
            .with_field(("name", "string"));
        let mut b = CodeBuffer::default();
        i.write(&mut b, 1);
        assert_eq!(
            b.peek(),
            "    interface Shape {\n        name: string;\n\n        area(): number;\n    }\n"
        );
    }
}
