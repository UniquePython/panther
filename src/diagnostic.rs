use crate::lexerror::LexError;

pub(crate) enum Diagnostic<'src> {
    Lex(LexError<'src>),
}
