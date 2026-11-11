use crate::span::Span;
use crate::typedexprkind::TypedExprKind;
use crate::typeid::TypeId;

pub(crate) struct TypedExpr {
    kind: TypedExprKind,
    span: Span,
    ty: TypeId,
}

impl TypedExpr {
    pub(crate) fn new(kind: TypedExprKind, span: Span, ty: TypeId) -> Self {
        Self { kind, span, ty }
    }
}
