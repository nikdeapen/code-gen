use crate::csharp::{
    Access, CSharpType, CommentType, EnumCase, WithAccess, WithAttributes, WithComments,
};
use crate::{CodeBuffer, Expression, Statement, WithName};

/// An enum declaration.
///
/// # Note
/// - A blank line is between two cases, unless both have no comments.
#[derive(Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub struct Enum {
    comments: Vec<String>,
    attributes: Vec<String>,
    access: Access,
    name: String,
    underlying_type: Option<CSharpType>,
    cases: Vec<EnumCase>,
}

impl<S: Into<String>> From<S> for Enum {
    fn from(name: S) -> Self {
        Self {
            comments: Vec::default(),
            attributes: Vec::default(),
            access: Access::default(),
            name: name.into(),
            underlying_type: None,
            cases: Vec::default(),
        }
    }
}

impl Enum {
    //! Underlying Type

    /// Gets the optional underlying type.
    #[must_use]
    pub fn underlying_type(&self) -> Option<&CSharpType> {
        self.underlying_type.as_ref()
    }

    /// Sets the underlying type.
    pub fn set_underlying_type<T>(&mut self, underlying_type: T)
    where
        T: Into<CSharpType>,
    {
        self.underlying_type = Some(underlying_type.into());
    }

    /// Sets the underlying type.
    #[must_use]
    pub fn with_underlying_type<T>(mut self, underlying_type: T) -> Self
    where
        T: Into<CSharpType>,
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

    /// Writes the cases.
    fn write_cases(&self, b: &mut CodeBuffer, level: usize) {
        let mut last_is_line: Option<bool> = None;
        for case in &self.cases {
            let this_is_line: bool = case.comments().is_empty();
            if last_is_line.is_some_and(|last_is_line| !(last_is_line && this_is_line)) {
                b.end_line();
            }
            case.write(b, level);
            last_is_line = Some(this_is_line);
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

impl WithAttributes for Enum {
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

impl WithAccess for Enum {
    fn access(&self) -> Access {
        self.access
    }

    fn set_access(&mut self, access: Access) {
        self.access = access;
    }
}

impl WithName for Enum {
    fn name(&self) -> &str {
        self.name.as_str()
    }
}

impl Statement for Enum {
    fn write(&self, b: &mut CodeBuffer, level: usize) {
        self.write_comments(CommentType::Doc, b, level);
        self.write_attributes(b, level);
        b.indent(level);
        self.write_access(b);
        b.write("enum ");
        self.write_name(b);
        if let Some(underlying_type) = &self.underlying_type {
            b.write(" : ");
            underlying_type.write(b);
        }
        b.end_line();
        b.line(level, "{");
        self.write_cases(b, level + 1);
        b.line(level, "}");
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::csharp::CSharpPrimitive;

    #[test]
    fn empty_enum() {
        let e: Enum = Enum::from("Empty");
        assert_eq!(e.to_code(), "enum Empty\n{\n}\n");
    }

    #[test]
    fn enum_with_simple_cases() {
        let e: Enum = Enum::from("Color")
            .with_access(Access::Public)
            .with_case("Red")
            .with_case("Green");
        let mut b: CodeBuffer = CodeBuffer::default();
        e.write(&mut b, 1);
        assert_eq!(
            b.peek(),
            "    public enum Color\n    {\n        Red,\n        Green,\n    }\n"
        );
    }

    #[test]
    fn enum_with_header() {
        let e: Enum = Enum::from("Color")
            .with_comment(" A color.")
            .with_attribute("Flags")
            .with_underlying_type(CSharpPrimitive::UnsignedInt16)
            .with_case(EnumCase::from("Red").with_value("1"));
        assert_eq!(
            e.to_code(),
            "/// <summary>\n/// A color.\n/// </summary>\n[Flags]\nenum Color : ushort\n{\n    Red = 1,\n}\n"
        );
    }

    #[test]
    fn enum_with_case_comments() {
        let e: Enum = Enum::from("Color")
            .with_case("Red")
            .with_case(EnumCase::from("Green").with_comment(" The color green."))
            .with_case("Blue")
            .with_case("Black");
        assert_eq!(
            e.to_code(),
            "\
enum Color
{
    Red,

    /// <summary>
    /// The color green.
    /// </summary>
    Green,

    Blue,
    Black,
}
"
        );
    }
}
