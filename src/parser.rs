use crate::token::Token;

pub(crate) struct Parser<'src> {
    tokens: Vec<Token<'src>>,
    cursor: usize,
}
