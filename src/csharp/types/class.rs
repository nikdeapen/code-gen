use crate::csharp::types::Members;
use crate::csharp::{
    Access, CSharpType, CommentType, Constructor, Field, Function, Modifier, Property,
    SignatureDec, WithAccess, WithAttributes, WithBaseTypes, WithComments, WithConstructors,
    WithFields, WithFunctions, WithModifiers, WithNestedTypes, WithProperties, WithSignatureDecs,
};
use crate::{CodeBuffer, Statement, WithName};
use std::collections::BTreeSet;

/// A class declaration.
///
/// # Note
/// - The members are written by kind. The fields are first, then the constructors, the
///   properties, the signature declarations, the functions & the nested types.
/// - A blank line is between two members, unless they are of one kind & each is one line.
pub struct Class {
    comments: Vec<String>,
    attributes: Vec<String>,
    access: Access,
    modifiers: BTreeSet<Modifier>,
    name: String,
    base_types: Vec<CSharpType>,
    members: Members,
}

impl<S: Into<String>> From<S> for Class {
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

impl WithAttributes for Class {
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

impl WithBaseTypes for Class {
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

    fn add_constructor<C>(&mut self, constructor: C)
    where
        C: Into<Constructor>,
    {
        self.members.constructors.push(constructor.into());
    }
}

impl WithProperties for Class {
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

impl WithSignatureDecs for Class {
    fn signature_decs(&self) -> &[SignatureDec] {
        self.members.signature_decs.as_slice()
    }

    fn add_signature_dec<S>(&mut self, signature_dec: S)
    where
        S: Into<SignatureDec>,
    {
        self.members.signature_decs.push(signature_dec.into());
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
    fn nested_types(&self) -> &[Box<dyn Statement>] {
        self.members.nested_types.as_slice()
    }

    fn add_nested_type<S>(&mut self, nested_type: S)
    where
        S: 'static + Statement,
    {
        self.members.nested_types.push(Box::new(nested_type));
    }
}

impl Statement for Class {
    fn write(&self, b: &mut CodeBuffer, level: usize) {
        self.write_comments(CommentType::Doc, b, level);
        self.write_attributes(b, level);
        b.indent(level);
        self.write_access(b);
        self.write_modifiers(b);
        b.write("class ");
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
    fn empty_class() {
        let c: Class = Class::from("Empty");
        assert_eq!(c.to_code(), "class Empty\n{\n}\n");
    }

    #[test]
    fn class_with_the_indent_and_line_ending_of_the_buffer() {
        let c: Class = Class::from("Task")
            .with_comment(" A task.")
            .with_field(("_count", "int"))
            .with_function(Function::from("Run").with_semi("_count++"));
        let mut b: CodeBuffer = CodeBuffer::new("\t", "\r\n", 64);
        c.write(&mut b, 1);
        assert_eq!(
            b.peek(),
            "\t/// <summary>\r\n\t/// A task.\r\n\t/// </summary>\r\n\tclass Task\r\n\t{\r\n\t\tint _count;\r\n\r\n\t\tvoid Run()\r\n\t\t{\r\n\t\t\t_count++;\r\n\t\t}\r\n\t}\r\n"
        );
    }

    #[test]
    fn class_with_header() {
        let c: Class = Class::from("Cache")
            .with_comment(" A cache of values.")
            .with_attribute("Serializable")
            .with_access(Access::Public)
            .with_modifier(Modifier::Partial)
            .with_modifier(Modifier::Sealed)
            .with_base_type("Store")
            .with_base_type("IDisposable");
        assert_eq!(
            c.to_code(),
            "/// <summary>\n/// A cache of values.\n/// </summary>\n[Serializable]\npublic sealed partial class Cache : Store, IDisposable\n{\n}\n"
        );
    }

    #[test]
    fn class_with_members() {
        let c: Class = Class::from("Shape")
            .with_access(Access::Public)
            .with_modifier(Modifier::Abstract)
            .with_field(Field::from(("_name", "string")).with_access(Access::Private))
            .with_constructor(
                Constructor::from("Shape")
                    .with_access(Access::Protected)
                    .with_param(("name", "string"))
                    .with_semi("_name = name"),
            )
            .with_property(
                Property::from(("Name", "string"))
                    .with_access(Access::Public)
                    .with_expression("_name"),
            )
            .with_signature_dec(
                SignatureDec::from(Signature::from("Area").with_result("double"))
                    .with_access(Access::Public)
                    .with_modifier(Modifier::Abstract),
            )
            .with_function(
                Function::from(Signature::from("ToString").with_result("string"))
                    .with_access(Access::Public)
                    .with_modifier(Modifier::Override)
                    .with_semi("return _name"),
            )
            .with_nested_type(Class::from("Builder").with_access(Access::Public));
        assert_eq!(
            c.to_code(),
            "\
public abstract class Shape
{
    private string _name;

    protected Shape(string name)
    {
        _name = name;
    }

    public string Name => _name;

    public abstract double Area();

    public override string ToString()
    {
        return _name;
    }

    public class Builder
    {
    }
}
"
        );
    }
}
