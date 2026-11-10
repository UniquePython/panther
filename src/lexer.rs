use crate::diagnostic::Diagnostic;
use crate::keywords::KEYWORDS;
use crate::lexerror::LexError;
use crate::span::Span;
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
            b'a'..=b'z' | b'_' => Ok(self.scan_lower_identifier()),
            b'A'..=b'Z' => Ok(self.scan_upper_identifier()),
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
        while !self.is_at_end() && matches!(self.peek(), b' ' | b'\t' | b'\n' | b'\r') {
            self.cursor += 1
        }
    }

    fn is_at_end(&self) -> bool {
        self.cursor >= self.src.len()
    }

    fn peek(&self) -> u8 {
        self.src[self.cursor]
    }

    fn scan_int_literal(&mut self) -> Result<Token<'src>, Diagnostic<'src>> {
        let start = self.cursor;
        let mut value: i64 = 0;
        let mut overflowed = false;

        while !self.is_at_end() && self.peek().is_ascii_digit() {
            let digit = (self.peek() - b'0') as i64;
            if !overflowed {
                match value.checked_mul(10).and_then(|v| v.checked_add(digit)) {
                    Some(v) => value = v,
                    None => overflowed = true,
                }
            }
            self.cursor += 1;
        }

        let span = Span::new(start, self.cursor);

        if overflowed {
            let lexeme = &self.src[start..self.cursor];
            return Err(Diagnostic::Lex(LexError::IntOverflow { lexeme, span }));
        } else {
            return Ok(Token::new(TokenKind::IntLiteral(value), span));
        }
    }

    fn scan_lower_identifier(&mut self) -> Token<'src> {
        let start = self.cursor;

        while !self.is_at_end() && (self.peek().is_ascii_alphanumeric() || self.peek() == b'_') {
            self.cursor += 1;
        }

        let lexeme = &self.src[start..self.cursor];
        let span = Span::new(start, self.cursor);

        let kind = KEYWORDS
            .iter()
            .find_map(|(keyword, kind)| (*keyword == lexeme).then_some(*kind))
            .unwrap_or(TokenKind::LowerIdent(lexeme));

        Token::new(kind, span)
    }

    fn scan_upper_identifier(&mut self) -> Token<'src> {
        let start = self.cursor;

        while !self.is_at_end() && (self.peek().is_ascii_alphanumeric() || self.peek() == b'_') {
            self.cursor += 1;
        }

        let lexeme = &self.src[start..self.cursor];
        let span = Span::new(start, self.cursor);

        Token::new(TokenKind::UpperIdent(lexeme), span)
    }

    fn scan_operator(&mut self) -> Result<Token<'src>, Diagnostic<'src>> {
        todo!()
    }
}
