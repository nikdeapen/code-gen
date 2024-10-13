use crate::rust::{
    CommentType, Function, TypeDec, TypeTag, WithComments, WithFunctions, WithTypeDecs, WithTypeTag,
};
use crate::{CodeBuffer, EmptyLine, Expression, IsEmpty, Statement};

/// A struct impl block.
pub struct ImplBlock {
    structure: TypeTag,
    for_trait: Option<TypeTag>,
    comments: Vec<String>,
    type_decs: Vec<TypeDec>,
    functions: Vec<Function>,
}

impl<T: Into<TypeTag>> From<T> for ImplBlock {
    fn from(base: T) -> Self {
        Self {
            structure: base.into(),
            for_trait: None,
            comments: Vec::default(),
            type_decs: Vec::default(),
            functions: Vec::default(),
        }
    }
}

impl WithTypeTag for ImplBlock {
    fn type_tag(&self) -> &TypeTag {
        &self.structure
    }
}

impl ImplBlock {
    //! For Trait

    /// Gets the optional `for` trait.
    pub fn for_trait(&self) -> Option<&TypeTag> {
        self.for_trait.as_ref()
    }

    /// Sets the `for` trait.
    pub fn set_for_trait<T>(&mut self, for_trait: T)
    where
        T: Into<TypeTag>,
    {
        self.for_trait = Some(for_trait.into());
    }

    /// Sets the `for` trait.
    pub fn with_for_trait<T>(mut self, for_trait: T) -> Self
    where
        T: Into<TypeTag>,
    {
        self.set_for_trait(for_trait);
        self
    }
}

impl WithComments for ImplBlock {
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

impl WithTypeDecs for ImplBlock {
    fn type_decs(&self) -> &[TypeDec] {
        self.type_decs.as_slice()
    }

    fn add_type_dec<D>(&mut self, type_dec: D)
    where
        D: Into<TypeDec>,
    {
        self.type_decs.push(type_dec.into());
    }
}

impl WithFunctions for ImplBlock {
    fn functions(&self) -> &[Function] {
        self.functions.as_slice()
    }

    fn add_function<F>(&mut self, function: F)
    where
        F: Into<Function>,
    {
        self.functions.push(function.into());
    }
}

impl IsEmpty for ImplBlock {
    fn is_empty(&self) -> bool {
        self.comments.is_empty() && self.functions.is_empty() && self.type_decs().is_empty()
    }
}

impl Statement for ImplBlock {
    fn write(&self, b: &mut CodeBuffer, level: usize) {
        b.indent(level);
        b.write("impl ");
        if let Some(for_trait) = self.for_trait() {
            for_trait.write(b);
            b.write(" for ");
        }
        self.structure.write(b);
        b.write(" {");
        if self.is_empty() {
            b.write("}");
            b.end_line();
        } else {
            b.end_line();
            self.write_comments(CommentType::InnerLineDoc, b, level + 1);
            EmptyLine::default().write(b, level);
            self.write_type_decs(b, level + 1);
            self.write_functions(b, level + 1);
            b.line(level, "}");
        }
    }
}
