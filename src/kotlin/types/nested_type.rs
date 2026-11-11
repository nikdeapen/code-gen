use crate::kotlin::{Class, Interface, Object};
use crate::{CodeBuffer, Statement};

/// A type declared in another type.
pub enum NestedType {
    Class(Class),
    Interface(Interface),
    Object(Object),
}

impl From<Class> for NestedType {
    fn from(class: Class) -> Self {
        Self::Class(class)
    }
}

impl From<Interface> for NestedType {
    fn from(interface: Interface) -> Self {
        Self::Interface(interface)
    }
}

impl From<Object> for NestedType {
    fn from(object: Object) -> Self {
        Self::Object(object)
    }
}

impl Statement for NestedType {
    fn write(&self, b: &mut CodeBuffer, level: usize) {
        match self {
            Self::Class(class) => class.write(b, level),
            Self::Interface(interface) => interface.write(b, level),
            Self::Object(object) => object.write(b, level),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn nested_class() {
        let t = NestedType::from(Class::from("Circle"));
        assert_eq!(t.to_code(), "class Circle\n");
    }

    #[test]
    fn nested_interface() {
        let t = NestedType::from(Interface::from("Shape"));
        assert_eq!(t.to_code(), "interface Shape\n");
    }

    #[test]
    fn nested_object() {
        let t = NestedType::from(Object::from("Red"));
        let mut b = CodeBuffer::default();
        t.write(&mut b, 1);
        assert_eq!(b.peek(), "    object Red\n");
    }
}
