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
}
