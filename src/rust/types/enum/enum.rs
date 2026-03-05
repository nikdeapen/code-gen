use crate::rust::CommentType::OuterLineDoc;
use crate::rust::{Access, EnumCase, WithAccess, WithComments, WithDerives};
use crate::{CodeBuffer, EmptyLine, Statement, WithName};

/// An enum declaration.
pub struct Enum {
    comments: Vec<String>,
    derives: Vec<String>,
    access: Access,
    name: String,
    cases: Vec<EnumCase>,
}

impl<S: Into<String>> From<S> for Enum {
    fn from(name: S) -> Self {
        Self {
            comments: Vec::default(),
            derives: Vec::default(),
            access: Access::default(),
            name: name.into(),
            cases: Vec::default(),
        }
    }
}

impl WithComments for Enum {
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

impl WithDerives for Enum {
    fn derives(&self) -> &[String] {
        self.derives.as_slice()
    }

    fn add_derive<S>(&mut self, derive: S)
    where
        S: Into<String>,
    {
        self.derives.push(derive.into());
    }
}

impl WithAccess for Enum {
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

impl WithName for Enum {
    fn name(&self) -> &str {
        self.name.as_str()
    }
}

impl Enum {
    //! Cases

    /// Gets the cases.
    #[must_use]
    pub fn cases(&self) -> &[EnumCase] {
        self.cases.as_slice()
    }

    /// Adds the case.
    pub fn add_case<C>(&mut self, case: C)
    where
        C: Into<EnumCase>,
    {
        self.cases.push(case.into());
    }

    /// Adds the case.
    #[must_use]
    pub fn with_case<C>(mut self, case: C) -> Self
    where
        C: Into<EnumCase>,
    {
        self.add_case(case);
        self
    }

    /// Writes the cases.
    fn write_cases(&self, b: &mut CodeBuffer, level: usize) {
        if let Some((first, rest)) = self.cases().split_first() {
            first.write(b, level);
            for case in rest {
                EmptyLine::default().write(b, level);
                case.write(b, level);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rust::{EnumFields, RustType, WithAccess};

    #[test]
    fn empty_enum() {
        let e = Enum::from("Empty");
        assert_eq!(e.to_code(), "enum Empty {}\n");
    }

    #[test]
    fn enum_with_simple_cases() {
        let e = Enum::from("Color")
            .with_access(Access::Public)
            .with_case(EnumCase::from("Red"))
            .with_case(EnumCase::from("Green"))
            .with_case(EnumCase::from("Blue"));
        assert_eq!(
            e.to_code(),
            "pub enum Color {\n    Red,\n    \n    Green,\n    \n    Blue,\n}\n"
        );
    }

    #[test]
    fn enum_with_derives() {
        let e = Enum::from("Dir")
            .with_derive("Clone")
            .with_case(EnumCase::from("Up"));
        assert_eq!(
            e.to_code(),
            "#[derive(Clone)]\nenum Dir {\n    Up,\n}\n"
        );
    }

    #[test]
    fn enum_with_unnamed_fields() {
        let e = Enum::from("Value").with_case(
            EnumCase::from("Int").with_fields(EnumFields::Unnamed(vec![RustType::from("i64")])),
        );
        assert_eq!(e.to_code(), "enum Value {\n    Int(i64),\n}\n");
    }
}

impl Statement for Enum {
    fn write(&self, b: &mut CodeBuffer, level: usize) {
        self.write_comments(OuterLineDoc, b, level);
        self.write_derives(b, level);
        b.indent(level);
        self.write_access(b);
        b.write("enum ");
        self.write_name(b);
        b.write(" {");
        if self.cases.is_empty() {
            b.push('}');
            b.end_line();
        } else {
            b.end_line();
            self.write_cases(b, level + 1);
            b.line(level, "}");
        }
    }
}
