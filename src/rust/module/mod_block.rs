use crate::rust::{Access, CommentType, WithAccess, WithAttributes, WithComments};
use crate::{CodeBuffer, Statement, WithName, WithStatements};

/// A `mod` block, which is a module with its code inline. (ex: `mod name { ... }`)
pub struct ModBlock {
    comments: Vec<String>,
    attributes: Vec<String>,
    access: Access,
    name: String,
    statements: Vec<Box<dyn Statement>>,
}

impl<S: Into<String>> From<S> for ModBlock {
    fn from(name: S) -> Self {
        Self {
            comments: Vec::default(),
            attributes: Vec::default(),
            access: Access::default(),
            name: name.into(),
            statements: Vec::default(),
        }
    }
}

impl WithComments for ModBlock {
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

impl WithAttributes for ModBlock {
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

impl WithAccess for ModBlock {
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

impl WithName for ModBlock {
    fn name(&self) -> &str {
        self.name.as_str()
    }
}

impl WithStatements for ModBlock {
    fn statements(&self) -> &[Box<dyn Statement>] {
        self.statements.as_slice()
    }

    fn add_boxed_statement(&mut self, statement: Box<dyn Statement>) {
        self.statements.push(statement);
    }
}

impl Statement for ModBlock {
    fn write(&self, b: &mut CodeBuffer, level: usize) {
        self.write_comments(CommentType::OuterLineDoc, b, level);
        self.write_attributes(b, level);
        b.indent(level);
        self.write_access(b);
        b.write("mod ");
        self.write_name(b);
        b.space();
        self.write_curly_statement_block(b, level);
        b.end_line();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rust::{Struct, StructField, WithDerives, WithStructFields};

    #[test]
    fn empty_mod_block() {
        let m: ModBlock = ModBlock::from("empty");
        assert_eq!(m.to_code(), "mod empty {}\n");
    }

    #[test]
    fn mod_block_with_a_struct() {
        let tag_number: StructField = StructField::from(("tag_number", "std::num::NonZeroU16"))
            .with_access(Access::PublicInSuper);
        let m: ModBlock = ModBlock::from("unrecognized")
            .with_attribute("allow(clippy::module_inception)")
            .with_statement(
                Struct::from("UnrecognizedTagNumber")
                    .with_comment(" The tag number of a case an enum doesn't have.")
                    .with_attribute("must_use")
                    .with_derive("Copy")
                    .with_derive("Clone")
                    .with_access(Access::Public)
                    .with_field(tag_number),
            );
        assert_eq!(
            m.to_code(),
            "#[allow(clippy::module_inception)]\nmod unrecognized {\n    /// The tag number of a case an enum doesn't have.\n    #[must_use]\n    #[derive(Copy, Clone)]\n    pub struct UnrecognizedTagNumber {\n        pub(super) tag_number: std::num::NonZeroU16,\n    }\n}\n"
        );
    }

    #[test]
    fn mod_block_with_comment_and_access() {
        let m: ModBlock = ModBlock::from("tests")
            .with_comment(" The tests.")
            .with_access(Access::PublicInCrate)
            .with_semi("use super::*");
        assert_eq!(
            m.to_code(),
            "/// The tests.\npub(crate) mod tests {\n    use super::*;\n}\n"
        );
    }

    #[test]
    fn mod_block_indented() {
        let m: ModBlock = ModBlock::from("inner").with_statement(ModBlock::from("innermost"));
        let mut b: CodeBuffer = CodeBuffer::default();
        m.write(&mut b, 1);
        assert_eq!(
            b.peek(),
            "    mod inner {\n        mod innermost {}\n    }\n"
        );
    }
}
