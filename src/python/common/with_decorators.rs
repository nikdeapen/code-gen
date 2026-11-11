use crate::CodeBuffer;

/// An element with decorators.
pub trait WithDecorators: Sized {
    /// Gets the decorators.
    fn decorators(&self) -> &[String];

    /// Adds a decorator. (ex: `staticmethod` or `dataclass(frozen=True)`)
    fn add_decorator<S>(&mut self, decorator: S)
    where
        S: Into<String>;

    /// Adds a decorator. (ex: `staticmethod` or `dataclass(frozen=True)`)
    #[must_use]
    fn with_decorator<S>(mut self, decorator: S) -> Self
    where
        S: Into<String>,
    {
        self.add_decorator(decorator);
        self
    }

    /// Writes the decorators.
    fn write_decorators(&self, b: &mut CodeBuffer, level: usize) {
        for decorator in self.decorators() {
            b.indent(level);
            b.push('@');
            b.write(decorator);
            b.end_line();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::python::Function;

    #[test]
    fn decorators() {
        let function = Function::from("foo")
            .with_decorator("staticmethod")
            .with_decorator("cache");
        let mut b = CodeBuffer::default();
        function.write_decorators(&mut b, 1);
        assert_eq!(b.peek(), "    @staticmethod\n    @cache\n");
    }
}
