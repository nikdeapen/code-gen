use crate::java::types::Members;
use crate::java::{
    Access, CommentType, Constructor, EnumConstant, Field, Function, JavaType, WithAccess,
    WithAnnotations, WithComments, WithConstructors, WithFields, WithFunctions, WithImplements,
};
use crate::{CodeBuffer, IsEmpty, Statement, WithName};

/// An enum declaration.
pub struct Enum {
    comments: Vec<String>,
    annotations: Vec<String>,
    access: Access,
    name: String,
    implements: Vec<JavaType>,
    constants: Vec<EnumConstant>,
    members: Members,
}

impl<S: Into<String>> From<S> for Enum {
    fn from(name: S) -> Self {
        Self {
            comments: Vec::default(),
            annotations: Vec::default(),
            access: Access::default(),
            name: name.into(),
            implements: Vec::default(),
            constants: Vec::default(),
            members: Members::default(),
        }
    }
}

impl WithComments for Enum {
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

impl WithAnnotations for Enum {
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

impl WithAccess for Enum {
    fn access(&self) -> Access {
        self.access
    }

    fn set_access(&mut self, access: Access) {
        self.access = access;
    }
}

impl WithName for Enum {
    fn name(&self) -> &str {
        self.name.as_str()
    }
}

impl WithImplements for Enum {
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

impl Enum {
    //! Constants

    /// Gets the constants.
    #[must_use]
    pub fn constants(&self) -> &[EnumConstant] {
        self.constants.as_slice()
    }

    /// Adds a constant.
    pub fn add_constant<C>(&mut self, constant: C)
    where
        C: Into<EnumConstant>,
    {
        self.constants.push(constant.into());
    }

    /// Adds a constant.
    #[must_use]
    pub fn with_constant<C>(mut self, constant: C) -> Self
    where
        C: Into<EnumConstant>,
    {
        self.add_constant(constant);
        self
    }

    /// Writes the constants.
    ///
    /// # Note
    /// - A blank line splits two constants, unless neither has comments.
    /// - A `;` ends the constants when members follow them.
    fn write_constants(&self, b: &mut CodeBuffer, level: usize) {
        let end: &str = if self.members.is_empty() { "" } else { ";" };
        if let Some((last, rest)) = self.constants.split_last() {
            for (index, constant) in rest.iter().enumerate() {
                constant.write(b, level, ",");
                let next: &EnumConstant = rest.get(index + 1).unwrap_or(last);
                if !(constant.comments().is_empty() && next.comments().is_empty()) {
                    b.end_line();
                }
            }
            last.write(b, level, end);
        } else {
            b.line(level, end);
        }
    }
}

impl WithFields for Enum {
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

impl WithConstructors for Enum {
    fn constructors(&self) -> &[Constructor] {
        self.members.constructors.as_slice()
    }

    fn add_constructor(&mut self, constructor: Constructor) {
        self.members.constructors.push(constructor);
    }
}

impl WithFunctions for Enum {
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

impl Statement for Enum {
    fn write(&self, b: &mut CodeBuffer, level: usize) {
        self.write_comments(CommentType::Doc, b, level);
        self.write_annotations(b, level);
        b.indent(level);
        self.write_access(b);
        b.write("enum ");
        self.write_name(b);
        self.write_implements(b);
        b.write(" {");
        if self.constants.is_empty() && self.members.is_empty() {
            b.push('}');
            b.end_line();
        } else {
            b.end_line();
            self.write_constants(b, level + 1);
            if !self.members.is_empty() {
                b.end_line();
                self.members.write(b, level + 1);
            }
            b.line(level, "}");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::WithStatements;
    use crate::java::{
        JavaPrimitive, Modifier, Signature, WithModifiers, WithResult, WithVarParams,
    };

    #[test]
    fn empty_enum() {
        let e: Enum = Enum::from("Empty");
        assert_eq!(e.to_code(), "enum Empty {}\n");
    }

    #[test]
    fn enum_with_constants() {
        let e: Enum = Enum::from("Color")
            .with_comment(" A color.")
            .with_annotation("Generated(\"proto-packet\")")
            .with_access(Access::Public)
            .with_constant("RED")
            .with_constant("GREEN")
            .with_constant("BLUE");
        assert_eq!(
            e.to_code(),
            "/**\n * A color.\n */\n@Generated(\"proto-packet\")\npublic enum Color {\n    RED,\n    GREEN,\n    BLUE\n}\n"
        );
    }

    #[test]
    fn enum_with_constant_comments() {
        let e: Enum = Enum::from("Color")
            .with_constant(EnumConstant::from("RED").with_comment(" Red."))
            .with_constant("GREEN")
            .with_constant("BLUE");
        assert_eq!(
            e.to_code(),
            "enum Color {\n    /**\n     * Red.\n     */\n    RED,\n\n    GREEN,\n    BLUE\n}\n"
        );
    }

    #[test]
    fn enum_with_members() {
        let e: Enum = Enum::from("Color")
            .with_implements("Tagged")
            .with_constant(EnumConstant::from("RED").with_argument("1"))
            .with_constant(EnumConstant::from("GREEN").with_argument("2"))
            .with_field(
                Field::from(("tagNumber", JavaPrimitive::Int))
                    .with_access(Access::Private)
                    .with_modifier(Modifier::Final),
            )
            .with_constructor(
                Constructor::from("Color")
                    .with_param(("tagNumber", JavaPrimitive::Int))
                    .with_semi("this.tagNumber = tagNumber"),
            )
            .with_function(
                Function::from(Signature::from("tagNumber").with_result(JavaPrimitive::Int))
                    .with_annotation("Override")
                    .with_access(Access::Public)
                    .with_semi("return this.tagNumber"),
            );
        assert_eq!(
            e.to_code(),
            "enum Color implements Tagged {\n    RED(1),\n    GREEN(2);\n\n    private final int tagNumber;\n\n    Color(int tagNumber) {\n        this.tagNumber = tagNumber;\n    }\n\n    @Override\n    public int tagNumber() {\n        return this.tagNumber;\n    }\n}\n"
        );
    }

    #[test]
    fn enum_with_members_and_no_constants() {
        let e: Enum = Enum::from("Empty").with_function(Function::from("reset"));
        let mut b: CodeBuffer = CodeBuffer::default();
        e.write(&mut b, 1);
        assert_eq!(
            b.peek(),
            "    enum Empty {\n        ;\n\n        void reset() {}\n    }\n"
        );
    }
}
