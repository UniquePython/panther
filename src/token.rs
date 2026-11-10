use crate::span::Span;
use crate::tokenkind::TokenKind;

#[derive(Debug, Clone, Copy)]
pub(crate) struct Token<'src> {
    kind: TokenKind<'src>,
    span: Span,
}

impl<'src> Token<'src> {
    pub(crate) fn new(kind: TokenKind<'src>, span: Span) -> Self {
        Self { kind, span }
    }

    pub(crate) fn eof(at: usize) -> Self {
        Self::new(TokenKind::Eof, Span::at(at))
    }

    pub(crate) fn kind(&self) -> TokenKind<'src> {
        self.kind
    }

    pub(crate) fn span(&self) -> Span {
        self.span
    }
}
