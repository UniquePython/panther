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

    pub(crate) fn kind(&self) -> &TypedExprKind {
        &self.kind
    }

    pub(crate) fn span(&self) -> Span {
        self.span
    }

    pub(crate) fn ty(&self) -> TypeId {
        self.ty
    }
}
