use crate::{CodeBuffer, Statement};

/// An `#include` directive.
///
/// # Note
/// - It is the quote form, unless it is set to a system include. (ex: `#include "point.h"`)
#[derive(Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub struct Include {
    path: String,
    is_system: bool,
}

impl<S: Into<String>> From<S> for Include {
    fn from(path: S) -> Self {
        Self {
            path: path.into(),
            is_system: false,
        }
    }
}

impl Include {
    //! Path

    /// Gets the path of the included file.
    #[must_use]
    pub fn path(&self) -> &str {
        self.path.as_str()
    }
}

impl Include {
    //! System

    /// Checks if the include is a system include, the angle bracket form. (ex: `<stdint.h>`)
    #[must_use]
    pub fn is_system(&self) -> bool {
        self.is_system
    }

    /// Sets if the include is a system include.
    pub fn set_system(&mut self, is_system: bool) {
        self.is_system = is_system;
    }

    /// Sets if the include is a system include.
    #[must_use]
    pub fn with_system(mut self, is_system: bool) -> Self {
        self.set_system(is_system);
        self
    }
}

impl Statement for Include {
    fn write(&self, b: &mut CodeBuffer, level: usize) {
        b.indent(level);
        b.write("#include ");
        b.push(if self.is_system { '<' } else { '"' });
        b.write(self.path.as_str());
        b.push(if self.is_system { '>' } else { '"' });
        b.end_line();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn quote_include() {
        let i: Include = Include::from("point.h");
        assert_eq!(i.path(), "point.h");
        assert!(!i.is_system());
        assert_eq!(i.to_code(), "#include \"point.h\"\n");
    }

    #[test]
    fn system_include() {
        let i: Include = Include::from("stdint.h").with_system(true);
        assert!(i.is_system());
        assert_eq!(i.to_code(), "#include <stdint.h>\n");
    }

    #[test]
    fn include_indented() {
        let i: Include = Include::from("point.h");
        let mut b: CodeBuffer = CodeBuffer::default();
        i.write(&mut b, 1);
        assert_eq!(b.peek(), "    #include \"point.h\"\n");
    }
}
