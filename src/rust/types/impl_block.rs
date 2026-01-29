use crate::rust::{
    CommentType, ConstInit, Function, RustType, TypeDec, Var, WithComments, WithFunctions,
    WithGenerics, WithRustType, WithTypeDecs,
};
use crate::{CodeBuffer, EmptyLine, Expression, IsEmpty, Statement};

/// An impl block.
pub struct ImplBlock {
    generics: Vec<Var>,
    structure: RustType,
    for_trait: Option<RustType>,
    comments: Vec<String>,
    type_decs: Vec<TypeDec>,
    constants: Vec<ConstInit>,
    functions: Vec<Function>,
}

impl<T: Into<RustType>> From<T> for ImplBlock {
    fn from(base: T) -> Self {
        Self {
            generics: Vec::default(),
            structure: base.into(),
            for_trait: None,
            comments: Vec::default(),
            type_decs: Vec::default(),
            constants: Vec::default(),
            functions: Vec::default(),
        }
    }
}

impl WithGenerics for ImplBlock {
    fn generics(&self) -> &[Var] {
        self.generics.as_slice()
    }

    fn add_generic<V>(&mut self, generic: V)
    where
        V: Into<Var>,
    {
        self.generics.push(generic.into());
    }
}

impl WithRustType for ImplBlock {
    fn rust_type(&self) -> &RustType {
        &self.structure
    }
}

impl ImplBlock {
    //! For Trait

    /// Gets the optional `for` trait.
    pub fn for_trait(&self) -> Option<&RustType> {
        self.for_trait.as_ref()
    }

    /// Sets the `for_trait`.
    pub fn set_for_trait<T>(&mut self, for_trait: T)
    where
        T: Into<RustType>,
    {
        self.for_trait = Some(for_trait.into());
    }

    /// Sets the `for_trait`.
    pub fn with_for_trait<T>(mut self, for_trait: T) -> Self
    where
        T: Into<RustType>,
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

impl ImplBlock {
    //! Constants

    /// Gets the constants.
    pub fn constants(&self) -> &[ConstInit] {
        self.constants.as_slice()
    }

    /// Adds the constant.
    pub fn add_constant(&mut self, constant: ConstInit) {
        self.constants.push(constant);
    }

    /// Adds the constant.
    pub fn with_constant(mut self, constant: ConstInit) -> Self {
        self.add_constant(constant);
        self
    }

    /// Adds the constant.
    pub fn write_constants(&self, b: &mut CodeBuffer, level: usize) {
        for constant in self.constants() {
            EmptyLine::default().write(b, level);
            constant.write(b, level);
        }
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
        self.comments.is_empty()
            && self.type_decs().is_empty()
            && self.constants.is_empty()
            && self.functions.is_empty()
    }
}

impl Statement for ImplBlock {
    fn write(&self, b: &mut CodeBuffer, level: usize) {
        b.indent(level);
        b.write("impl");
        self.write_generic_brackets(b);
        b.space();
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
            self.write_constants(b, level + 1);
            self.write_functions(b, level + 1);
            b.line(level, "}");
        }
    }
}
