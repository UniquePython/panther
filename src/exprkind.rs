use crate::expr::Expr;
use crate::mutability::Mutability;
use crate::ops::{AssignOp, BinaryOp, UnaryOp};

pub(crate) enum ExprKind {
    // literals / identifiers
    IntLiteral(i64),
    BoolLiteral(bool),
    UnitLiteral,
    Ident(String),

    // operators
    Unary {
        op: UnaryOp,
        operand: Box<Expr>,
    },
    Binary {
        op: BinaryOp,
        left: Box<Expr>,
        right: Box<Expr>,
    },

    // binding
    Decl {
        mutability: Mutability,
        name: String,
        ty: String,
        value: Box<Expr>,
    },

    // assignment
    Assign {
        target: String,
        op: AssignOp,
        value: Box<Expr>,
    },

    // blocks
    Block {
        exprs: Vec<Expr>,
        trailing: Option<Box<Expr>>,
    },

    // control flow
    If {
        condition: Box<Expr>,
        then_branch: Box<Expr>,
        else_branch: Box<Expr>,
    },
    When {
        condition: Box<Expr>,
        body: Box<Expr>,
    },
    While {
        condition: Box<Expr>,
        body: Box<Expr>,
    },
    Repeat {
        count: Box<Expr>,
        body: Box<Expr>,
    },
    Break,
    Continue,
}
