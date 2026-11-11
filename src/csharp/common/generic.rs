use crate::WithName;
use crate::csharp::CSharpType;

/// A type parameter, with optional constraints.
///
/// # Note
/// - A constraint is a type or a keyword. (ex: `IDisposable`, `class` or `new()`)
/// - The constraints are written as they are added. C# wants `class` or `struct` first & `new()`
///   last.
#[derive(Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub struct Generic {
    name: String,
    constraints: Vec<CSharpType>,
}

impl<S: Into<String>> From<S> for Generic {
    fn from(name: S) -> Self {
        Self {
            name: name.into(),
            constraints: Vec::default(),
        }
    }
}

impl Generic {
    //! Constraints

    /// Gets the constraints.
    #[must_use]
    pub fn constraints(&self) -> &[CSharpType] {
        self.constraints.as_slice()
    }

    /// Adds the constraint.
    pub fn add_constraint<T>(&mut self, constraint: T)
    where
        T: Into<CSharpType>,
    {
        self.constraints.push(constraint.into());
    }

    /// Adds the constraint.
    #[must_use]
    pub fn with_constraint<T>(mut self, constraint: T) -> Self
    where
        T: Into<CSharpType>,
    {
        self.add_constraint(constraint);
        self
    }
}

impl WithName for Generic {
    fn name(&self) -> &str {
        self.name.as_str()
    }
}
