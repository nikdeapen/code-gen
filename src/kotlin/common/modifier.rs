use crate::{CodeBuffer, Expression};

/// A keyword modifier.
///
/// # Note
/// - The cases are in the order of the Kotlin coding conventions. They are written in that order.
#[must_use]
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub enum Modifier {
    Sealed,
    Const,
    Override,
    Suspend,
    Value,
    Data,
}

impl Expression for Modifier {
    fn write(&self, b: &mut CodeBuffer) {
        let keyword: &str = match self {
            Self::Sealed => "sealed",
            Self::Const => "const",
            Self::Override => "override",
            Self::Suspend => "suspend",
            Self::Value => "value",
            Self::Data => "data",
        };
        b.write(keyword);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn keywords() {
        assert_eq!(Modifier::Sealed.to_code(), "sealed");
        assert_eq!(Modifier::Const.to_code(), "const");
        assert_eq!(Modifier::Override.to_code(), "override");
        assert_eq!(Modifier::Suspend.to_code(), "suspend");
        assert_eq!(Modifier::Value.to_code(), "value");
        assert_eq!(Modifier::Data.to_code(), "data");
    }
}
