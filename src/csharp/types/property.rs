use crate::csharp::{
    Access, CommentType, Modifier, Var, WithAccess, WithAttributes, WithComments, WithModifiers,
    WithVar,
};
use crate::{CodeBuffer, Statement};
use std::collections::BTreeSet;

/// A property declaration.
///
/// # Note
/// - A property with no accessors & no expression body is written with a `get` accessor.
#[derive(Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub struct Property {
    comments: Vec<String>,
    attributes: Vec<String>,
    access: Access,
    modifiers: BTreeSet<Modifier>,
    var: Var,
    accessors: Vec<String>,
    expression: Option<String>,
}

impl<V: Into<Var>> From<V> for Property {
    fn from(var: V) -> Self {
        Self {
            comments: Vec::default(),
            attributes: Vec::default(),
            access: Access::default(),
            modifiers: BTreeSet::default(),
            var: var.into(),
            accessors: Vec::default(),
            expression: None,
        }
    }
}

impl Property {
    //! Accessors

    /// Gets the accessors.
    #[must_use]
    pub fn accessors(&self) -> &[String] {
        self.accessors.as_slice()
    }

    /// Adds the accessor. (ex: `get`, `private set`, `init` or `get => _x`)
    pub fn add_accessor<S>(&mut self, accessor: S)
    where
        S: Into<String>,
    {
        self.accessors.push(accessor.into());
    }

    /// Adds the accessor. (ex: `get`, `private set`, `init` or `get => _x`)
    #[must_use]
    pub fn with_accessor<S>(mut self, accessor: S) -> Self
    where
        S: Into<String>,
    {
        self.add_accessor(accessor);
        self
    }
}

impl Property {
    //! Expression

    /// Gets the optional expression body.
    #[must_use]
    pub fn expression(&self) -> Option<&str> {
        self.expression.as_deref()
    }

    /// Sets the expression body, which is written in place of the accessors.
    pub fn set_expression<S>(&mut self, expression: S)
    where
        S: Into<String>,
    {
        self.expression = Some(expression.into());
    }

    /// Sets the expression body, which is written in place of the accessors.
    #[must_use]
    pub fn with_expression<S>(mut self, expression: S) -> Self
    where
        S: Into<String>,
    {
        self.set_expression(expression);
        self
    }
}

impl WithComments for Property {
    fn comments(&self) -> &[String] {
        self.comments.as_slice()
    }

    fn add_comment<S>(&mut self, comment: S)
    where
        S: Into<String>,
    {
        self.comments.push(comment.into());
    }
}

impl WithAttributes for Property {
    fn attributes(&self) -> &[String] {
        self.attributes.as_slice()
    }

    fn add_attribute<S>(&mut self, attribute: S)
    where
        S: Into<String>,
    {
        self.attributes.push(attribute.into());
    }
}

impl WithAccess for Property {
    fn access(&self) -> Access {
        self.access
    }

    fn set_access(&mut self, access: Access) {
        self.access = access;
    }
}

impl WithModifiers for Property {
    fn modifiers(&self) -> &BTreeSet<Modifier> {
        &self.modifiers
    }

    fn add_modifier(&mut self, modifier: Modifier) {
        self.modifiers.insert(modifier);
    }
}

impl WithVar for Property {
    fn var(&self) -> &Var {
        &self.var
    }
}

impl Statement for Property {
    fn write(&self, b: &mut CodeBuffer, level: usize) {
        self.write_comments(CommentType::Doc, b, level);
        self.write_attributes(b, level);
        b.indent(level);
        self.write_access(b);
        self.write_modifiers(b);
        self.write_var(b);
        if let Some(expression) = &self.expression {
            b.write(" => ");
            b.write(expression.as_str());
            b.push(';');
        } else {
            b.write(" { ");
            if self.accessors.is_empty() {
                b.write("get; ");
            }
            for accessor in &self.accessors {
                b.write(accessor.as_str());
                b.write("; ");
            }
            b.push('}');
        }
        b.end_line();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn property_without_accessors() {
        let p: Property = Property::from(("X", "int"));
        assert_eq!(p.to_code(), "int X { get; }\n");
    }

    #[test]
    fn property_with_accessors() {
        let p: Property = Property::from(("Name", "string?"))
            .with_access(Access::Public)
            .with_accessor("get")
            .with_accessor("private set");
        assert_eq!(p.to_code(), "public string? Name { get; private set; }\n");
    }

    #[test]
    fn property_with_accessor_bodies() {
        let p: Property = Property::from(("X", "int"))
            .with_access(Access::Public)
            .with_accessor("get => _x")
            .with_accessor("set => _x = value");
        assert_eq!(
            p.to_code(),
            "public int X { get => _x; set => _x = value; }\n"
        );
    }

    #[test]
    fn property_with_expression() {
        let p: Property = Property::from(("TagNumber", "ushort"))
            .with_access(Access::Public)
            .with_modifier(Modifier::Override)
            .with_accessor("get")
            .with_expression("1");
        assert_eq!(p.to_code(), "public override ushort TagNumber => 1;\n");
    }

    #[test]
    fn property_with_comment_and_attribute() {
        let p: Property = Property::from(("Name", "string"))
            .with_comment(" The name.")
            .with_attribute("JsonPropertyName(\"name\")")
            .with_access(Access::Public)
            .with_modifier(Modifier::Required)
            .with_accessor("get")
            .with_accessor("init");
        let mut b: CodeBuffer = CodeBuffer::default();
        p.write(&mut b, 1);
        assert_eq!(
            b.peek(),
            "    /// <summary>\n    /// The name.\n    /// </summary>\n    [JsonPropertyName(\"name\")]\n    public required string Name { get; init; }\n"
        );
    }
}
