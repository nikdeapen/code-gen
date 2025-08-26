use crate::rust::CommentType::OuterLineDoc;
use crate::rust::{
    Access, Function, SignatureDec, WithAccess, WithAttributes, WithComments, WithFunctions,
    WithTraitFunctions,
};
use crate::{CodeBuffer, IsEmpty, Statement, WithName};

/// A trait declaration.
pub struct Trait {
    comments: Vec<String>,
    attributes: Vec<String>,
    access: Access,
    name: String,
    trait_functions: Vec<SignatureDec>,
    functions: Vec<Function>,
}

impl<S: Into<String>> From<S> for Trait {
    fn from(name: S) -> Self {
        Self {
            comments: Vec::default(),
            attributes: Vec::default(),
            access: Access::default(),
            name: name.into(),
            trait_functions: Vec::default(),
            functions: Vec::default(),
        }
    }
}

impl WithComments for Trait {
    fn comments(&self) -> &[String] {
        self.comments.as_slice()
    }

    fn add_comment<S>(&mut self, comment: S)
    where
        S: Into<String>,
    {
        self.comments.push(comment.into())
    }
}

impl WithAttributes for Trait {
    fn attributes(&self) -> &[String] {
        self.attributes.as_slice()
    }

    fn add_attribute<S>(&mut self, attribute: S)
    where
        S: Into<String>,
    {
        self.attributes.push(attribute.into())
    }
}

impl WithAccess for Trait {
    fn access(&self) -> &Access {
        &self.access
    }

    fn set_access<A>(&mut self, access: A)
    where
        A: Into<Access>,
    {
        self.access = access.into();
    }
}

impl WithName for Trait {
    fn name(&self) -> &str {
        self.name.as_str()
    }
}

impl WithTraitFunctions for Trait {
    fn signature_decs(&self) -> &[SignatureDec] {
        self.trait_functions.as_slice()
    }

    fn add_signature_dec<F>(&mut self, function: F)
    where
        F: Into<SignatureDec>,
    {
        self.trait_functions.push(function.into());
    }
}

impl WithFunctions for Trait {
    fn functions(&self) -> &[Function] {
        self.functions.as_slice()
    }

    fn add_function<F>(&mut self, function: F)
    where
        F: Into<Function>,
    {
        self.functions.push(function.into())
    }
}

impl IsEmpty for Trait {
    fn is_empty(&self) -> bool {
        self.functions.is_empty() && self.trait_functions.is_empty()
    }
}

impl Statement for Trait {
    fn write(&self, b: &mut CodeBuffer, level: usize) {
        self.write_comments(OuterLineDoc, b, level);
        self.write_attributes(b, level);
        b.indent(level);
        self.write_access(b);
        b.write("trait ");
        self.write_name(b);
        b.write(" {");
        if self.is_empty() {
            b.write("}");
            b.end_line();
        } else {
            b.end_line();
            for function in self.signature_decs() {
                b.end_line();
                function.write(b, level + 1);
            }
            for function in self.functions() {
                b.end_line();
                function.write(b, level + 1);
            }
            b.line(level, "}");
        }
    }
}
