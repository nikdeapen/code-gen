use crate::{CodeBuffer, Expression};

/// A function receiver.
#[must_use]
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug, Default)]
pub enum Receiver {
    /// The `self` of a method.
    #[default]
    Instance,

    /// The `cls` of a class method.
    Class,
}

impl Expression for Receiver {
    fn write(&self, b: &mut CodeBuffer) {
        let name: &str = match self {
            Self::Instance => "self",
            Self::Class => "cls",
        };
        b.write(name);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn instance() {
        assert_eq!(Receiver::Instance.to_code(), "self");
    }

    #[test]
    fn class() {
        assert_eq!(Receiver::Class.to_code(), "cls");
    }

    #[test]
    fn default_is_instance() {
        assert_eq!(Receiver::default(), Receiver::Instance);
    }
}
