#[derive(Debug, Clone, Copy)]
pub(crate) enum Value {
    Int(i64),
    Bool(bool),
    Unit,
}
