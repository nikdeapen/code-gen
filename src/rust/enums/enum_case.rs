use crate::rust::{CommentType, WithComments};
use crate::{CodeBuffer, Statement, WithName};

/// An enum case.
pub struct EnumCase {
    comments: Vec<String>,
    name: String,
}

impl<S: Into<String>> From<S> for EnumCase {
    fn from(name: S) -> Self {
        Self {
            comments: Vec::default(),
            name: name.into(),
        }
    }
}

impl WithComments for EnumCase {
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

impl WithName for EnumCase {
    fn name(&self) -> &str {
        self.name.as_str()
    }
}

impl Statement for EnumCase {
    fn write(&self, b: &mut CodeBuffer, level: usize) {
        self.write_comments(CommentType::OuterLineDoc, b, level);
        b.indent(level);
        self.write_name(b);
        b.write(",");
        b.end_line();
    }
}
