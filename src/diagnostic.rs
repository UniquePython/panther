use crate::lexerror::LexError;
use crate::parseerror::ParseError;

pub(crate) enum Diagnostic<'src> {
    Lex(LexError<'src>),
    Parse(ParseError<'src>),
}
