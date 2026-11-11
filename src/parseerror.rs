use crate::span::Span;
use crate::tokenkind::TokenKind;

pub(crate) enum ParseError<'src> {
    UnexpectedToken {
        expected: &'static str,
        found: TokenKind<'src>,
        span: Span,
    },
    InvalidAssignmentTarget {
        span: Span,
    },
}
