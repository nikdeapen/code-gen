use crate::go::{ConstInit, WithComments, write_members};
use crate::{CodeBuffer, Statement};

/// A `const` block.
///
/// # Note
/// - A block with no constants writes nothing.
#[derive(Default)]
pub struct ConstBlock {
    comments: Vec<String>,
    constants: Vec<ConstInit>,
}

impl WithComments for ConstBlock {
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

impl ConstBlock {
    //! Constants

    /// Gets the constants.
    #[must_use]
    pub fn constants(&self) -> &[ConstInit] {
        self.constants.as_slice()
    }

    /// Adds the constant.
    pub fn add_constant<C>(&mut self, constant: C)
    where
        C: Into<ConstInit>,
    {
        self.constants.push(constant.into());
    }

    /// Adds the constant.
    #[must_use]
    pub fn with_constant<C>(mut self, constant: C) -> Self
    where
        C: Into<ConstInit>,
    {
        self.add_constant(constant);
        self
    }
}

impl Statement for ConstBlock {
    fn write(&self, b: &mut CodeBuffer, level: usize) {
        if !self.constants.is_empty() {
            self.write_comments(b, level);
            b.line(level, "const (");
            write_members(
                self.constants.as_slice(),
                ConstInit::write_spec,
                b,
                level + 1,
            );
            b.line(level, ")");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Literal;

    #[test]
    fn empty_const_block() {
        let c = ConstBlock::default().with_comment(" The cases of Color.");
        assert_eq!(c.to_code(), "");
    }

    #[test]
    fn const_block() {
        let c = ConstBlock::default()
            .with_constant((("ColorRed", "Color"), Literal::from("1")))
            .with_constant((("ColorGreen", "Color"), Literal::from("2")));
        assert_eq!(
            c.to_code(),
            "const (\n    ColorRed Color = 1\n    ColorGreen Color = 2\n)\n"
        );
    }

    #[test]
    fn const_block_with_comments() {
        let c = ConstBlock::default()
            .with_comment(" The cases of Color.")
            .with_constant(
                ConstInit::from((("ColorRed", "Color"), Literal::from("1")))
                    .with_comment(" ColorRed is red."),
            )
            .with_constant(
                ConstInit::from((("ColorGreen", "Color"), Literal::from("2")))
                    .with_comment(" ColorGreen is green."),
            );
        assert_eq!(
            c.to_code(),
            "// The cases of Color.\nconst (\n    // ColorRed is red.\n    ColorRed Color = 1\n\n    // ColorGreen is green.\n    ColorGreen Color = 2\n)\n"
        );
    }
}
