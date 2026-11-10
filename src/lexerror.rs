use crate::span::Span;

pub(crate) enum LexError<'src> {
    UnexpectedChar { ch: char, span: Span },
    IntOverflow { lexeme: &'src str, span: Span },
}
