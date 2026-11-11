use crate::CodeBuffer;
use crate::go::WithComments;

/// Writes the members of a declaration with the `write` function.
///
/// # Note
/// - An empty line goes between two members when either has comments.
pub(crate) fn write_members<M, F>(members: &[M], write: F, b: &mut CodeBuffer, level: usize)
where
    M: WithComments,
    F: Fn(&M, &mut CodeBuffer, usize),
{
    if let Some((first, rest)) = members.split_first() {
        write(first, b, level);
        let mut previous: &M = first;
        for member in rest {
            if !(previous.comments().is_empty() && member.comments().is_empty()) {
                b.end_line();
            }
            write(member, b, level);
            previous = member;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Statement;
    use crate::go::StructField;

    /// Writes the `fields` as members.
    fn code(fields: &[StructField]) -> String {
        let mut b = CodeBuffer::default();
        write_members(fields, StructField::write, &mut b, 1);
        b.into()
    }

    #[test]
    fn no_members() {
        assert_eq!(code(&[]), "");
    }

    #[test]
    fn members_without_comments() {
        let fields = [
            StructField::from(("x", "uint32")),
            StructField::from(("y", "uint32")),
        ];
        assert_eq!(code(&fields), "    x uint32\n    y uint32\n");
    }

    #[test]
    fn members_with_comments() {
        let fields = [
            StructField::from(("a", "bool")).with_comment(" The first."),
            StructField::from(("b", "bool")),
            StructField::from(("c", "bool")),
            StructField::from(("d", "bool")).with_comment(" The last."),
        ];
        assert_eq!(
            code(&fields),
            "    // The first.\n    a bool\n\n    b bool\n    c bool\n\n    // The last.\n    d bool\n"
        );
    }
}
