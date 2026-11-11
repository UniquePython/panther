use crate::lexerror::LexError;
use crate::parseerror::ParseError;
use crate::typeerror::TypeError;

pub(crate) enum Diagnostic<'src> {
    Lex(LexError<'src>),
    Parse(ParseError<'src>),
    Type(TypeError),
}
