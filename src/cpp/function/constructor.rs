use crate::cpp::{
    Access, CommentType, Modifier, Var, WithAccess, WithComments, WithModifiers, WithVarParams,
};
use crate::{CodeBuffer, Statement, WithName, WithStatements};
use std::collections::BTreeSet;

/// A constructor definition.
///
/// # Note
/// - A defaulted constructor is written with no member initializers & no body.
pub struct Constructor {
    comments: Vec<String>,
    access: Access,
    modifiers: BTreeSet<Modifier>,
    name: String,
    params: Vec<Var>,
    initializers: Vec<(String, String)>,
    statements: Vec<Box<dyn Statement>>,
}

impl<S: Into<String>> From<S> for Constructor {
    fn from(name: S) -> Self {
        Self {
            comments: Vec::default(),
            access: Access::default(),
            modifiers: BTreeSet::default(),
            name: name.into(),
            params: Vec::default(),
            initializers: Vec::default(),
            statements: Vec::default(),
        }
    }
}

impl WithComments for Constructor {
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

impl WithAccess for Constructor {
    fn access(&self) -> Access {
        self.access
    }

    fn set_access(&mut self, access: Access) {
        self.access = access;
    }
}

impl WithModifiers for Constructor {
    fn modifiers(&self) -> &BTreeSet<Modifier> {
        &self.modifiers
    }

    fn add_modifier(&mut self, modifier: Modifier) {
        self.modifiers.insert(modifier);
    }
}

impl WithName for Constructor {
    fn name(&self) -> &str {
        self.name.as_str()
    }
}

impl WithVarParams for Constructor {
    fn params(&self) -> &[Var] {
        self.params.as_slice()
    }

    fn add_param<V>(&mut self, param: V)
    where
        V: Into<Var>,
    {
        self.params.push(param.into());
    }
}

impl Constructor {
    //! Initializers

    /// Gets the member initializers, each a member & its value.
    #[must_use]
    pub fn initializers(&self) -> &[(String, String)] {
        self.initializers.as_slice()
    }

    /// Adds the member initializer. (ex: `x_(x)` for the `member` `x_` & the `value` `x`)
    pub fn add_initializer<S0, S1>(&mut self, member: S0, value: S1)
    where
        S0: Into<String>,
        S1: Into<String>,
    {
        self.initializers.push((member.into(), value.into()));
    }

    /// Adds the member initializer. (ex: `x_(x)` for the `member` `x_` & the `value` `x`)
    #[must_use]
    pub fn with_initializer<S0, S1>(mut self, member: S0, value: S1) -> Self
    where
        S0: Into<String>,
        S1: Into<String>,
    {
        self.add_initializer(member, value);
        self
    }

    /// Writes the member initializer list. (ex: ` : x_(x), y_(y)`)
    fn write_initializers(&self, b: &mut CodeBuffer) {
        for (index, (member, value)) in self.initializers.iter().enumerate() {
            b.write(if index == 0 { " : " } else { ", " });
            b.write(member.as_str());
            b.push('(');
            b.write(value.as_str());
            b.push(')');
        }
    }
}

impl WithStatements for Constructor {
    fn statements(&self) -> &[Box<dyn Statement>] {
        self.statements.as_slice()
    }

    fn add_boxed_statement(&mut self, statement: Box<dyn Statement>) {
        self.statements.push(statement);
    }
}

impl Statement for Constructor {
    fn write(&self, b: &mut CodeBuffer, level: usize) {
        self.write_comments(CommentType::LineDoc, b, level);
        b.indent(level);
        self.write_modifiers(b);
        self.write_name(b);
        b.push('(');
        self.write_params(b);
        b.push(')');
        self.write_trailing_modifiers(b);
        if self.modifiers.contains(&Modifier::Default) {
            b.push(';');
        } else {
            self.write_initializers(b);
            b.space();
            self.write_curly_statement_block(b, level);
        }
        b.end_line();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_constructor() {
        let c = Constructor::from("Point");
        assert_eq!(c.to_code(), "Point() {}\n");
    }

    #[test]
    fn constructor_with_initializers() {
        let c = Constructor::from("Point")
            .with_access(Access::Public)
            .with_param(("x", "std::uint32_t"))
            .with_param(("y", "std::uint32_t"))
            .with_initializer("x_", "x")
            .with_initializer("y_", "y");
        assert_eq!(c.access(), Access::Public);
        assert_eq!(c.initializers().len(), 2);
        assert_eq!(
            c.to_code(),
            "Point(std::uint32_t x, std::uint32_t y) : x_(x), y_(y) {}\n"
        );
    }

    #[test]
    fn constructor_with_body() {
        let c = Constructor::from("Buffer")
            .with_comment(" Creates the Buffer.")
            .with_modifier(Modifier::Explicit)
            .with_param(("size", "std::size_t"))
            .with_semi("bytes_.resize(size)");
        let mut b = CodeBuffer::default();
        c.write(&mut b, 1);
        assert_eq!(
            b.peek(),
            "    /// Creates the Buffer.\n    explicit Buffer(std::size_t size) {\n        bytes_.resize(size);\n    }\n"
        );
    }

    #[test]
    fn constructor_with_modifiers() {
        let c = Constructor::from("Point")
            .with_modifier(Modifier::Noexcept)
            .with_modifier(Modifier::Explicit)
            .with_modifier(Modifier::Constexpr)
            .with_param(("x", "std::uint32_t"))
            .with_initializer("x_", "x");
        assert_eq!(
            c.to_code(),
            "constexpr explicit Point(std::uint32_t x) noexcept : x_(x) {}\n"
        );
    }

    #[test]
    fn defaulted_constructor() {
        let c = Constructor::from("Point")
            .with_modifier(Modifier::Default)
            .with_initializer("x_", "0")
            .with_semi("reset()");
        assert_eq!(c.to_code(), "Point() = default;\n");
    }
}
