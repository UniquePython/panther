pub(crate) enum UnaryOp {
    Neg,
    Not,
}
pub(crate) enum BinaryOp {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    Lt,
    Eq,
    And,
    Or,
}

pub(crate) enum AssignOp {
    Old,
    New,
}
