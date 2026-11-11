use crate::diagnostic::Diagnostic;
use crate::expr::Expr;
use crate::exprkind::ExprKind;
use crate::mutability::Mutability;
use crate::ops::{AssignOp, BinaryOp, UnaryOp};
use crate::parseerror::ParseError;
use crate::span::Span;
use crate::token::Token;
use crate::tokenkind::TokenKind;

fn binary(op: BinaryOp, left: Expr, right: Expr) -> ExprKind {
    ExprKind::Binary {
        op,
        left: Box::new(left),
        right: Box::new(right),
    }
}

fn not(inner: ExprKind, span: Span) -> ExprKind {
    ExprKind::Unary {
        op: UnaryOp::Not,
        operand: Box::new(Expr::new(inner, span)),
    }
}

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

    fn expect_ident(
        &mut self,
        description: &'static str,
        is_valid: impl FnOnce(&TokenKind) -> bool,
    ) -> Result<(String, Span), Diagnostic<'src>> {
        let token = self.peek();

        if is_valid(&token.kind()) {
            let name = match token.kind() {
                TokenKind::LowerIdent(name) | TokenKind::UpperIdent(name) => {
                    String::from_utf8(name.to_vec()).unwrap()
                }
                _ => unreachable!(),
            };

            self.advance();
            Ok((name, token.span()))
        } else {
            Err(Diagnostic::Parse(ParseError::UnexpectedToken {
                expected: description,
                found: token.kind(),
                span: token.span(),
            }))
        }
    }

    fn expect_lower_ident(
        &mut self,
        description: &'static str,
    ) -> Result<(String, Span), Diagnostic<'src>> {
        self.expect_ident(description, |kind| matches!(kind, TokenKind::LowerIdent(_)))
    }

    fn expect_upper_ident(
        &mut self,
        description: &'static str,
    ) -> Result<(String, Span), Diagnostic<'src>> {
        self.expect_ident(description, |kind| matches!(kind, TokenKind::UpperIdent(_)))
    }

    fn parse_unary(&mut self) -> Result<Expr, Diagnostic<'src>> {
        let token = self.peek();

        let op = match token.kind() {
            TokenKind::Bang => UnaryOp::Not,
            TokenKind::Minus => UnaryOp::Neg,
            _ => return self.parse_primary(),
        };

        self.advance();
        let operand = self.parse_unary()?;
        let span = token.span().join(operand.span);
        Ok(Expr::new(
            ExprKind::Unary {
                op,
                operand: Box::new(operand),
            },
            span,
        ))
    }

    fn parse_multiplicative(&mut self) -> Result<Expr, Diagnostic<'src>> {
        let mut left = self.parse_unary()?;

        loop {
            let op = match self.peek().kind() {
                TokenKind::Star => BinaryOp::Mul,
                TokenKind::Slash => BinaryOp::Div,
                TokenKind::Percent => BinaryOp::Mod,
                _ => break,
            };
            self.advance();
            let right = self.parse_unary()?;
            let span = left.span.join(right.span);
            left = Expr::new(
                ExprKind::Binary {
                    op,
                    left: Box::new(left),
                    right: Box::new(right),
                },
                span,
            );
        }

        Ok(left)
    }

    fn parse_additive(&mut self) -> Result<Expr, Diagnostic<'src>> {
        let mut left = self.parse_multiplicative()?;

        loop {
            let op = match self.peek().kind() {
                TokenKind::Plus => BinaryOp::Add,
                TokenKind::Minus => BinaryOp::Sub,
                _ => break,
            };
            self.advance();
            let right = self.parse_multiplicative()?;
            let span = left.span.join(right.span);
            left = Expr::new(
                ExprKind::Binary {
                    op,
                    left: Box::new(left),
                    right: Box::new(right),
                },
                span,
            );
        }

        Ok(left)
    }

    fn parse_comparison(&mut self) -> Result<Expr, Diagnostic<'src>> {
        let left = self.parse_additive()?;

        let op_kind = self.peek().kind();
        let is_comparison = matches!(
            op_kind,
            TokenKind::Lt
                | TokenKind::EqEq
                | TokenKind::Gt
                | TokenKind::Le
                | TokenKind::Ge
                | TokenKind::BangEq
                | TokenKind::BangLt
                | TokenKind::BangGt
                | TokenKind::BangLe
                | TokenKind::BangGe
        );

        if !is_comparison {
            return Ok(left);
        }

        self.advance();
        let right = self.parse_additive()?;
        let span = left.span.join(right.span);

        let kind = match op_kind {
            TokenKind::Lt => binary(BinaryOp::Lt, left, right),
            TokenKind::EqEq => binary(BinaryOp::Eq, left, right),
            TokenKind::Gt => binary(BinaryOp::Lt, right, left), // swapped
            TokenKind::Le => not(binary(BinaryOp::Lt, right, left), span), // !(b < a)
            TokenKind::Ge => not(binary(BinaryOp::Lt, left, right), span), // !(a < b)
            TokenKind::BangEq => not(binary(BinaryOp::Eq, left, right), span),
            TokenKind::BangLt => not(binary(BinaryOp::Lt, left, right), span), // same as >=
            TokenKind::BangGt => not(binary(BinaryOp::Lt, right, left), span), // same as <=
            TokenKind::BangLe => binary(BinaryOp::Lt, right, left),            // same as >
            TokenKind::BangGe => binary(BinaryOp::Lt, left, right),            // same as <
            _ => unreachable!(),
        };

        Ok(Expr::new(kind, span))
    }

    fn parse_and(&mut self) -> Result<Expr, Diagnostic<'src>> {
        let mut left = self.parse_comparison()?;

        while self.check(TokenKind::AmpAmp) {
            self.advance();
            let right = self.parse_comparison()?;
            let span = left.span.join(right.span);
            left = Expr::new(
                ExprKind::Binary {
                    op: BinaryOp::And,
                    left: Box::new(left),
                    right: Box::new(right),
                },
                span,
            );
        }

        Ok(left)
    }

    fn parse_or(&mut self) -> Result<Expr, Diagnostic<'src>> {
        let mut left = self.parse_and()?;

        while self.check(TokenKind::PipePipe) {
            self.advance();
            let right = self.parse_and()?;
            let span = left.span.join(right.span);
            left = Expr::new(
                ExprKind::Binary {
                    op: BinaryOp::Or,
                    left: Box::new(left),
                    right: Box::new(right),
                },
                span,
            );
        }

        Ok(left)
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
        let start = self.expect(TokenKind::LBrace, "'{'")?.span();

        let mut exprs = Vec::new();
        let mut trailing = None;

        while !self.check(TokenKind::RBrace) {
            let expr = self.parse_expression()?;

            if self.check(TokenKind::Semi) {
                self.advance();
                exprs.push(expr);
            } else if self.check(TokenKind::RBrace) {
                trailing = Some(Box::new(expr));
                break;
            } else {
                return Err(Diagnostic::Parse(ParseError::UnexpectedToken {
                    expected: "';' or '}'",
                    found: self.peek().kind(),
                    span: self.peek().span(),
                }));
            }
        }

        let end = self.expect(TokenKind::RBrace, "'}'")?.span();
        let span = start.join(end);

        Ok(Expr::new(ExprKind::Block { exprs, trailing }, span))
    }

    fn parse_decl(&mut self) -> Result<Expr, Diagnostic<'src>> {
        let start = self.expect(TokenKind::Decl, "'decl'")?.span();

        let mutability = match self.peek().kind() {
            TokenKind::Mut => {
                self.advance();
                Mutability::Mut
            }
            TokenKind::Const => {
                self.advance();
                Mutability::Const
            }
            _ => {
                return Err(Diagnostic::Parse(ParseError::UnexpectedToken {
                    expected: "'mut' or 'const'",
                    found: self.peek().kind(),
                    span: self.peek().span(),
                }));
            }
        };

        let (name, _) = self.expect_lower_ident("a variable name")?;
        self.expect(TokenKind::Colon, "':'")?;
        let (ty, _) = self.expect_upper_ident("a type name")?;
        self.expect(TokenKind::Eq, "'='")?;
        let value = self.parse_expression()?;

        let span = start.join(value.span);
        Ok(Expr::new(
            ExprKind::Decl {
                mutability,
                name,
                ty,
                value: Box::new(value),
            },
            span,
        ))
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
