use crate::CodeBuffer;

/// An element with annotations.
pub trait WithAnnotations: Sized {
    /// Gets the annotations.
    fn annotations(&self) -> &[String];

    /// Adds the annotation, without its `@`. (ex: `JvmInline`)
    fn add_annotation<S>(&mut self, annotation: S)
    where
        S: Into<String>;

    /// Adds the annotation, without its `@`. (ex: `JvmInline`)
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
    use crate::kotlin::Function;

    /// Writes the annotations of the `function` at the indent `level`.
    fn code(function: &Function, level: usize) -> String {
        let mut b = CodeBuffer::default();
        function.write_annotations(&mut b, level);
        b.into()
    }

    #[test]
    fn no_annotations() {
        assert_eq!(code(&Function::from("f"), 0), "");
    }

    #[test]
    fn annotations() {
        let f = Function::from("f")
            .with_annotation("JvmStatic")
            .with_annotation("Deprecated(\"Use g.\")");
        assert_eq!(code(&f, 1), "    @JvmStatic\n    @Deprecated(\"Use g.\")\n");
    }
}
