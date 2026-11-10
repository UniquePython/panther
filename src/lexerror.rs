use crate::span::Span;

pub(crate) enum LexError<'src> {
    UnexpectedByte { byte: u8, span: Span },
    NonAsciiByte { byte: u8, span: Span },
    IntOverflow { lexeme: &'src [u8], span: Span },
}
