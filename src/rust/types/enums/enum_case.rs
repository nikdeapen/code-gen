use crate::rust::{CommentType, EnumFields, WithComments};
use crate::{CodeBuffer, Expression, Statement, WithName};

/// An enum case.
pub struct EnumCase {
    comments: Vec<String>,
    name: String,
    fields: EnumFields,
}

impl<S: Into<String>> From<S> for EnumCase {
    fn from(name: S) -> Self {
        Self {
            comments: Vec::default(),
            name: name.into(),
            fields: EnumFields::default(),
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

impl EnumCase {
    //! Fields

    /// Gets the fields.
    pub fn fields(&self) -> &EnumFields {
        &self.fields
    }

    /// Sets the `fields`.
    pub fn set_fields<F>(&mut self, fields: F)
    where
        F: Into<EnumFields>,
    {
        self.fields = fields.into();
    }

    /// Sets the `fields`.
    pub fn with_fields<F>(mut self, fields: F) -> Self
    where
        F: Into<EnumFields>,
    {
        self.set_fields(fields);
        self
    }
}

impl Statement for EnumCase {
    fn write(&self, b: &mut CodeBuffer, level: usize) {
        self.write_comments(CommentType::OuterLineDoc, b, level);
        b.indent(level);
        self.write_name(b);
        self.fields.write(b);
        b.write(",");
        b.end_line();
    }
}
