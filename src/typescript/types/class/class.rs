use crate::typescript::{
    CommentType, Constructor, Field, Function, Generic, Modifier, TypeScriptType, WithComments,
    WithFields, WithGenerics, WithModifiers,
};
use crate::{CodeBuffer, Expression, IsEmpty, Statement, WithName};
use std::collections::BTreeSet;

/// A class declaration.
pub struct Class {
    comments: Vec<String>,
    modifiers: BTreeSet<Modifier>,
    name: String,
    generics: Vec<Generic>,
    extends: Option<TypeScriptType>,
    implements: Vec<TypeScriptType>,
    fields: Vec<Field>,
    constructor: Option<Constructor>,
    functions: Vec<Function>,
}

impl<S: Into<String>> From<S> for Class {
    fn from(name: S) -> Self {
        Self {
            comments: Vec::default(),
            modifiers: BTreeSet::default(),
            name: name.into(),
            generics: Vec::default(),
            extends: None,
            implements: Vec::default(),
            fields: Vec::default(),
            constructor: None,
            functions: Vec::default(),
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

impl Class {
    //! Extends

    /// Gets the optional class this one extends.
    #[must_use]
    pub fn extends(&self) -> Option<&TypeScriptType> {
        self.extends.as_ref()
    }

    /// Sets the class this one `extends`.
    pub fn set_extends<T>(&mut self, extends: T)
    where
        T: Into<TypeScriptType>,
    {
        self.extends = Some(extends.into());
    }

    /// Sets the class this one `extends`.
    #[must_use]
    pub fn with_extends<T>(mut self, extends: T) -> Self
    where
        T: Into<TypeScriptType>,
    {
        self.set_extends(extends);
        self
    }

    /// Writes the optional `extends` clause.
    fn write_extends(&self, b: &mut CodeBuffer) {
        if let Some(extends) = &self.extends {
            b.write(" extends ");
            extends.write(b);
        }
    }
}

impl Class {
    //! Implements

    /// Gets the interfaces the class implements.
    #[must_use]
    pub fn implements(&self) -> &[TypeScriptType] {
        self.implements.as_slice()
    }

    /// Adds an interface the class `implements`.
    pub fn add_implements<T>(&mut self, implements: T)
    where
        T: Into<TypeScriptType>,
    {
        self.implements.push(implements.into());
    }

    /// Adds an interface the class `implements`.
    #[must_use]
    pub fn with_implements<T>(mut self, implements: T) -> Self
    where
        T: Into<TypeScriptType>,
    {
        self.add_implements(implements);
        self
    }

    /// Writes the `implements` clause. (ex: ` implements Shape, Comparable<Circle>`)
    fn write_implements(&self, b: &mut CodeBuffer) {
        if let Some((first, rest)) = self.implements.split_first() {
            b.write(" implements ");
            first.write(b);
            for implements in rest {
                b.write(", ");
                implements.write(b);
            }
        }
    }
}

impl WithFields for Class {
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

impl Class {
    //! Constructor

    /// Gets the optional constructor.
    #[must_use]
    pub fn constructor(&self) -> Option<&Constructor> {
        self.constructor.as_ref()
    }

    /// Sets the `constructor`.
    pub fn set_constructor(&mut self, constructor: Constructor) {
        self.constructor = Some(constructor);
    }

    /// Sets the `constructor`.
    #[must_use]
    pub fn with_constructor(mut self, constructor: Constructor) -> Self {
        self.set_constructor(constructor);
        self
    }
}

impl Class {
    //! Functions

    /// Gets the functions.
    #[must_use]
    pub fn functions(&self) -> &[Function] {
        self.functions.as_slice()
    }

    /// Adds the `function`.
    pub fn add_function<F>(&mut self, function: F)
    where
        F: Into<Function>,
    {
        self.functions.push(function.into());
    }

    /// Adds the `function`.
    #[must_use]
    pub fn with_function<F>(mut self, function: F) -> Self
    where
        F: Into<Function>,
    {
        self.add_function(function);
        self
    }
}

impl IsEmpty for Class {
    fn is_empty(&self) -> bool {
        self.fields.is_empty() && self.constructor.is_none() && self.functions.is_empty()
    }
}

impl Class {
    //! Members

    /// Writes the fields, the constructor & the functions, with an empty line between them.
    fn write_members(&self, b: &mut CodeBuffer, level: usize) {
        self.write_fields(b, level);
        let mut is_first: bool = self.fields.is_empty();
        if let Some(constructor) = &self.constructor {
            if !is_first {
                b.end_line();
            }
            constructor.write(b, level);
            is_first = false;
        }
        for function in &self.functions {
            if !is_first {
                b.end_line();
            }
            function.write_method(b, level);
            is_first = false;
        }
    }
}

impl Statement for Class {
    fn write(&self, b: &mut CodeBuffer, level: usize) {
        self.write_comments(CommentType::Doc, b, level);
        b.indent(level);
        self.write_modifiers(b);
        b.write("class ");
        self.write_name(b);
        self.write_generic_brackets(b);
        self.write_extends(b);
        self.write_implements(b);
        b.write(" {");
        if self.is_empty() {
            b.push('}');
            b.end_line();
        } else {
            b.end_line();
            self.write_members(b, level + 1);
            b.line(level, "}");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::WithStatements;
    use crate::typescript::{Signature, WithVarParams};

    #[test]
    fn empty_class() {
        let c = Class::from("Empty");
        assert_eq!(c.to_code(), "class Empty {}\n");
    }

    #[test]
    fn exported_class_with_comment() {
        let c = Class::from("Point")
            .with_comment(" A point.")
            .with_modifier(Modifier::Export);
        assert_eq!(c.to_code(), "/** A point. */\nexport class Point {}\n");
    }

    #[test]
    fn class_with_generics() {
        let c = Class::from("Box").with_generic("T");
        assert_eq!(c.to_code(), "class Box<T> {}\n");
    }

    #[test]
    fn class_with_extends() {
        let c = Class::from("Circle").with_extends("Shape");
        assert_eq!(c.to_code(), "class Circle extends Shape {}\n");
    }

    #[test]
    fn class_with_implements() {
        let c = Class::from("Circle")
            .with_extends("Shape")
            .with_implements("CircleView")
            .with_implements(TypeScriptType::from("Comparable").with_generic("Circle"));
        assert_eq!(
            c.to_code(),
            "class Circle extends Shape implements CircleView, Comparable<Circle> {}\n"
        );
    }

    #[test]
    fn class_with_fields() {
        let c = Class::from("Point")
            .with_field(("x", "number"))
            .with_field(("y", "number"));
        assert_eq!(
            c.to_code(),
            "class Point {\n    x: number;\n    y: number;\n}\n"
        );
    }

    #[test]
    fn class_with_constructor() {
        let c = Class::from("Point").with_constructor(Constructor::default());
        assert_eq!(c.to_code(), "class Point {\n    constructor() {}\n}\n");
    }

    #[test]
    fn class_with_functions() {
        let c = Class::from("Point")
            .with_function("foo")
            .with_function("bar");
        assert_eq!(
            c.to_code(),
            "class Point {\n    foo() {}\n\n    bar() {}\n}\n"
        );
    }

    #[test]
    fn class_with_all_members() {
        let c = Class::from("Point")
            .with_function(
                Function::from(Signature::from("length").with_result("number"))
                    .with_semi("return Math.abs(this.x)"),
            )
            .with_constructor(
                Constructor::default()
                    .with_param(("x", "number"))
                    .with_semi("this.x = x"),
            )
            .with_field(("x", "number"));
        let mut b = CodeBuffer::default();
        c.write(&mut b, 1);
        assert_eq!(
            b.peek(),
            "    class Point {\n        x: number;\n\n        constructor(x: number) {\n            this.x = x;\n        }\n\n        length(): number {\n            return Math.abs(this.x);\n        }\n    }\n"
        );
    }
}
