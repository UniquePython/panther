use crate::diagnostic::Diagnostic;
use crate::token::Token;
use crate::tokenkind::TokenKind;

pub(crate) struct Lexer<'src> {
    src: &'src [u8],
    cursor: usize,
}

impl<'src> Lexer<'src> {
    pub(crate) fn new(src: &'src str) -> Self {
        Self {
            src: src.as_bytes(),
            cursor: 0,
        }
    }

    fn next_token(&mut self) -> Result<Token<'src>, Diagnostic<'src>> {
        self.skip_whitespace();

        if self.is_at_end() {
            return Ok(Token::eof(self.cursor));
        }

        match self.peek() {
            b'0'..=b'9' => self.scan_int_literal(),
            b'a'..=b'z' | b'_' => self.scan_lower_identifier(),
            b'A'..=b'Z' => self.scan_upper_identifier(),
            _ => self.scan_operator(),
        }
    }

    pub(crate) fn lex(mut self) -> Result<Vec<Token<'src>>, Diagnostic<'src>> {
        let mut tokens = Vec::new();
        loop {
            let token = self.next_token()?;
            let is_eof = token.kind() == TokenKind::Eof;
            tokens.push(token);
            if is_eof {
                return Ok(tokens);
            }
        }
    }

    fn skip_whitespace(&mut self) {
        todo!()
    }

    fn is_at_end(&self) -> bool {
        todo!()
    }

    fn peek(&self) -> u8 {
        self.src[self.cursor]
    }

    fn scan_int_literal(&mut self) -> Result<Token<'src>, Diagnostic<'src>> {
        todo!()
    }

    fn scan_lower_identifier(&mut self) -> Result<Token<'src>, Diagnostic<'src>> {
        todo!()
    }

    fn scan_upper_identifier(&mut self) -> Result<Token<'src>, Diagnostic<'src>> {
        todo!()
    }

    fn scan_operator(&mut self) -> Result<Token<'src>, Diagnostic<'src>> {
        todo!()
    }
}
