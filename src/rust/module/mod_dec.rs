use crate::rust::{Access, CommentType, WithAccess, WithAttributes, WithComments};
use crate::{CodeBuffer, Statement, WithName};

/// A `mod` declaration, which is a module with its code in another file. (ex: `mod name;`)
#[derive(Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub struct ModDec {
    comments: Vec<String>,
    attributes: Vec<String>,
    access: Access,
    name: String,
}

impl<S: Into<String>> From<S> for ModDec {
    fn from(name: S) -> Self {
        Self {
            comments: Vec::default(),
            attributes: Vec::default(),
            access: Access::default(),
            name: name.into(),
        }
    }
}

impl WithComments for ModDec {
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

impl WithAttributes for ModDec {
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

impl WithAccess for ModDec {
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

impl WithName for ModDec {
    fn name(&self) -> &str {
        self.name.as_str()
    }
}

impl Statement for ModDec {
    fn write(&self, b: &mut CodeBuffer, level: usize) {
        self.write_comments(CommentType::OuterLineDoc, b, level);
        self.write_attributes(b, level);
        b.indent(level);
        self.write_access(b);
        b.write("mod ");
        self.write_name(b);
        b.push(';');
        b.end_line();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mod_dec() {
        let m: ModDec = ModDec::from("lexer");
        assert_eq!(m.to_code(), "mod lexer;\n");
    }

    #[test]
    fn mod_dec_with_access() {
        let test_cases: &[(Access, &str)] = &[
            (Access::Private, "mod matchers;\n"),
            (Access::Public, "pub mod matchers;\n"),
            (Access::PublicInCrate, "pub(crate) mod matchers;\n"),
            (Access::PublicInSuper, "pub(super) mod matchers;\n"),
        ];

        for (access, expected) in test_cases {
            let m: ModDec = ModDec::from("matchers").with_access(access.clone());
            assert_eq!(m.to_code(), *expected, "access: {:?}", access);
        }
    }

    #[test]
    fn mod_dec_with_comment_and_attributes() {
        let m: ModDec = ModDec::from("Shapes")
            .with_comment(" The shapes.")
            .with_attribute("allow(clippy::module_inception)")
            .with_attribute("allow(non_snake_case)")
            .with_access(Access::Public);
        assert_eq!(
            m.to_code(),
            "/// The shapes.\n#[allow(clippy::module_inception)]\n#[allow(non_snake_case)]\npub mod Shapes;\n"
        );
    }

    #[test]
    fn mod_dec_indented() {
        let m: ModDec = ModDec::from("tests").with_attribute("cfg(test)");
        let mut b: CodeBuffer = CodeBuffer::default();
        m.write(&mut b, 1);
        assert_eq!(b.peek(), "    #[cfg(test)]\n    mod tests;\n");
    }
}
