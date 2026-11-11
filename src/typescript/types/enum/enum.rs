use crate::typescript::types::members::write_members;
use crate::typescript::{CommentType, EnumCase, Modifier, WithComments, WithModifiers};
use crate::{CodeBuffer, Statement, WithName};
use std::collections::BTreeSet;

/// An enum declaration.
#[derive(Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub struct Enum {
    comments: Vec<String>,
    modifiers: BTreeSet<Modifier>,
    name: String,
    cases: Vec<EnumCase>,
}

impl<S: Into<String>> From<S> for Enum {
    fn from(name: S) -> Self {
        Self {
            comments: Vec::default(),
            modifiers: BTreeSet::default(),
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

impl WithModifiers for Enum {
    fn modifiers(&self) -> &BTreeSet<Modifier> {
        &self.modifiers
    }

    fn add_modifier(&mut self, modifier: Modifier) {
        self.modifiers.insert(modifier);
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
}

impl Statement for Enum {
    fn write(&self, b: &mut CodeBuffer, level: usize) {
        self.write_comments(CommentType::Doc, b, level);
        b.indent(level);
        self.write_modifiers(b);
        b.write("enum ");
        self.write_name(b);
        b.write(" {");
        if self.cases.is_empty() {
            b.push('}');
            b.end_line();
        } else {
            b.end_line();
            write_members(self.cases(), b, level + 1);
            b.line(level, "}");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_enum() {
        let e = Enum::from("Empty");
        assert_eq!(e.to_code(), "enum Empty {}\n");
    }

    #[test]
    fn enum_with_cases() {
        let e = Enum::from("Color")
            .with_case("Red")
            .with_case("Green")
            .with_case("Blue");
        assert_eq!(
            e.to_code(),
            "enum Color {\n    Red,\n    Green,\n    Blue,\n}\n"
        );
    }

    #[test]
    fn exported_enum_with_comments() {
        let e = Enum::from("Color")
            .with_comment(" A color.")
            .with_modifier(Modifier::Export)
            .with_case(
                EnumCase::from("Red")
                    .with_comment(" The color of a rose.")
                    .with_value("1"),
            )
            .with_case(EnumCase::from("Green").with_value("2"));
        let mut b = CodeBuffer::default();
        e.write(&mut b, 1);
        assert_eq!(
            b.peek(),
            "    /** A color. */\n    export enum Color {\n        /** The color of a rose. */\n        Red = 1,\n\n        Green = 2,\n    }\n"
        );
    }
}
