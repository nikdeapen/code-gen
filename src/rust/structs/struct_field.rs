use crate::rust::{Access, TypeTag, Var, WithAccess, WithTypeTag, WithVar};
use crate::{CodeBuffer, Statement, WithName};

/// A field of a struct.
#[derive(Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub struct StructField {
    access: Access,
    var: Var,
}

impl<V: Into<Var>> From<V> for StructField {
    fn from(var: V) -> Self {
        Self {
            access: Access::default(),
            var: var.into(),
        }
    }
}

impl WithAccess for StructField {
    fn access(&self) -> &Access {
        &self.access
    }

    fn set_access<A>(&mut self, access: A)
    where
        A: Into<Access>,
    {
        self.access = access.into();
    }
}

impl WithName for StructField {
    fn name(&self) -> &str {
        self.var.name()
    }
}

impl WithTypeTag for StructField {
    fn type_tag(&self) -> &TypeTag {
        self.var.type_tag()
    }
}

impl WithVar for StructField {
    fn var(&self) -> &Var {
        &self.var
    }
}

impl Statement for StructField {
    fn write(&self, b: &mut CodeBuffer, level: usize) {
        b.indent(level);
        self.write_access(b);
        self.write_var(b);
        b.write(",");
        b.end_line();
    }
}

#[cfg(test)]
mod tests {
    use crate::rust::Access::Public;
    use crate::rust::PrimitiveType::UnsignedInt32;
    use crate::rust::{StructField, WithAccess};
    use crate::CodeBuffer;

    #[test]
    fn write() {
        let field: StructField = ("one", UnsignedInt32).into();
        let result: String = CodeBuffer::display_statement(&field);
        assert_eq!(result, "one: u32,\n");
    }

    #[test]
    fn write_access() {
        let field: StructField = StructField::from(("one", UnsignedInt32)).with_access(Public);
        let result: String = CodeBuffer::display_statement(&field);
        assert_eq!(result, "pub one: u32,\n");
    }
}
