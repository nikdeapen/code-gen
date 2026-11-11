use crate::typescript::{
    CommentType, Generic, Modifier, TypeScriptType, WithComments, WithGenerics, WithModifiers,
    WithTypeScriptType,
};
use crate::{CodeBuffer, Statement, WithName};
use std::collections::BTreeSet;

/// A type declaration. (ex: `type Shape = Circle | Square;`)
#[derive(Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub struct TypeDec {
    comments: Vec<String>,
    modifiers: BTreeSet<Modifier>,
    name: String,
    generics: Vec<Generic>,
    typescript_type: TypeScriptType,
}

impl<S: Into<String>, T: Into<TypeScriptType>> From<(S, T)> for TypeDec {
    fn from(tuple: (S, T)) -> Self {
        Self {
            comments: Vec::default(),
            modifiers: BTreeSet::default(),
            name: tuple.0.into(),
            generics: Vec::default(),
            typescript_type: tuple.1.into(),
        }
    }
}

impl WithComments for TypeDec {
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

impl WithModifiers for TypeDec {
    fn modifiers(&self) -> &BTreeSet<Modifier> {
        &self.modifiers
    }

    fn add_modifier(&mut self, modifier: Modifier) {
        self.modifiers.insert(modifier);
    }
}

impl WithName for TypeDec {
    fn name(&self) -> &str {
        self.name.as_str()
    }
}

impl WithGenerics for TypeDec {
    fn generics(&self) -> &[Generic] {
        self.generics.as_slice()
    }

    fn add_generic<G>(&mut self, generic: G)
    where
        G: Into<Generic>,
    {
        self.generics.push(generic.into());
    }
}

impl WithTypeScriptType for TypeDec {
    fn typescript_type(&self) -> Option<&TypeScriptType> {
        Some(&self.typescript_type)
    }
}

impl Statement for TypeDec {
    fn write(&self, b: &mut CodeBuffer, level: usize) {
        self.write_comments(CommentType::Doc, b, level);
        b.indent(level);
        self.write_modifiers(b);
        b.write("type ");
        self.write_name(b);
        self.write_generic_brackets(b);
        b.write(" = ");
        self.write_typescript_type(b);
        b.push(';');
        b.end_line();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_type_dec() {
        let t = TypeDec::from(("Id", "bigint"));
        assert_eq!(t.to_code(), "type Id = bigint;\n");
    }

    #[test]
    fn exported_type_dec_with_comment() {
        let t = TypeDec::from((
            "Shape",
            TypeScriptType::from("Circle").with_member("Square"),
        ))
        .with_comment(" A shape.")
        .with_modifier(Modifier::Export);
        assert_eq!(
            t.to_code(),
            "/** A shape. */\nexport type Shape = Circle | Square;\n"
        );
    }

    #[test]
    fn type_dec_with_generics() {
        let t = TypeDec::from(("Maybe", TypeScriptType::from("T").to_optional())).with_generic("T");
        assert_eq!(t.to_code(), "type Maybe<T> = T | undefined;\n");
    }
}
