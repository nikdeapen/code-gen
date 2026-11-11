use crate::{CodeBuffer, Expression};

/// A modifier keyword of a declaration.
///
/// # Note
/// - The cases are in the order they are written in.
#[must_use]
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub enum Modifier {
    Const,
    Static,
    Virtual,
    Abstract,
    Sealed,
    Override,
    Readonly,
    Required,
    Async,
    Partial,
}

impl Expression for Modifier {
    fn write(&self, b: &mut CodeBuffer) {
        let s: &str = match self {
            Self::Const => "const ",
            Self::Static => "static ",
            Self::Virtual => "virtual ",
            Self::Abstract => "abstract ",
            Self::Sealed => "sealed ",
            Self::Override => "override ",
            Self::Readonly => "readonly ",
            Self::Required => "required ",
            Self::Async => "async ",
            Self::Partial => "partial ",
        };
        b.write(s);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn keywords() {
        assert_eq!(Modifier::Const.to_code(), "const ");
        assert_eq!(Modifier::Static.to_code(), "static ");
        assert_eq!(Modifier::Virtual.to_code(), "virtual ");
        assert_eq!(Modifier::Abstract.to_code(), "abstract ");
        assert_eq!(Modifier::Sealed.to_code(), "sealed ");
        assert_eq!(Modifier::Override.to_code(), "override ");
        assert_eq!(Modifier::Readonly.to_code(), "readonly ");
        assert_eq!(Modifier::Required.to_code(), "required ");
        assert_eq!(Modifier::Async.to_code(), "async ");
        assert_eq!(Modifier::Partial.to_code(), "partial ");
    }
}
