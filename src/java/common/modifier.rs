use crate::{CodeBuffer, Expression};

/// A keyword modifier of a declaration, other than its access level.
///
/// # Note
/// - The modifiers are in the order they are written.
#[must_use]
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub enum Modifier {
    /// The modifier of an interface function with a body.
    Default,

    Static,
    Final,
    Sealed,
}

impl Expression for Modifier {
    fn write(&self, b: &mut CodeBuffer) {
        match self {
            Self::Default => b.write("default "),
            Self::Static => b.write("static "),
            Self::Final => b.write("final "),
            Self::Sealed => b.write("sealed "),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default() {
        assert_eq!(Modifier::Default.to_code(), "default ");
    }

    #[test]
    fn r#static() {
        assert_eq!(Modifier::Static.to_code(), "static ");
    }

    #[test]
    fn r#final() {
        assert_eq!(Modifier::Final.to_code(), "final ");
    }

    #[test]
    fn sealed() {
        assert_eq!(Modifier::Sealed.to_code(), "sealed ");
    }
}
