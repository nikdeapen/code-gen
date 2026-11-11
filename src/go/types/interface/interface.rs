use crate::go::{GoType, SignatureDec, WithComments, WithEmbedded, write_members};
use crate::{CodeBuffer, Statement, WithName};

/// An interface declaration.
#[derive(Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub struct Interface {
    comments: Vec<String>,
    name: String,
    embedded: Vec<GoType>,
    signature_decs: Vec<SignatureDec>,
}

impl<S: Into<String>> From<S> for Interface {
    fn from(name: S) -> Self {
        Self {
            comments: Vec::default(),
            name: name.into(),
            embedded: Vec::default(),
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

impl WithName for Interface {
    fn name(&self) -> &str {
        self.name.as_str()
    }
}

impl WithEmbedded for Interface {
    fn embedded(&self) -> &[GoType] {
        self.embedded.as_slice()
    }

    fn add_embedded<T>(&mut self, embedded: T)
    where
        T: Into<GoType>,
    {
        self.embedded.push(embedded.into());
    }
}

impl Interface {
    //! Signature Decs

    /// Gets the function signature declarations.
    #[must_use]
    pub fn signature_decs(&self) -> &[SignatureDec] {
        self.signature_decs.as_slice()
    }

    /// Adds the function signature declaration.
    pub fn add_signature_dec<S>(&mut self, signature: S)
    where
        S: Into<SignatureDec>,
    {
        self.signature_decs.push(signature.into());
    }

    /// Adds the function signature declaration.
    #[must_use]
    pub fn with_signature_dec<S>(mut self, signature: S) -> Self
    where
        S: Into<SignatureDec>,
    {
        self.add_signature_dec(signature);
        self
    }
}

impl Statement for Interface {
    fn write(&self, b: &mut CodeBuffer, level: usize) {
        self.write_comments(b, level);
        b.indent(level);
        b.write("type ");
        self.write_name(b);
        b.write(" interface");
        if self.embedded.is_empty() && self.signature_decs.is_empty() {
            b.write("{}");
            b.end_line();
        } else {
            b.write(" {");
            b.end_line();
            self.write_embedded(b, level + 1);
            if !self.embedded.is_empty() && !self.signature_decs.is_empty() {
                b.end_line();
            }
            write_members(
                self.signature_decs.as_slice(),
                SignatureDec::write,
                b,
                level + 1,
            );
            b.line(level, "}");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::go::{Signature, WithVarParams};

    #[test]
    fn empty_interface() {
        let i = Interface::from("Empty");
        assert_eq!(i.to_code(), "type Empty interface{}\n");
    }

    #[test]
    fn interface_with_signature_decs() {
        let i = Interface::from("PointView")
            .with_signature_dec(Signature::from("X").with_result("uint32"))
            .with_signature_dec(Signature::from("Y").with_result("uint32"));
        assert_eq!(
            i.to_code(),
            "type PointView interface {\n    X() uint32\n    Y() uint32\n}\n"
        );
    }

    #[test]
    fn interface_with_signature_dec_comments() {
        let i = Interface::from("PointView")
            .with_signature_dec(
                SignatureDec::from(Signature::from("X").with_result("uint32"))
                    .with_comment(" X gets the field: x."),
            )
            .with_signature_dec(
                SignatureDec::from(Signature::from("Y").with_result("uint32"))
                    .with_comment(" Y gets the field: y."),
            );
        assert_eq!(
            i.to_code(),
            "type PointView interface {\n    // X gets the field: x.\n    X() uint32\n\n    // Y gets the field: y.\n    Y() uint32\n}\n"
        );
    }

    #[test]
    fn interface_with_embedded_type() {
        let i = Interface::from("ReadWriter")
            .with_embedded("io.Reader")
            .with_embedded("io.Writer");
        assert_eq!(
            i.to_code(),
            "type ReadWriter interface {\n    io.Reader\n    io.Writer\n}\n"
        );
    }

    #[test]
    fn interface_with_embedded_type_and_signature_decs() {
        let i = Interface::from("PointViewMut")
            .with_embedded("PointView")
            .with_signature_dec(Signature::from("SetX").with_param(("x", "uint32")));
        assert_eq!(
            i.to_code(),
            "type PointViewMut interface {\n    PointView\n\n    SetX(x uint32)\n}\n"
        );
    }

    #[test]
    fn interface_with_comment() {
        let i = Interface::from("Empty").with_comment(" Empty is an interface with no methods.");
        assert_eq!(
            i.to_code(),
            "// Empty is an interface with no methods.\ntype Empty interface{}\n"
        );
    }
}
