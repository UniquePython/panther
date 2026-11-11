use crate::diagnostic::Diagnostic;
use crate::expr::Expr;
use crate::exprkind::ExprKind;
use crate::ops::{AssignOp, BinaryOp, UnaryOp};
use crate::typedexpr::TypedExpr;
use crate::typedexprkind::TypedExprKind;
use crate::typeerror::TypeError;
use crate::typeid::TypeId;
use crate::typescope::TypeScope;
use crate::varscope::VarScope;

pub(crate) struct TypeChecker {
    type_scope: TypeScope,
    var_scope: VarScope,
    int_ty: TypeId,
    bool_ty: TypeId,
    unit_ty: TypeId,
    never_ty: TypeId,
}

impl TypeChecker {
    fn types_compatible(&self, expected: TypeId, actual: TypeId) -> bool {
        actual == self.never_ty || expected == actual
    }

    fn expect_both(
        &self,
        expected: TypeId,
        left: &TypedExpr,
        right: &TypedExpr,
    ) -> Result<(), Diagnostic<'_>> {
        if !self.types_compatible(expected, left.ty()) {
            return Err(Diagnostic::Type(TypeError::TypeMismatch {
                expected,
                found: left.ty(),
                span: left.span(),
            }));
        }
        if !self.types_compatible(expected, right.ty()) {
            return Err(Diagnostic::Type(TypeError::TypeMismatch {
                expected,
                found: right.ty(),
                span: right.span(),
            }));
        }
        Ok(())
    }

    fn check_expr(&mut self, expr: Expr) -> Result<TypedExpr, Diagnostic<'_>> {
        match expr.kind {
            ExprKind::IntLiteral(v) => Ok(TypedExpr::new(
                TypedExprKind::IntLiteral(v),
                expr.span,
                self.int_ty,
            )),
            ExprKind::BoolLiteral(v) => Ok(TypedExpr::new(
                TypedExprKind::BoolLiteral(v),
                expr.span,
                self.bool_ty,
            )),
            ExprKind::UnitLiteral => Ok(TypedExpr::new(
                TypedExprKind::UnitLiteral,
                expr.span,
                self.unit_ty,
            )),
            ExprKind::Ident(name) => match self.var_scope.lookup(&name) {
                Some(info) => Ok(TypedExpr::new(
                    TypedExprKind::Ident(name),
                    expr.span,
                    info.ty(),
                )),
                None => Err(Diagnostic::Type(TypeError::UndefinedVariable {
                    name,
                    span: expr.span,
                })),
            },
            ExprKind::Unary { op, operand } => {
                let typed_operand = self.check_expr(*operand)?;

                let expected = match op {
                    UnaryOp::Neg => self.int_ty,
                    UnaryOp::Not => self.bool_ty,
                };

                if !self.types_compatible(expected, typed_operand.ty()) {
                    return Err(Diagnostic::Type(TypeError::TypeMismatch {
                        expected,
                        found: typed_operand.ty(),
                        span: typed_operand.span(),
                    }));
                }

                Ok(TypedExpr::new(
                    TypedExprKind::Unary {
                        op,
                        operand: Box::new(typed_operand),
                    },
                    expr.span,
                    expected,
                ))
            }
            ExprKind::Binary { op, left, right } => {
                let typed_left = self.check_expr(*left)?;
                let typed_right = self.check_expr(*right)?;

                let result_ty = match op {
                    BinaryOp::Add
                    | BinaryOp::Sub
                    | BinaryOp::Mul
                    | BinaryOp::Div
                    | BinaryOp::Mod => {
                        self.expect_both(self.int_ty, &typed_left, &typed_right)?;
                        self.int_ty
                    }
                    BinaryOp::Lt => {
                        self.expect_both(self.int_ty, &typed_left, &typed_right)?;
                        self.bool_ty
                    }
                    BinaryOp::Eq => {
                        // operands must match EACH OTHER, not one fixed type
                        if !self.types_compatible(typed_left.ty(), typed_right.ty())
                            && !self.types_compatible(typed_right.ty(), typed_left.ty())
                        {
                            return Err(Diagnostic::Type(TypeError::TypeMismatch {
                                expected: typed_left.ty(),
                                found: typed_right.ty(),
                                span: typed_right.span(),
                            }));
                        }
                        self.bool_ty
                    }
                    BinaryOp::And | BinaryOp::Or => {
                        self.expect_both(self.bool_ty, &typed_left, &typed_right)?;
                        self.bool_ty
                    }
                };

                Ok(TypedExpr::new(
                    TypedExprKind::Binary {
                        op,
                        left: Box::new(typed_left),
                        right: Box::new(typed_right),
                    },
                    expr.span,
                    result_ty,
                ))
            }
        }
    }
}
