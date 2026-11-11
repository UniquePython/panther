use crate::exprkind::ExprKind;
use crate::span::Span;

pub(crate) struct Expr {
    pub(crate) kind: ExprKind,
    pub(crate) span: Span,
}

impl Expr {
    pub(crate) fn new(kind: ExprKind, span: Span) -> Self {
        Self { kind, span }
    }
}
