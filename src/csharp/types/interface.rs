use crate::csharp::types::Members;
use crate::csharp::{
    Access, CSharpType, CommentType, Property, SignatureDec, WithAccess, WithAttributes,
    WithBaseTypes, WithComments, WithProperties, WithSignatureDecs,
};
use crate::{CodeBuffer, Statement, WithName};

/// An interface declaration.
///
/// # Note
/// - The members are written as the members of a [Class](crate::csharp::Class) are.
pub struct Interface {
    comments: Vec<String>,
    attributes: Vec<String>,
    access: Access,
    name: String,
    base_types: Vec<CSharpType>,
    members: Members,
}

impl<S: Into<String>> From<S> for Interface {
    fn from(name: S) -> Self {
        Self {
            comments: Vec::default(),
            attributes: Vec::default(),
            access: Access::default(),
            name: name.into(),
            base_types: Vec::default(),
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

impl WithAttributes for Interface {
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

impl WithAccess for Interface {
    fn access(&self) -> Access {
        self.access
    }

    fn set_access(&mut self, access: Access) {
        self.access = access;
    }
}

impl WithName for Interface {
    fn name(&self) -> &str {
        self.name.as_str()
    }
}

impl WithBaseTypes for Interface {
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

impl WithProperties for Interface {
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

impl WithSignatureDecs for Interface {
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

impl Statement for Interface {
    fn write(&self, b: &mut CodeBuffer, level: usize) {
        self.write_comments(CommentType::Doc, b, level);
        self.write_attributes(b, level);
        b.indent(level);
        self.write_access(b);
        b.write("interface ");
        self.write_name(b);
        self.write_base_types(b);
        b.end_line();
        self.members.write(b, level);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::csharp::Signature;

    #[test]
    fn empty_interface() {
        let i: Interface = Interface::from("IEmpty");
        assert_eq!(i.to_code(), "interface IEmpty\n{\n}\n");
    }

    #[test]
    fn interface_with_header() {
        let i: Interface = Interface::from("IStore")
            .with_comment(" A store of values.")
            .with_attribute("Obsolete")
            .with_access(Access::Public)
            .with_base_type("IDisposable");
        let mut b: CodeBuffer = CodeBuffer::default();
        i.write(&mut b, 1);
        assert_eq!(
            b.peek(),
            "    /// <summary>\n    /// A store of values.\n    /// </summary>\n    [Obsolete]\n    public interface IStore : IDisposable\n    {\n    }\n"
        );
    }

    #[test]
    fn interface_with_members() {
        let i: Interface = Interface::from("IPointView")
            .with_access(Access::Public)
            .with_property(("X", "int"))
            .with_property(("Y", "int"))
            .with_signature_dec(Signature::from("Encode").with_result("byte[]"));
        assert_eq!(
            i.to_code(),
            "\
public interface IPointView
{
    int X { get; }
    int Y { get; }

    byte[] Encode();
}
"
        );
    }
}
