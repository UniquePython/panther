use crate::ops::{AssignOp, BinaryOp, UnaryOp};
use crate::typedexpr::TypedExpr;

pub(crate) enum TypedExprKind {
    IntLiteral(i64),
    BoolLiteral(bool),
    UnitLiteral,
    Ident(String),

    Unary {
        op: UnaryOp,
        operand: Box<TypedExpr>,
    },
    Binary {
        op: BinaryOp,
        left: Box<TypedExpr>,
        right: Box<TypedExpr>,
    },

    Decl {
        name: String,
        value: Box<TypedExpr>,
    },

    Assign {
        target: String,
        op: AssignOp,
        value: Box<TypedExpr>,
    },

    Block {
        exprs: Vec<TypedExpr>,
        trailing: Option<Box<TypedExpr>>,
    },

    If {
        condition: Box<TypedExpr>,
        then_branch: Box<TypedExpr>,
        else_branch: Box<TypedExpr>,
    },
    When {
        condition: Box<TypedExpr>,
        body: Box<TypedExpr>,
    },
    While {
        condition: Box<TypedExpr>,
        body: Box<TypedExpr>,
    },
    Repeat {
        count: Box<TypedExpr>,
        body: Box<TypedExpr>,
    },
    Break,
    Continue,
}
