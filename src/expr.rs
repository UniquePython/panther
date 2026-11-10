use crate::exprkind::ExprKind;
use crate::span::Span;

pub(crate) struct Expr {
    kind: ExprKind,
    span: Span,
}
