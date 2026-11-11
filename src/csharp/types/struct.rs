use crate::csharp::types::Members;
use crate::csharp::{
    Access, CSharpType, CommentType, Constructor, Field, Function, Modifier, Property, WithAccess,
    WithAttributes, WithBaseTypes, WithComments, WithConstructors, WithFields, WithFunctions,
    WithModifiers, WithProperties,
};
use crate::{CodeBuffer, Statement, WithName};
use std::collections::BTreeSet;

/// A struct declaration.
///
/// # Note
/// - The members are written as the members of a [Class](crate::csharp::Class) are.
pub struct Struct {
    comments: Vec<String>,
    attributes: Vec<String>,
    access: Access,
    modifiers: BTreeSet<Modifier>,
    name: String,
    base_types: Vec<CSharpType>,
    members: Members,
}

impl<S: Into<String>> From<S> for Struct {
    fn from(name: S) -> Self {
        Self {
            comments: Vec::default(),
            attributes: Vec::default(),
            access: Access::default(),
            modifiers: BTreeSet::default(),
            name: name.into(),
            base_types: Vec::default(),
            members: Members::default(),
        }
    }
}

impl WithComments for Struct {
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

impl WithAttributes for Struct {
    fn attributes(&self) -> &[String] {
        self.attributes.as_slice()
    }

    fn add_attribute<S>(&mut self, attribute: S)
    where
        S: Into<String>,
    {
        self.attributes.push(attribute.into());
    }
}

impl WithAccess for Struct {
    fn access(&self) -> Access {
        self.access
    }

    fn set_access(&mut self, access: Access) {
        self.access = access;
    }
}

impl WithModifiers for Struct {
    fn modifiers(&self) -> &BTreeSet<Modifier> {
        &self.modifiers
    }

    fn add_modifier(&mut self, modifier: Modifier) {
        self.modifiers.insert(modifier);
    }
}

impl WithName for Struct {
    fn name(&self) -> &str {
        self.name.as_str()
    }
}

impl WithBaseTypes for Struct {
    fn base_types(&self) -> &[CSharpType] {
        self.base_types.as_slice()
    }

    fn add_base_type<T>(&mut self, base_type: T)
    where
        T: Into<CSharpType>,
    {
        self.base_types.push(base_type.into());
    }
}

impl WithFields for Struct {
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

impl WithConstructors for Struct {
    fn constructors(&self) -> &[Constructor] {
        self.members.constructors.as_slice()
    }

    fn add_constructor<C>(&mut self, constructor: C)
    where
        C: Into<Constructor>,
    {
        self.members.constructors.push(constructor.into());
    }
}

impl WithProperties for Struct {
    fn properties(&self) -> &[Property] {
        self.members.properties.as_slice()
    }

    fn add_property<P>(&mut self, property: P)
    where
        P: Into<Property>,
    {
        self.members.properties.push(property.into());
    }
}

impl WithFunctions for Struct {
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

impl Statement for Struct {
    fn write(&self, b: &mut CodeBuffer, level: usize) {
        self.write_comments(CommentType::Doc, b, level);
        self.write_attributes(b, level);
        b.indent(level);
        self.write_access(b);
        self.write_modifiers(b);
        b.write("struct ");
        self.write_name(b);
        self.write_base_types(b);
        b.end_line();
        self.members.write(b, level);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::WithStatements;
    use crate::csharp::{Signature, WithVarParams};

    #[test]
    fn empty_struct() {
        let s: Struct = Struct::from("Empty");
        assert_eq!(s.to_code(), "struct Empty\n{\n}\n");
    }

    #[test]
    fn struct_with_header() {
        let s: Struct = Struct::from("Meters")
            .with_comment(" A length in meters.")
            .with_attribute("Serializable")
            .with_access(Access::Public)
            .with_modifier(Modifier::Readonly)
            .with_base_type(CSharpType::from("IEquatable").with_generic("Meters"))
            .with_base_type("IFormattable");
        assert_eq!(
            s.to_code(),
            "/// <summary>\n/// A length in meters.\n/// </summary>\n[Serializable]\npublic readonly struct Meters : IEquatable<Meters>, IFormattable\n{\n}\n"
        );
    }

    #[test]
    fn struct_with_members() {
        let s: Struct = Struct::from("Meters")
            .with_access(Access::Public)
            .with_modifier(Modifier::Readonly)
            .with_field(
                Field::from(("_value", "double"))
                    .with_access(Access::Private)
                    .with_modifier(Modifier::Readonly),
            )
            .with_constructor(
                Constructor::from("Meters")
                    .with_access(Access::Public)
                    .with_param(("value", "double"))
                    .with_semi("_value = value"),
            )
            .with_property(
                Property::from(("Value", "double"))
                    .with_access(Access::Public)
                    .with_expression("_value"),
            )
            .with_function(
                Function::from(Signature::from("ToFeet").with_result("double"))
                    .with_access(Access::Public)
                    .with_semi("return _value * 3.28084"),
            );
        assert_eq!(
            s.to_code(),
            "\
public readonly struct Meters
{
    private readonly double _value;

    public Meters(double value)
    {
        _value = value;
    }

    public double Value => _value;

    public double ToFeet()
    {
        return _value * 3.28084;
    }
}
"
        );
    }
}
