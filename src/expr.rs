use crate::exprkind::ExprKind;
use crate::span::{self, Span};

pub(crate) struct Expr {
    kind: ExprKind,
    span: Span,
}

impl Expr {
    pub(crate) fn new(kind: ExprKind, span: Span) -> Self {
        Self { kind, span }
    }
}
