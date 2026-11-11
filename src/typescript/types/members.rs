use crate::typescript::WithComments;
use crate::{CodeBuffer, Statement};

/// Writes the `members` of a type.
///
/// # Note
/// - An empty line is between two members when either has comments.
pub(super) fn write_members<M>(members: &[M], b: &mut CodeBuffer, level: usize)
where
    M: WithComments + Statement,
{
    if let Some((first, rest)) = members.split_first() {
        first.write(b, level);
        for (previous, member) in members.iter().zip(rest) {
            if !previous.comments().is_empty() || !member.comments().is_empty() {
                b.end_line();
            }
            member.write(b, level);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::typescript::Field;

    /// Writes the `members`.
    fn write(members: &[Field]) -> String {
        let mut b: CodeBuffer = CodeBuffer::default();
        write_members(members, &mut b, 1);
        b.into()
    }

    #[test]
    fn no_members() {
        assert_eq!(write(&[]), "");
    }

    #[test]
    fn members_without_comments() {
        let members = [Field::from("x"), Field::from("y")];
        assert_eq!(write(&members), "    x;\n    y;\n");
    }

    #[test]
    fn members_with_comments() {
        let members = [
            Field::from("x").with_comment(" The x."),
            Field::from("y").with_comment(" The y."),
        ];
        assert_eq!(
            write(&members),
            "    /** The x. */\n    x;\n\n    /** The y. */\n    y;\n"
        );
    }

    #[test]
    fn members_with_and_without_comments() {
        let members = [
            Field::from("x"),
            Field::from("y").with_comment(" The y."),
            Field::from("z"),
            Field::from("w"),
        ];
        assert_eq!(
            write(&members),
            "    x;\n\n    /** The y. */\n    y;\n\n    z;\n    w;\n"
        );
    }
}
