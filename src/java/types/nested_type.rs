use crate::java::{Class, Enum, Interface, Record};
use crate::{CodeBuffer, Statement};

/// A type declared inside another type.
pub enum NestedType {
    Class(Class),
    Enum(Enum),
    Interface(Interface),
    Record(Record),
}

impl From<Class> for NestedType {
    fn from(class: Class) -> Self {
        Self::Class(class)
    }
}

impl From<Enum> for NestedType {
    fn from(enumeration: Enum) -> Self {
        Self::Enum(enumeration)
    }
}

impl From<Interface> for NestedType {
    fn from(interface: Interface) -> Self {
        Self::Interface(interface)
    }
}

impl From<Record> for NestedType {
    fn from(record: Record) -> Self {
        Self::Record(record)
    }
}

impl Statement for NestedType {
    fn write(&self, b: &mut CodeBuffer, level: usize) {
        match self {
            Self::Class(class) => class.write(b, level),
            Self::Enum(enumeration) => enumeration.write(b, level),
            Self::Interface(interface) => interface.write(b, level),
            Self::Record(record) => record.write(b, level),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn nested_class() {
        let t: NestedType = Class::from("Inner").into();
        assert_eq!(t.to_code(), "class Inner {}\n");
    }

    #[test]
    fn nested_enum() {
        let t: NestedType = Enum::from("Inner").into();
        assert_eq!(t.to_code(), "enum Inner {}\n");
    }

    #[test]
    fn nested_interface() {
        let t: NestedType = Interface::from("Inner").into();
        assert_eq!(t.to_code(), "interface Inner {}\n");
    }

    #[test]
    fn nested_record() {
        let t: NestedType = Record::from("Inner").into();
        assert_eq!(t.to_code(), "record Inner() {}\n");
    }
}
