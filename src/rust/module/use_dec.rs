use crate::rust::{Access, WithAccess, WithAttributes};
use crate::{CodeBuffer, Statement};

/// A `use` declaration. (ex: `use std::fmt::Display;` or `pub use lexer::*;`)
#[derive(Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub struct UseDec {
    attributes: Vec<String>,
    access: Access,
    path: String,
}

impl<S: Into<String>> From<S> for UseDec {
    fn from(path: S) -> Self {
        Self {
            attributes: Vec::default(),
            access: Access::default(),
            path: path.into(),
        }
    }
}

impl WithAttributes for UseDec {
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

impl WithAccess for UseDec {
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

impl UseDec {
    //! Path

    /// Gets the path, which is written as given. (ex: `std::fmt::{Debug, Display}`)
    #[must_use]
    pub fn path(&self) -> &str {
        self.path.as_str()
    }
}

impl Statement for UseDec {
    fn write(&self, b: &mut CodeBuffer, level: usize) {
        self.write_attributes(b, level);
        b.indent(level);
        self.write_access(b);
        b.write("use ");
        b.write(self.path.as_str());
        b.push(';');
        b.end_line();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn use_dec() {
        let u: UseDec = UseDec::from("std::fmt::{Debug, Display}");
        assert_eq!(u.path(), "std::fmt::{Debug, Display}");
        assert_eq!(u.to_code(), "use std::fmt::{Debug, Display};\n");
    }

    #[test]
    fn use_dec_with_access() {
        let test_cases: &[(Access, &str)] = &[
            (Access::Private, "use lexer::*;\n"),
            (Access::Public, "pub use lexer::*;\n"),
            (Access::PublicInCrate, "pub(crate) use lexer::*;\n"),
        ];

        for (access, expected) in test_cases {
            let u: UseDec = UseDec::from("lexer::*").with_access(access.clone());
            assert_eq!(u.to_code(), *expected, "access: {:?}", access);
        }
    }

    #[test]
    fn use_dec_with_attribute() {
        let u: UseDec = UseDec::from(String::from("rust::*"))
            .with_attribute("cfg(feature = \"rust\")")
            .with_access(Access::Public);
        assert_eq!(
            u.to_code(),
            "#[cfg(feature = \"rust\")]\npub use rust::*;\n"
        );
    }

    #[test]
    fn use_dec_indented() {
        let u: UseDec = UseDec::from("super::*");
        let mut b: CodeBuffer = CodeBuffer::default();
        u.write(&mut b, 1);
        assert_eq!(b.peek(), "    use super::*;\n");
    }
}
