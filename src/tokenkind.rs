#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) enum TokenKind<'src> {
    // Literals
    IntLiteral(i64),
    BoolLiteral(bool),
    UnitLiteral,

    // Identifiers
    LowerIdent(&'src str),
    UpperIdent(&'src str),

    // Keywords
    Decl,
    Mut,
    Const,
    If,
    Unless,
    Else,
    When,
    While,
    Repeat,
    Break,
    Continue,

    // Operators
    // - Arithmetic
    Plus,
    Minus,
    Star,
    Slash,
    Percent,
    // - Comparison
    EqEq,
    BangEq,
    Lt,
    Gt,
    Le,
    Ge,
    BangLt,
    BangGt,
    BangLe,
    BangGe,
    // - Logical
    AmpAmp,
    PipePipe,
    Bang,
    // - Assignment
    ColonEq,
    EqColon,
    // - Binding
    Eq,

    // Punctuation
    Colon,
    Semi,
    LBrace,
    RBrace,
    LParen,
    RParen,

    // Special
    Eof,
}
