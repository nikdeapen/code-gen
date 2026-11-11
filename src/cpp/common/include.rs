use crate::{CodeBuffer, Statement};

/// An `#include` directive. (ex: `#include <vector>` or `#include "point.h"`)
#[derive(Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub struct Include {
    path: String,
    is_quoted: bool,
}

impl<S: Into<String>> From<S> for Include {
    fn from(path: S) -> Self {
        Self {
            path: path.into(),
            is_quoted: false,
        }
    }
}

impl Include {
    //! Quoted

    /// Checks if the path is in quotes, not in angle brackets.
    #[must_use]
    pub fn is_quoted(&self) -> bool {
        self.is_quoted
    }

    /// Sets if the path is in quotes, not in angle brackets.
    pub fn set_quoted(&mut self, is_quoted: bool) {
        self.is_quoted = is_quoted;
    }

    /// Sets if the path is in quotes, not in angle brackets.
    #[must_use]
    pub fn with_quoted(mut self, is_quoted: bool) -> Self {
        self.set_quoted(is_quoted);
        self
    }
}

impl Statement for Include {
    fn write(&self, b: &mut CodeBuffer, level: usize) {
        b.indent(level);
        b.write("#include ");
        b.push(if self.is_quoted { '"' } else { '<' });
        b.write(self.path.as_str());
        b.push(if self.is_quoted { '"' } else { '>' });
        b.end_line();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn angle_include() {
        let include = Include::from("cstdint");
        assert!(!include.is_quoted());
        assert_eq!(include.to_code(), "#include <cstdint>\n");
    }

    #[test]
    fn quoted_include() {
        let include = Include::from("shapes/point.h").with_quoted(true);
        assert!(include.is_quoted());
        assert_eq!(include.to_code(), "#include \"shapes/point.h\"\n");
    }
}
