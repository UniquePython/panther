use crate::diagnostic::Diagnostic;
use crate::parseerror::ParseError;
use crate::token::Token;
use crate::tokenkind::TokenKind;

pub(crate) struct Parser<'src> {
    tokens: Vec<Token<'src>>,
    cursor: usize,
}

impl<'src> Parser<'src> {
    fn is_at_end(&self) -> bool {
        matches!(self.peek().kind(), TokenKind::Eof)
    }

    fn peek(&self) -> Token<'src> {
        self.tokens.get(self.cursor).copied().unwrap_or_else(|| {
            self.tokens
                .last()
                .copied()
                .expect("token stream is never empty")
        })
    }

    fn advance(&mut self) -> Token<'src> {
        let token = self.peek();
        if !self.is_at_end() {
            self.cursor += 1;
        }
        token
    }

    fn check(&self, kind: TokenKind<'src>) -> bool {
        self.peek().kind() == kind
    }

    fn expect(
        &mut self,
        kind: TokenKind<'src>,
        description: &'static str,
    ) -> Result<Token<'src>, Diagnostic<'src>> {
        if self.check(kind) {
            Ok(self.advance())
        } else {
            Err(Diagnostic::Parse(ParseError::UnexpectedToken {
                expected: description,
                found: self.peek().kind(),
                span: self.peek().span(),
            }))
        }
    }
}
