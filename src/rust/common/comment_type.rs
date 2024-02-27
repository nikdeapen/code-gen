use std::fmt::{Display, Formatter};

use crate::{CodeBuffer, ToStaticStr};

/// A comment type.
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub enum CommentType {
    Line,
    InnerLineDoc,
    OuterLineDoc,
}

impl CommentType {
    //! Write

    /// Writes the comment line.
    pub fn write_line(&self, line: &str, b: &mut CodeBuffer, level: usize) {
        b.indent(level);
        b.write(self.to_static_str());
        b.space();
        b.write(line);
        b.end_line();
    }
}

impl ToStaticStr for CommentType {
    fn to_static_str(&self) -> &'static str {
        match self {
            Self::Line => "//",
            Self::InnerLineDoc => "//!",
            Self::OuterLineDoc => "///",
        }
    }
}

impl Display for CommentType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_static_str())
    }
}

#[cfg(test)]
mod tests {
    use crate::rust::CommentType;

    #[test]
    fn display() {
        assert_eq!(CommentType::Line.to_string(), "//");
        assert_eq!(CommentType::InnerLineDoc.to_string(), "//!");
        assert_eq!(CommentType::OuterLineDoc.to_string(), "///");
    }
}
