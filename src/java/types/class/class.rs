use crate::java::types::Members;
use crate::java::{
    Access, CommentType, Constructor, Field, Function, JavaType, Modifier, NestedType, WithAccess,
    WithAnnotations, WithComments, WithConstructors, WithFields, WithFunctions, WithImplements,
    WithModifiers, WithNestedTypes,
};
use crate::{CodeBuffer, IsEmpty, Statement, WithName};
use std::collections::BTreeSet;

/// A class declaration.
pub struct Class {
    comments: Vec<String>,
    annotations: Vec<String>,
    access: Access,
    modifiers: BTreeSet<Modifier>,
    name: String,
    implements: Vec<JavaType>,
    members: Members,
}

impl<S: Into<String>> From<S> for Class {
    fn from(name: S) -> Self {
        Self {
            comments: Vec::default(),
            annotations: Vec::default(),
            access: Access::default(),
            modifiers: BTreeSet::default(),
            name: name.into(),
            implements: Vec::default(),
            members: Members::default(),
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

impl WithImplements for Class {
    fn implements(&self) -> &[JavaType] {
        self.implements.as_slice()
    }

    fn add_implements<T>(&mut self, interface: T)
    where
        T: Into<JavaType>,
    {
        self.implements.push(interface.into());
    }
}

impl WithFields for Class {
    fn fields(&self) -> &[Field] {
        self.members.fields.as_slice()
    }

    fn add_field<F>(&mut self, field: F)
    where
        F: Into<Field>,
    {
        self.members.fields.push(field.into());
    }
}

impl WithConstructors for Class {
    fn constructors(&self) -> &[Constructor] {
        self.members.constructors.as_slice()
    }

    fn add_constructor(&mut self, constructor: Constructor) {
        self.members.constructors.push(constructor);
    }
}

impl WithFunctions for Class {
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

impl WithNestedTypes for Class {
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

impl Statement for Class {
    fn write(&self, b: &mut CodeBuffer, level: usize) {
        self.write_comments(CommentType::Doc, b, level);
        self.write_annotations(b, level);
        b.indent(level);
        self.write_access(b);
        self.write_modifiers(b);
        b.write("class ");
        self.write_name(b);
        self.write_implements(b);
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
    use crate::java::{JavaPrimitive, Record, WithVarParams};

    #[test]
    fn empty_class() {
        let c: Class = Class::from("Empty");
        assert_eq!(c.to_code(), "class Empty {}\n");
    }

    #[test]
    fn class_with_header() {
        let c: Class = Class::from("Point")
            .with_comment(" A point.")
            .with_annotation("Generated(\"proto-packet\")")
            .with_access(Access::Public)
            .with_modifier(Modifier::Final)
            .with_implements("PointView");
        assert_eq!(
            c.to_code(),
            "/**\n * A point.\n */\n@Generated(\"proto-packet\")\npublic final class Point implements PointView {}\n"
        );
    }

    #[test]
    fn class_with_members() {
        let c: Class = Class::from("Point")
            .with_field(Field::from(("x", JavaPrimitive::Int)).with_access(Access::Private))
            .with_constructor(
                Constructor::from("Point")
                    .with_param(("x", JavaPrimitive::Int))
                    .with_semi("this.x = x"),
            )
            .with_function(Function::from("reset").with_semi("this.x = 0"))
            .with_nested_type(Record::from("Pair"));
        assert_eq!(
            c.to_code(),
            "class Point {\n    private int x;\n\n    Point(int x) {\n        this.x = x;\n    }\n\n    void reset() {\n        this.x = 0;\n    }\n\n    record Pair() {}\n}\n"
        );
    }

    #[test]
    fn class_indented() {
        let c: Class = Class::from("Point").with_field(("x", JavaPrimitive::Int));
        let mut b: CodeBuffer = CodeBuffer::default();
        c.write(&mut b, 1);
        assert_eq!(b.peek(), "    class Point {\n        int x;\n    }\n");
    }
}
