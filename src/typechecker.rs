use crate::diagnostic::Diagnostic;
use crate::expr::Expr;
use crate::exprkind::ExprKind;
use crate::mutability::Mutability;
use crate::ops::{AssignOp, BinaryOp, UnaryOp};
use crate::typedexpr::TypedExpr;
use crate::typedexprkind::TypedExprKind;
use crate::typeerror::TypeError;
use crate::typeid::TypeId;
use crate::typescope::TypeScope;
use crate::varinfo::VarInfo;
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
            ExprKind::Decl {
                mutability,
                name,
                name_span,
                ty,
                ty_span,
                value,
            } => {
                let declared_ty = self.type_scope.lookup(&ty).ok_or_else(|| {
                    Diagnostic::Type(TypeError::UnknownType {
                        name: ty.clone(),
                        span: ty_span,
                    })
                })?;

                let typed_value = self.check_expr(*value)?;

                if !self.types_compatible(declared_ty, typed_value.ty()) {
                    return Err(Diagnostic::Type(TypeError::TypeMismatch {
                        expected: declared_ty,
                        found: typed_value.ty(),
                        span: typed_value.span(),
                    }));
                }

                let var_info = VarInfo::new(declared_ty, mutability);
                self.var_scope
                    .declare(name.clone(), var_info)
                    .map_err(|_| {
                        Diagnostic::Type(TypeError::DuplicateVariable {
                            name: name.clone(),
                            span: name_span,
                        })
                    })?;

                Ok(TypedExpr::new(
                    TypedExprKind::Decl {
                        mutability,
                        name,
                        value: Box::new(typed_value),
                    },
                    expr.span,
                    declared_ty,
                ))
            }
            ExprKind::Assign { target, op, value } => {
                let var_info = self
                    .var_scope
                    .lookup(&target)
                    .ok_or_else(|| TypeError::UndefinedVariable {
                        name: target.clone(),
                        span: expr.span,
                    })
                    .map_err(Diagnostic::Type)?;

                let target_ty = var_info.ty();
                let target_mutability = var_info.mutability();

                if target_mutability == Mutability::Const {
                    return Err(Diagnostic::Type(TypeError::AssignToConst {
                        name: target.clone(),
                        span: expr.span,
                    }));
                }

                let typed_value = self.check_expr(*value)?;

                if !self.types_compatible(target_ty, typed_value.ty()) {
                    return Err(Diagnostic::Type(TypeError::TypeMismatch {
                        expected: target_ty,
                        found: typed_value.ty(),
                        span: typed_value.span(),
                    }));
                }

                Ok(TypedExpr::new(
                    TypedExprKind::Assign {
                        target,
                        op,
                        value: Box::new(typed_value),
                    },
                    expr.span,
                    target_ty,
                ))
            }
            ExprKind::Assign {
                target,
                target_span,
                op,
                value,
            } => {
                let var_info = self
                    .var_scope
                    .lookup(&target)
                    .ok_or_else(|| TypeError::UndefinedVariable {
                        name: target.clone(),
                        span: target_span,
                    })
                    .map_err(Diagnostic::Type)?;

                let target_ty = var_info.ty();
                let target_mutability = var_info.mutability();

                if target_mutability == Mutability::Const {
                    return Err(Diagnostic::Type(TypeError::AssignToConst {
                        name: target.clone(),
                        span: target_span,
                    }));
                }

                let typed_value = self.check_expr(*value)?;

                if !self.types_compatible(target_ty, typed_value.ty()) {
                    return Err(Diagnostic::Type(TypeError::TypeMismatch {
                        expected: target_ty,
                        found: typed_value.ty(),
                        span: typed_value.span(),
                    }));
                }

                Ok(TypedExpr::new(
                    TypedExprKind::Assign {
                        target,
                        op,
                        value: Box::new(typed_value),
                    },
                    expr.span,
                    target_ty,
                ))
            }
            ExprKind::Block { exprs, trailing } => {
                self.var_scope.push();

                let mut typed_exprs = Vec::with_capacity(exprs.len());
                for e in exprs {
                    typed_exprs.push(self.check_expr(e)?);
                }

                let typed_trailing = match trailing {
                    Some(t) => Some(Box::new(self.check_expr(*t)?)),
                    None => None,
                };

                let block_ty = match &typed_trailing {
                    Some(t) => t.ty(),
                    None => self.unit_ty,
                };

                self.var_scope.pop();

                Ok(TypedExpr::new(
                    TypedExprKind::Block {
                        exprs: typed_exprs,
                        trailing: typed_trailing,
                    },
                    expr.span,
                    block_ty,
                ))
            }
            ExprKind::If {
                condition,
                then_branch,
                else_branch,
            } => {
                let typed_condition = self.check_expr(*condition)?;

                if !self.types_compatible(self.bool_ty, typed_condition.ty()) {
                    return Err(Diagnostic::Type(TypeError::TypeMismatch {
                        expected: self.bool_ty,
                        found: typed_condition.ty(),
                        span: typed_condition.span(),
                    }));
                }

                let typed_then = self.check_expr(*then_branch)?;
                let typed_else = self.check_expr(*else_branch)?;

                let then_ty = typed_then.ty();
                let else_ty = typed_else.ty();

                let result_ty = if then_ty == self.never_ty {
                    else_ty
                } else if else_ty == self.never_ty {
                    then_ty
                } else if then_ty == else_ty {
                    then_ty
                } else {
                    return Err(Diagnostic::Type(TypeError::BranchMismatch {
                        then_ty,
                        else_ty,
                        then_span: typed_then.span(),
                        else_span: typed_else.span(),
                    }));
                };

                Ok(TypedExpr::new(
                    TypedExprKind::If {
                        condition: Box::new(typed_condition),
                        then_branch: Box::new(typed_then),
                        else_branch: Box::new(typed_else),
                    },
                    expr.span,
                    result_ty,
                ))
            }
            ExprKind::When { condition, body } => {
                let typed_condition = self.check_expr(*condition)?;

                if !self.types_compatible(self.bool_ty, typed_condition.ty()) {
                    return Err(Diagnostic::Type(TypeError::TypeMismatch {
                        expected: self.bool_ty,
                        found: typed_condition.ty(),
                        span: typed_condition.span(),
                    }));
                }

                let typed_body = self.check_expr(*body)?;

                Ok(TypedExpr::new(
                    TypedExprKind::When {
                        condition: Box::new(typed_condition),
                        body: Box::new(typed_body),
                    },
                    expr.span,
                    self.unit_ty,
                ))
            }
            ExprKind::While { condition, body } => {
                let typed_condition = self.check_expr(*condition)?;
                if !self.types_compatible(self.bool_ty, typed_condition.ty()) {
                    return Err(Diagnostic::Type(TypeError::TypeMismatch {
                        expected: self.bool_ty,
                        found: typed_condition.ty(),
                        span: typed_condition.span(),
                    }));
                }
                let typed_body = self.check_expr(*body)?;
                Ok(TypedExpr::new(
                    TypedExprKind::While {
                        condition: Box::new(typed_condition),
                        body: Box::new(typed_body),
                    },
                    expr.span,
                    self.unit_ty,
                ))
            }
            ExprKind::Repeat { count, body } => {
                let typed_count = self.check_expr(*count)?;
                if !self.types_compatible(self.int_ty, typed_count.ty()) {
                    return Err(Diagnostic::Type(TypeError::TypeMismatch {
                        expected: self.int_ty,
                        found: typed_count.ty(),
                        span: typed_count.span(),
                    }));
                }
                let typed_body = self.check_expr(*body)?;
                Ok(TypedExpr::new(
                    TypedExprKind::Repeat {
                        count: Box::new(typed_count),
                        body: Box::new(typed_body),
                    },
                    expr.span,
                    self.unit_ty,
                ))
            }
            ExprKind::Break => Ok(TypedExpr::new(
                TypedExprKind::Break,
                expr.span,
                self.never_ty,
            )),
            ExprKind::Continue => Ok(TypedExpr::new(
                TypedExprKind::Continue,
                expr.span,
                self.never_ty,
            )),
        }
    }
}
