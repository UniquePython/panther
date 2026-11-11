use crate::value::Value;

#[derive(Debug, Clone, Copy)]
pub(crate) enum Flow {
    Value(Value),
    Break,
    Continue,
}
