use crate::java::types::Members;
use crate::java::{
    Access, CommentType, Field, Function, JavaType, Var, WithAccess, WithAnnotations, WithComments,
    WithFields, WithFunctions, WithImplements, WithVarParams,
};
use crate::{CodeBuffer, IsEmpty, Statement, WithName};

/// A record declaration.
///
/// # Note
/// - Its parameters are its components.
/// - Its fields must be `static`.
pub struct Record {
    comments: Vec<String>,
    annotations: Vec<String>,
    access: Access,
    name: String,
    params: Vec<Var>,
    implements: Vec<JavaType>,
    members: Members,
}

impl<S: Into<String>> From<S> for Record {
    fn from(name: S) -> Self {
        Self {
            comments: Vec::default(),
            annotations: Vec::default(),
            access: Access::default(),
            name: name.into(),
            params: Vec::default(),
            implements: Vec::default(),
            members: Members::default(),
        }
    }
}

impl WithComments for Record {
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

impl WithAnnotations for Record {
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

impl WithAccess for Record {
    fn access(&self) -> Access {
        self.access
    }

    fn set_access(&mut self, access: Access) {
        self.access = access;
    }
}

impl WithName for Record {
    fn name(&self) -> &str {
        self.name.as_str()
    }
}

impl WithVarParams for Record {
    fn params(&self) -> &[Var] {
        self.params.as_slice()
    }

    fn add_param<V>(&mut self, param: V)
    where
        V: Into<Var>,
    {
        self.params.push(param.into());
    }
}

impl WithImplements for Record {
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

impl WithFields for Record {
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

impl WithFunctions for Record {
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

impl Statement for Record {
    fn write(&self, b: &mut CodeBuffer, level: usize) {
        self.write_comments(CommentType::Doc, b, level);
        self.write_annotations(b, level);
        b.indent(level);
        self.write_access(b);
        b.write("record ");
        self.write_name(b);
        b.push('(');
        self.write_params(b);
        b.push(')');
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
    use crate::java::{JavaPrimitive, Modifier, Signature, WithModifiers, WithResult};

    #[test]
    fn empty_record() {
        let r: Record = Record::from("Empty");
        assert_eq!(r.to_code(), "record Empty() {}\n");
    }

    #[test]
    fn record_with_header() {
        let r: Record = Record::from("Circle")
            .with_comment(" A circle.")
            .with_annotation("Generated(\"proto-packet\")")
            .with_access(Access::Public)
            .with_param(("x", JavaPrimitive::Double))
            .with_param(("radius", JavaPrimitive::Double))
            .with_implements("Shape");
        assert_eq!(
            r.to_code(),
            "/**\n * A circle.\n */\n@Generated(\"proto-packet\")\npublic record Circle(double x, double radius) implements Shape {}\n"
        );
    }

    #[test]
    fn record_with_members() {
        let r: Record = Record::from("Color")
            .with_param(("tagNumber", JavaPrimitive::Int))
            .with_field(
                Field::from(("RED", "Color"))
                    .with_modifier(Modifier::Static)
                    .with_modifier(Modifier::Final)
                    .with_initializer("new Color(1)"),
            )
            .with_function(
                Function::from(Signature::from("isKnown").with_result(JavaPrimitive::Boolean))
                    .with_semi("return this.tagNumber == 1"),
            );
        let mut b: CodeBuffer = CodeBuffer::default();
        r.write(&mut b, 1);
        assert_eq!(
            b.peek(),
            "    record Color(int tagNumber) {\n        static final Color RED = new Color(1);\n\n        boolean isKnown() {\n            return this.tagNumber == 1;\n        }\n    }\n"
        );
    }
}
