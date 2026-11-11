use crate::cpp::{CommentType, CppType, EnumCase, WithComments};
use crate::{CodeBuffer, Expression, Statement, WithName};

/// A scoped enum declaration. (ex: `enum class Color : std::uint16_t { ... };`)
#[derive(Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub struct Enum {
    comments: Vec<String>,
    name: String,
    underlying_type: Option<CppType>,
    cases: Vec<EnumCase>,
}

impl<S: Into<String>> From<S> for Enum {
    fn from(name: S) -> Self {
        Self {
            comments: Vec::default(),
            name: name.into(),
            underlying_type: None,
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

impl Enum {
    //! Underlying Type

    /// Gets the optional underlying type.
    #[must_use]
    pub fn underlying_type(&self) -> Option<&CppType> {
        self.underlying_type.as_ref()
    }

    /// Sets the underlying type.
    pub fn set_underlying_type<T>(&mut self, underlying_type: T)
    where
        T: Into<CppType>,
    {
        self.underlying_type = Some(underlying_type.into());
    }

    /// Sets the underlying type.
    #[must_use]
    pub fn with_underlying_type<T>(mut self, underlying_type: T) -> Self
    where
        T: Into<CppType>,
    {
        self.set_underlying_type(underlying_type);
        self
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

    /// Writes the cases, with a blank line around each case that has comments.
    fn write_cases(&self, b: &mut CodeBuffer, level: usize) {
        let mut is_after_comments: bool = false;
        for (index, case) in self.cases.iter().enumerate() {
            let has_comments: bool = !case.comments().is_empty();
            if index > 0 && (is_after_comments || has_comments) {
                b.end_line();
            }
            case.write(b, level);
            is_after_comments = has_comments;
        }
    }
}

impl Statement for Enum {
    fn write(&self, b: &mut CodeBuffer, level: usize) {
        self.write_comments(CommentType::LineDoc, b, level);
        b.indent(level);
        b.write("enum class ");
        self.write_name(b);
        if let Some(underlying_type) = &self.underlying_type {
            b.write(" : ");
            underlying_type.write(b);
        }
        b.write(" {");
        if self.cases.is_empty() {
            b.write("};");
            b.end_line();
        } else {
            b.end_line();
            self.write_cases(b, level + 1);
            b.line(level, "};");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_enum() {
        let e = Enum::from("Empty");
        assert_eq!(e.underlying_type(), None);
        assert_eq!(e.to_code(), "enum class Empty {};\n");
    }

    #[test]
    fn enum_with_simple_cases() {
        let e = Enum::from("Color")
            .with_case("Red")
            .with_case("Green")
            .with_case("Blue");
        assert_eq!(e.cases().len(), 3);
        assert_eq!(
            e.to_code(),
            "enum class Color {\n    Red,\n    Green,\n    Blue,\n};\n"
        );
    }

    #[test]
    fn enum_with_underlying_type() {
        let e = Enum::from("Color")
            .with_underlying_type("std::uint16_t")
            .with_case(EnumCase::from("Red").with_value("1"));
        assert_eq!(e.underlying_type(), Some(&CppType::from("std::uint16_t")));
        assert_eq!(
            e.to_code(),
            "enum class Color : std::uint16_t {\n    Red = 1,\n};\n"
        );
    }

    #[test]
    fn enum_with_comments() {
        let e = Enum::from("Color")
            .with_comment(" A color.")
            .with_case(EnumCase::from("Red").with_comment(" The color red."))
            .with_case("Green")
            .with_case("Blue")
            .with_case(EnumCase::from("Black").with_comment(" No color."));
        let mut b = CodeBuffer::default();
        e.write(&mut b, 1);
        assert_eq!(
            b.peek(),
            "    /// A color.\n    enum class Color {\n        /// The color red.\n        Red,\n\n        Green,\n        Blue,\n\n        /// No color.\n        Black,\n    };\n"
        );
    }
}
