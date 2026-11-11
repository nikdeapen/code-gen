use crate::java::JavaType;
use crate::{CodeBuffer, Expression};

/// An element that implements interfaces.
pub trait WithImplements: Sized {
    /// Gets the interfaces the element implements.
    fn implements(&self) -> &[JavaType];

    /// Adds an interface the element implements.
    fn add_implements<T>(&mut self, interface: T)
    where
        T: Into<JavaType>;

    /// Adds an interface the element implements.
    #[must_use]
    fn with_implements<T>(mut self, interface: T) -> Self
    where
        T: Into<JavaType>,
    {
        self.add_implements(interface);
        self
    }

    /// Writes the `implements` clause, after a space.
    fn write_implements(&self, b: &mut CodeBuffer) {
        if let Some((first, rest)) = self.implements().split_first() {
            b.write(" implements ");
            first.write(b);
            for interface in rest {
                b.write(", ");
                interface.write(b);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::java::Class;

    /// Writes the `implements` clause of the `class`.
    fn write(class: Class) -> String {
        let mut b: CodeBuffer = CodeBuffer::default();
        class.write_implements(&mut b);
        b.into()
    }

    #[test]
    fn no_interfaces() {
        assert_eq!(write(Class::from("Point")), "");
    }

    #[test]
    fn one_interface() {
        let class: Class = Class::from("Point").with_implements("PointView");
        assert_eq!(write(class), " implements PointView");
    }

    #[test]
    fn many_interfaces() {
        let class: Class = Class::from("Point")
            .with_implements("PointView")
            .with_implements(JavaType::from("Comparable").with_generic("Point"));
        assert_eq!(write(class), " implements PointView, Comparable<Point>");
    }
}
