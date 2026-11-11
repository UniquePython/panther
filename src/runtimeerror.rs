use crate::ops::BinaryOp;
use crate::span::Span;

pub(crate) enum RuntimeError {
    NegativeRepeatCount {
        count: i64,
        span: Span,
    },
    IntegerOverflow {
        op: BinaryOp,
        left: i64,
        right: i64,
        span: Span,
    },
    DivisionByZero {
        span: Span,
    },
}
