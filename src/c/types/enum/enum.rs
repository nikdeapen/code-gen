use crate::c::{CommentType, EnumCase, WithComments, WithTypedefName};
use crate::{CodeBuffer, Statement, WithName};

/// An enum declaration.
///
/// # Note
/// - The name is the enum's tag. (ex: the `color` of `enum color`)
/// - An enum with no cases is not valid C.
#[derive(Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub struct Enum {
    comments: Vec<String>,
    name: String,
    typedef_name: Option<String>,
    cases: Vec<EnumCase>,
}

impl<S: Into<String>> From<S> for Enum {
    fn from(name: S) -> Self {
        Self {
            comments: Vec::default(),
            name: name.into(),
            typedef_name: None,
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

impl WithName for Enum {
    fn name(&self) -> &str {
        self.name.as_str()
    }
}

impl WithTypedefName for Enum {
    fn typedef_name(&self) -> Option<&str> {
        self.typedef_name.as_deref()
    }

    fn set_typedef_name<S>(&mut self, typedef_name: S)
    where
        S: Into<String>,
    {
        self.typedef_name = Some(typedef_name.into());
    }
}

impl Enum {
    //! Cases

    /// Gets the cases.
    #[must_use]
    pub fn cases(&self) -> &[EnumCase] {
        self.cases.as_slice()
    }

    /// Adds a case.
    pub fn add_case<C>(&mut self, case: C)
    where
        C: Into<EnumCase>,
    {
        self.cases.push(case.into());
    }

    /// Adds a case.
    #[must_use]
    pub fn with_case<C>(mut self, case: C) -> Self
    where
        C: Into<EnumCase>,
    {
        self.add_case(case);
        self
    }

    /// Writes the cases.
    ///
    /// # Note
    /// - A blank line is written between two cases if either has comments.
    fn write_cases(&self, b: &mut CodeBuffer, level: usize) {
        if let Some((first, rest)) = self.cases.split_first() {
            first.write(b, level);
            for (previous, case) in self.cases.iter().zip(rest) {
                if !previous.comments().is_empty() || !case.comments().is_empty() {
                    b.end_line();
                }
                case.write(b, level);
            }
        }
    }
}

impl Statement for Enum {
    fn write(&self, b: &mut CodeBuffer, level: usize) {
        self.write_comments(CommentType::Doc, b, level);
        b.indent(level);
        self.write_typedef_keyword(b);
        b.write("enum ");
        self.write_name(b);
        b.write(" {");
        if !self.cases.is_empty() {
            b.end_line();
            self.write_cases(b, level + 1);
            b.indent(level);
        }
        b.push('}');
        self.write_typedef_name(b);
        b.push(';');
        b.end_line();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_enum() {
        let e: Enum = Enum::from("empty");
        assert_eq!(e.to_code(), "enum empty {};\n");
    }

    #[test]
    fn enum_with_cases() {
        let e: Enum = Enum::from("color")
            .with_case("COLOR_RED")
            .with_case("COLOR_GREEN");
        assert_eq!(e.cases().len(), 2);
        assert_eq!(
            e.to_code(),
            "enum color {\n    COLOR_RED,\n    COLOR_GREEN,\n};\n"
        );
    }

    #[test]
    fn enum_with_typedef_name() {
        let e: Enum = Enum::from("color")
            .with_typedef_name("color_t")
            .with_case(EnumCase::from("COLOR_RED").with_value("1"));
        assert_eq!(e.typedef_name(), Some("color_t"));
        assert_eq!(
            e.to_code(),
            "typedef enum color {\n    COLOR_RED = 1,\n} color_t;\n"
        );
    }

    #[test]
    fn enum_with_comments() {
        let e: Enum = Enum::from("color")
            .with_comment(" A color.")
            .with_case(EnumCase::from("COLOR_RED").with_comment(" The color red."))
            .with_case(EnumCase::from("COLOR_GREEN").with_comment(" The color green."));
        assert_eq!(
            e.to_code(),
            "/** A color. */\nenum color {\n    /** The color red. */\n    COLOR_RED,\n\n    /** The color green. */\n    COLOR_GREEN,\n};\n"
        );
    }

    #[test]
    fn enum_with_some_comments() {
        let e: Enum = Enum::from("color")
            .with_case("COLOR_RED")
            .with_case("COLOR_GREEN")
            .with_case(EnumCase::from("COLOR_BLUE").with_comment(" The color blue."))
            .with_case("COLOR_BLACK");
        assert_eq!(
            e.to_code(),
            "enum color {\n    COLOR_RED,\n    COLOR_GREEN,\n\n    /** The color blue. */\n    COLOR_BLUE,\n\n    COLOR_BLACK,\n};\n"
        );
    }

    #[test]
    fn enum_indented() {
        let e: Enum = Enum::from("color")
            .with_comment(" A color.")
            .with_typedef_name("color_t")
            .with_case("COLOR_RED");
        let mut b: CodeBuffer = CodeBuffer::default();
        e.write(&mut b, 1);
        assert_eq!(
            b.peek(),
            "    /** A color. */\n    typedef enum color {\n        COLOR_RED,\n    } color_t;\n"
        );
    }
}
