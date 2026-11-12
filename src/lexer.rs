use crate::diagnostic::Diagnostic;
use crate::keywords::KEYWORDS;
use crate::lexerror::LexError;
use crate::lookahead::LOOKAHEAD;
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

    pub(crate) fn lex(mut self) -> Result<Vec<Token<'src>>, Diagnostic<'src>> {
        let mut tokens = Vec::new();

        loop {
            let token = self.next_token()?;
            let is_eof = token.kind() == TokenKind::Eof;

            tokens.push(token);

            if is_eof {
                break;
            }
        }

        Ok(tokens)
    }

    fn is_at_end(&self) -> bool {
        self.peek().is_none()
    }

    fn peek(&self) -> Option<u8> {
        self.peek_by(0)
    }

    fn peek_by(&self, offset: usize) -> Option<u8> {
        self.src.get(self.cursor.saturating_add(offset)).copied()
    }

    fn advance(&mut self) -> Option<u8> {
        let byte = self.peek()?;
        self.cursor += 1;
        Some(byte)
    }

    fn consume_while(&mut self, mut predicate: impl FnMut(u8) -> bool) {
        while let Some(byte) = self.peek() {
            if !predicate(byte) {
                break;
            }

            self.advance();
        }
    }

    fn span_from(&self, start: usize) -> Span {
        Span::new(start, self.cursor)
    }

    fn lexeme_from(&self, start: usize) -> &'src [u8] {
        &self.src[start..self.cursor]
    }

    fn make_token(&self, kind: TokenKind<'src>, start: usize) -> Token<'src> {
        Token::new(kind, self.span_from(start))
    }

    fn next_token(&mut self) -> Result<Token<'src>, Diagnostic<'src>> {
        self.skip_whitespace();

        let Some(byte) = self.peek() else {
            return Ok(Token::eof(self.cursor));
        };

        match byte {
            b'0'..=b'9' => self.scan_int_literal(),

            b'a'..=b'z' | b'_' => Ok(self.scan_identifier(false)),

            b'A'..=b'Z' => Ok(self.scan_identifier(true)),

            _ => self.scan_operator(),
        }
    }

    fn skip_whitespace(&mut self) {
        self.consume_while(|byte| matches!(byte, b' ' | b'\t' | b'\n' | b'\r'));
    }

    fn scan_int_literal(&mut self) -> Result<Token<'src>, Diagnostic<'src>> {
        let start = self.cursor;
        let mut value = 0_u64;
        let mut overflowed = false;

        self.consume_while(|byte| {
            if !byte.is_ascii_digit() {
                return false;
            }

            let digit = (byte - b'0') as u64;

            if !overflowed {
                match value
                    .checked_mul(10)
                    .and_then(|value| value.checked_add(digit))
                {
                    Some(next) => value = next,
                    None => overflowed = true,
                }
            }

            true
        });

        if overflowed {
            let lexeme = self.lexeme_from(start);
            let span = self.span_from(start);

            return Err(Diagnostic::Lex(LexError::IntOverflow { lexeme, span }));
        }

        Ok(self.make_token(TokenKind::IntLiteral(value), start))
    }

    fn scan_identifier(&mut self, uppercase: bool) -> Token<'src> {
        let start = self.cursor;

        self.consume_while(|byte| byte.is_ascii_alphanumeric() || byte == b'_');

        let lexeme = self.lexeme_from(start);

        let kind = if uppercase {
            TokenKind::UpperIdent(lexeme)
        } else {
            self.resolve_lower_identifier(lexeme)
        };

        self.make_token(kind, start)
    }

    fn resolve_lower_identifier(&self, lexeme: &'src [u8]) -> TokenKind<'src> {
        KEYWORDS
            .iter()
            .find_map(|(keyword, kind)| (*keyword == lexeme).then_some(*kind))
            .unwrap_or(TokenKind::LowerIdent(lexeme))
    }

    fn scan_operator(&mut self) -> Result<Token<'src>, Diagnostic<'src>> {
        let start = self.cursor;

        if let Some((len, kind)) = self.match_operator() {
            self.cursor += len;
            return Ok(self.make_token(kind, start));
        }

        let byte = self.advance().expect("scanner is not at EOF");
        let span = self.span_from(start);

        let error = if byte.is_ascii() {
            LexError::UnexpectedByte { byte, span }
        } else {
            LexError::NonAsciiByte { byte, span }
        };

        Err(Diagnostic::Lex(error))
    }

    fn match_operator(&self) -> Option<(usize, TokenKind<'src>)> {
        let remaining = &self.src[self.cursor..];

        LOOKAHEAD
            .iter()
            .filter_map(|(bytes, kind)| {
                let len = bytes.len();

                remaining.starts_with(*bytes).then_some((len, *kind))
            })
            .max_by_key(|(len, _)| *len)
    }
}
