use std::fmt::{Display, Formatter};

use crate::ToStaticStr;

/// A function receiver.
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub enum Receiver {
    Borrowed,
    BorrowedMut,
    Owned,
    OwnedMut,
}

impl ToStaticStr for Receiver {
    fn to_static_str(&self) -> &'static str {
        match self {
            Self::Borrowed => "&self",
            Self::BorrowedMut => "&mut self",
            Self::Owned => "self",
            Self::OwnedMut => "mut self",
        }
    }
}

impl Display for Receiver {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_static_str())
    }
}

#[cfg(test)]
mod tests {
    use crate::rust::function::Receiver::*;

    #[test]
    fn display() {
        assert_eq!(Borrowed.to_string(), "&self");
        assert_eq!(BorrowedMut.to_string(), "&mut self");
        assert_eq!(Owned.to_string(), "self");
        assert_eq!(OwnedMut.to_string(), "mut self");
    }
}
