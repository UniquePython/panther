use crate::diagnostic::Diagnostic;
use crate::expr::Expr;
use crate::exprkind::ExprKind;
use crate::ops::AssignOp;
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

    fn parse_or(&mut self) -> Result<Expr, Diagnostic<'src>> {
        todo!()
    }

    fn parse_assignment(&mut self) -> Result<Expr, Diagnostic<'src>> {
        let expr = self.parse_or()?;

        if self.check(TokenKind::ColonEq) || self.check(TokenKind::EqColon) {
            // `expr` must actually be a bare identifier for this to be valid
            let ExprKind::Ident(name) = expr.kind else {
                return Err(Diagnostic::Parse(ParseError::InvalidAssignmentTarget {
                    span: expr.span,
                }));
            };

            let op = if self.check(TokenKind::ColonEq) {
                AssignOp::Old
            } else {
                AssignOp::New
            };
            self.advance(); // consume := or =:

            let value = self.parse_assignment()?;
            let span = expr.span.join(value.span);
            return Ok(Expr::new(
                ExprKind::Assign {
                    target: name,
                    op,
                    value: Box::new(value),
                },
                span,
            ));
        }

        Ok(expr)
    }

    fn parse_expression(&mut self) -> Result<Expr, Diagnostic<'src>> {
        self.parse_assignment()
    }

    fn parse_grouped(&mut self) -> Result<Expr, Diagnostic<'src>> {
        self.expect(TokenKind::LParen, "'('")?;
        let inner = self.parse_expression()?;
        self.expect(TokenKind::RParen, "')'")?;
        Ok(inner)
    }

    fn parse_block(&mut self) -> Result<Expr, Diagnostic<'src>> {
        todo!()
    }

    fn parse_decl(&mut self) -> Result<Expr, Diagnostic<'src>> {
        todo!()
    }

    fn parse_if(&mut self) -> Result<Expr, Diagnostic<'src>> {
        todo!()
    }

    fn parse_when(&mut self) -> Result<Expr, Diagnostic<'src>> {
        todo!()
    }

    fn parse_while(&mut self) -> Result<Expr, Diagnostic<'src>> {
        todo!()
    }

    fn parse_repeat(&mut self) -> Result<Expr, Diagnostic<'src>> {
        todo!()
    }

    fn parse_primary(&mut self) -> Result<Expr, Diagnostic<'src>> {
        let token = self.peek();
        let span = token.span();

        match token.kind() {
            TokenKind::IntLiteral(value) => {
                self.advance();
                Ok(Expr::new(ExprKind::IntLiteral(value), span))
            }
            TokenKind::BoolLiteral(value) => {
                self.advance();
                Ok(Expr::new(ExprKind::BoolLiteral(value), span))
            }
            TokenKind::UnitLiteral => {
                self.advance();
                Ok(Expr::new(ExprKind::UnitLiteral, span))
            }
            TokenKind::LowerIdent(name) => {
                self.advance();
                Ok(Expr::new(
                    ExprKind::Ident(String::from_utf8(name.to_vec()).unwrap()),
                    span,
                ))
            }
            TokenKind::LParen => self.parse_grouped(),
            TokenKind::LBrace => self.parse_block(),
            TokenKind::Decl => self.parse_decl(),
            TokenKind::If | TokenKind::Unless => self.parse_if(),
            TokenKind::When => self.parse_when(),
            TokenKind::While => self.parse_while(),
            TokenKind::Repeat => self.parse_repeat(),
            TokenKind::Break => {
                self.advance();
                Ok(Expr::new(ExprKind::Break, span))
            }
            TokenKind::Continue => {
                self.advance();
                Ok(Expr::new(ExprKind::Continue, span))
            }
            _ => Err(Diagnostic::Parse(ParseError::UnexpectedToken {
                expected: "an expression",
                found: token.kind(),
                span,
            })),
        }
    }
}
