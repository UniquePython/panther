use crate::span::Span;
use crate::typeid::TypeId;

pub(crate) enum TypeError {
    UndefinedVariable {
        name: String,
        span: Span,
    },
    TypeMismatch {
        expected: TypeId,
        found: TypeId,
        span: Span,
    },
    UnknownType {
        name: String,
        span: Span,
    },
    DuplicateVariable {
        name: String,
        span: Span,
    },
    AssignToConst {
        name: String,
        span: Span,
    },
    BranchMismatch {
        then_ty: TypeId,
        else_ty: TypeId,
        then_span: Span,
        else_span: Span,
    },
    BreakOutsideLoop {
        span: Span,
    },
    ContinueOutsideLoop {
        span: Span,
    },
}
