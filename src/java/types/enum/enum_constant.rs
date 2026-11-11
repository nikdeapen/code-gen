use crate::java::{CommentType, WithComments};
use crate::{CodeBuffer, WithName};

/// A constant of an enum declaration.
#[derive(Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub struct EnumConstant {
    comments: Vec<String>,
    name: String,
    arguments: Vec<String>,
}

impl<S: Into<String>> From<S> for EnumConstant {
    fn from(name: S) -> Self {
        Self {
            comments: Vec::default(),
            name: name.into(),
            arguments: Vec::default(),
        }
    }
}

impl WithComments for EnumConstant {
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

impl WithName for EnumConstant {
    fn name(&self) -> &str {
        self.name.as_str()
    }
}

impl EnumConstant {
    //! Arguments

    /// Gets the arguments of the constructor call.
    #[must_use]
    pub fn arguments(&self) -> &[String] {
        self.arguments.as_slice()
    }

    /// Adds an argument of the constructor call.
    pub fn add_argument<S>(&mut self, argument: S)
    where
        S: Into<String>,
    {
        self.arguments.push(argument.into());
    }

    /// Adds an argument of the constructor call.
    #[must_use]
    pub fn with_argument<S>(mut self, argument: S) -> Self
    where
        S: Into<String>,
    {
        self.add_argument(argument);
        self
    }
}

impl EnumConstant {
    //! Write

    /// Writes the constant, ended by the `end` that splits it from what follows.
    pub(super) fn write(&self, b: &mut CodeBuffer, level: usize, end: &str) {
        self.write_comments(CommentType::Doc, b, level);
        b.indent(level);
        self.write_name(b);
        if let Some((first, rest)) = self.arguments.split_first() {
            b.push('(');
            b.write(first.as_str());
            for argument in rest {
                b.write(", ");
                b.write(argument.as_str());
            }
            b.push(')');
        }
        b.write(end);
        b.end_line();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Writes the `constant` at indent level 1, ended by the `end`.
    fn write(constant: EnumConstant, end: &str) -> String {
        let mut b: CodeBuffer = CodeBuffer::default();
        constant.write(&mut b, 1, end);
        b.into()
    }

    #[test]
    fn simple_constant() {
        assert_eq!(write(EnumConstant::from("RED"), ","), "    RED,\n");
    }

    #[test]
    fn constant_with_argument() {
        let constant: EnumConstant = EnumConstant::from("RED").with_argument("1");
        assert_eq!(write(constant, ";"), "    RED(1);\n");
    }

    #[test]
    fn constant_with_arguments_and_comment() {
        let constant: EnumConstant = EnumConstant::from("RED")
            .with_comment(" Red.")
            .with_argument("1")
            .with_argument("\"red\"");
        assert_eq!(
            write(constant, ""),
            "    /**\n     * Red.\n     */\n    RED(1, \"red\")\n"
        );
    }
}
