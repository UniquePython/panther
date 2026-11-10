use crate::tokenkind::TokenKind;

pub(crate) const KEYWORDS: &[(&'static [u8], TokenKind<'static>)] = &[
    (b"true", TokenKind::BoolLiteral(true)),
    (b"false", TokenKind::BoolLiteral(false)),
    (b"unit", TokenKind::UnitLiteral),
    (b"decl", TokenKind::Decl),
    (b"mut", TokenKind::Mut),
    (b"const", TokenKind::Const),
    (b"if", TokenKind::If),
    (b"unless", TokenKind::Unless),
    (b"else", TokenKind::Else),
    (b"when", TokenKind::When),
    (b"while", TokenKind::While),
    (b"repeat", TokenKind::Repeat),
    (b"break", TokenKind::Break),
    (b"continue", TokenKind::Continue),
];
