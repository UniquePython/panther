use crate::span::Span;

pub(crate) enum TypeError {
    UndefinedVariable { name: String, span: Span },
}
