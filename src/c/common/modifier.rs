use crate::{CodeBuffer, Expression};

/// A function modifier.
///
/// # Note
/// - The cases are in the order they are written in.
#[must_use]
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub enum Modifier {
    Static,
    Inline,
}

impl Expression for Modifier {
    fn write(&self, b: &mut CodeBuffer) {
        let keyword: &str = match self {
            Self::Static => "static",
            Self::Inline => "inline",
        };
        b.write(keyword);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn static_modifier() {
        assert_eq!(Modifier::Static.to_code(), "static");
    }

    #[test]
    fn inline_modifier() {
        assert_eq!(Modifier::Inline.to_code(), "inline");
    }

    #[test]
    fn static_is_before_inline() {
        assert!(Modifier::Static < Modifier::Inline);
    }
}
