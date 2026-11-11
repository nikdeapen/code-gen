use crate::CodeBuffer;

/// An element with annotations.
pub trait WithAnnotations: Sized {
    /// Gets the annotations.
    fn annotations(&self) -> &[String];

    /// Adds an annotation, without its `@`.
    fn add_annotation<S>(&mut self, annotation: S)
    where
        S: Into<String>;

    /// Adds an annotation, without its `@`.
    #[must_use]
    fn with_annotation<S>(mut self, annotation: S) -> Self
    where
        S: Into<String>,
    {
        self.add_annotation(annotation);
        self
    }

    /// Writes the annotations, each on its own line.
    fn write_annotations(&self, b: &mut CodeBuffer, level: usize) {
        for annotation in self.annotations() {
            b.indent(level);
            b.push('@');
            b.write(annotation);
            b.end_line();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::java::Function;

    #[test]
    fn annotations() {
        let function: Function = Function::from("f")
            .with_annotation("Override")
            .with_annotation("SuppressWarnings(\"unchecked\")");
        let mut b: CodeBuffer = CodeBuffer::default();
        function.write_annotations(&mut b, 1);
        assert_eq!(
            b.peek(),
            "    @Override\n    @SuppressWarnings(\"unchecked\")\n"
        );
    }
}
