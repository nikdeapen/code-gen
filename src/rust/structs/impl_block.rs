use crate::rust::{CommentType, Function, TypeTag, WithComments, WithFunctions, WithTypeTag};
use crate::{CodeBuffer, Expression, IsEmpty, Statement};

/// A struct impl block.
pub struct ImplBlock {
    base: TypeTag,
    comments: Vec<String>,
    functions: Vec<Function>,
}

impl<T: Into<TypeTag>> From<T> for ImplBlock {
    fn from(base: T) -> Self {
        Self {
            base: base.into(),
            comments: Vec::default(),
            functions: Vec::default(),
        }
    }
}

impl WithTypeTag for ImplBlock {
    fn type_tag(&self) -> &TypeTag {
        &self.base
    }
}

impl WithComments for ImplBlock {
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

impl WithFunctions for ImplBlock {
    fn functions(&self) -> &[Function] {
        self.functions.as_slice()
    }

    fn add_function<F>(&mut self, function: F)
    where
        F: Into<Function>,
    {
        self.functions.push(function.into());
    }
}

impl IsEmpty for ImplBlock {
    fn is_empty(&self) -> bool {
        self.comments.is_empty() && self.functions.is_empty()
    }
}

impl Statement for ImplBlock {
    fn write(&self, b: &mut CodeBuffer, level: usize) {
        b.indent(level);
        b.write("impl ");
        self.base.write(b);
        b.write(" {");
        if self.is_empty() {
            b.write("}");
            b.end_line();
        } else {
            b.end_line();
            self.write_comments(CommentType::InnerLineDoc, b, level + 1);
            b.end_line();
            self.write_functions(b, level + 1);
            b.line(level, "}");
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::rust::PrimitiveType::UnsignedInt32;
    use crate::rust::{ImplBlock, WithComments, WithFunctions};
    use crate::CodeBuffer;

    #[test]
    fn write_empty() {
        let block: ImplBlock = UnsignedInt32.into();
        let result: String = CodeBuffer::display_statement(&block);
        let expected: &str = "impl u32 {}\n";
        assert_eq!(result, expected);
    }

    #[test]
    fn write_comments() {
        let block: ImplBlock = UnsignedInt32.into();
        let block: ImplBlock = block.with_comment("one").with_comment("two");
        let result: String = CodeBuffer::display_statement(&block);
        let expected: &str = "impl u32 {\n\t//! one\n\t//! two\n\n}\n";
        assert_eq!(result, expected);
    }

    #[test]
    fn write_functions() {
        let block: ImplBlock = UnsignedInt32.into();

        let block: ImplBlock = block.with_function("one");
        let result: String = CodeBuffer::display_statement(&block);
        let expected: &str = "impl u32 {\n\n\tfn one() {}\n}\n";
        assert_eq!(result, expected);

        let block: ImplBlock = block.with_function("two");
        let result: String = CodeBuffer::display_statement(&block);
        let expected: &str = "impl u32 {\n\n\tfn one() {}\n\n\tfn two() {}\n}\n";
        assert_eq!(result, expected);
    }
}
