use crate::flow::Flow;
use crate::ops::{BinaryOp, UnaryOp};
use crate::runtimeerror::RuntimeError;
use crate::signal::Signal;
use crate::typedexpr::TypedExpr;
use crate::typedexprkind::TypedExprKind;
use crate::value::Value;
use crate::valuescope::ValueScope;

struct Interpreter {
    value_scope: ValueScope,
}

impl Interpreter {
    pub(crate) fn new() -> Self {
        Self {
            value_scope: ValueScope::new(),
        }
    }

    fn eval(&mut self, expr: &TypedExpr) -> Result<Flow, RuntimeError> {
        match *expr.kind() {
            TypedExprKind::IntLiteral(v) => Ok(Flow::Value(Value::Int(v))),
            TypedExprKind::BoolLiteral(v) => Ok(Flow::Value(Value::Bool(v))),
            TypedExprKind::UnitLiteral => Ok(Flow::Value(Value::Unit)),
            TypedExprKind::Ident(name) => Ok(Flow::Value(self.value_scope.lookup(&name))),
            TypedExprKind::Unary { op, operand } => {
                let value = match self.eval_value(&operand) {
                    Ok(v) => v,
                    Err(Signal::Break) => return Ok(Flow::Break),
                    Err(Signal::Continue) => return Ok(Flow::Continue),
                    Err(Signal::Error(e)) => return Err(e),
                };

                match (op, value) {
                    (UnaryOp::Not, Value::Bool(b)) => Ok(Flow::Value(Value::Bool(!b))),
                    (UnaryOp::Neg, Value::Int(i)) => match i.checked_neg() {
                        Some(result) => Ok(Flow::Value(Value::Int(result))),
                        None => Err(RuntimeError::NegationOverflow {
                            value: i,
                            span: expr.span(),
                        }),
                    },
                    _ => unreachable!("typechecker guarantees operand type matches operator"),
                }
            }
            TypedExprKind::Binary { op, left, right } => match op {
                BinaryOp::Add | BinaryOp::Sub | BinaryOp::Mul | BinaryOp::Div | BinaryOp::Mod => {
                    let l = match self.eval_value(&left) {
                        Ok(v) => v,
                        Err(Signal::Break) => return Ok(Flow::Break),
                        Err(Signal::Continue) => return Ok(Flow::Continue),
                        Err(Signal::Error(e)) => return Err(e),
                    };
                    let r = match self.eval_value(&right) {
                        Ok(v) => v,
                        Err(Signal::Break) => return Ok(Flow::Break),
                        Err(Signal::Continue) => return Ok(Flow::Continue),
                        Err(Signal::Error(e)) => return Err(e),
                    };

                    let (Value::Int(l), Value::Int(r)) = (l, r) else {
                        unreachable!("typechecker guarantees Int operands for arithmetic")
                    };

                    let result = match op {
                        BinaryOp::Add => l.checked_add(r),
                        BinaryOp::Sub => l.checked_sub(r),
                        BinaryOp::Mul => l.checked_mul(r),
                        BinaryOp::Div => {
                            if r == 0 {
                                return Err(RuntimeError::DivisionByZero { span: expr.span() });
                            }
                            l.checked_div(r)
                        }
                        BinaryOp::Mod => {
                            if r == 0 {
                                return Err(RuntimeError::DivisionByZero { span: expr.span() });
                            }
                            l.checked_rem(r)
                        }
                        _ => unreachable!(),
                    };

                    match result {
                        Some(v) => Ok(Flow::Value(Value::Int(v))),
                        None => Err(RuntimeError::IntegerOverflow {
                            op,
                            left: l,
                            right: r,
                            span: expr.span(),
                        }),
                    }
                }
                BinaryOp::Lt => {
                    let l = match self.eval_value(&left) {
                        Ok(v) => v,
                        Err(Signal::Break) => return Ok(Flow::Break),
                        Err(Signal::Continue) => return Ok(Flow::Continue),
                        Err(Signal::Error(e)) => return Err(e),
                    };
                    let r = match self.eval_value(&right) {
                        Ok(v) => v,
                        Err(Signal::Break) => return Ok(Flow::Break),
                        Err(Signal::Continue) => return Ok(Flow::Continue),
                        Err(Signal::Error(e)) => return Err(e),
                    };
                    let (Value::Int(l), Value::Int(r)) = (l, r) else {
                        unreachable!("typechecker guarantees Int operands for Lt")
                    };
                    Ok(Flow::Value(Value::Bool(l < r)))
                }
                BinaryOp::Eq => {
                    let l = match self.eval_value(&left) {
                        Ok(v) => v,
                        Err(Signal::Break) => return Ok(Flow::Break),
                        Err(Signal::Continue) => return Ok(Flow::Continue),
                        Err(Signal::Error(e)) => return Err(e),
                    };
                    let r = match self.eval_value(&right) {
                        Ok(v) => v,
                        Err(Signal::Break) => return Ok(Flow::Break),
                        Err(Signal::Continue) => return Ok(Flow::Continue),
                        Err(Signal::Error(e)) => return Err(e),
                    };
                    let result = match (l, r) {
                        (Value::Int(a), Value::Int(b)) => a == b,
                        (Value::Bool(a), Value::Bool(b)) => a == b,
                        (Value::Unit, Value::Unit) => true,
                        _ => unreachable!("typechecker guarantees matching operand types for Eq"),
                    };
                    Ok(Flow::Value(Value::Bool(result)))
                }
                BinaryOp::And => {
                    let l = match self.eval_value(&left) {
                        Ok(v) => v,
                        Err(Signal::Break) => return Ok(Flow::Break),
                        Err(Signal::Continue) => return Ok(Flow::Continue),
                        Err(Signal::Error(e)) => return Err(e),
                    };
                    let Value::Bool(l) = l else {
                        unreachable!("typechecker guarantees Bool operand")
                    };

                    if !l {
                        return Ok(Flow::Value(Value::Bool(false))); // short-circuit: skip right entirely
                    }

                    let r = match self.eval_value(&right) {
                        Ok(v) => v,
                        Err(Signal::Break) => return Ok(Flow::Break),
                        Err(Signal::Continue) => return Ok(Flow::Continue),
                        Err(Signal::Error(e)) => return Err(e),
                    };
                    let Value::Bool(r) = r else {
                        unreachable!("typechecker guarantees Bool operand")
                    };
                    Ok(Flow::Value(Value::Bool(r)))
                }
                BinaryOp::Or => {
                    let l = match self.eval_value(&left) {
                        Ok(v) => v,
                        Err(Signal::Break) => return Ok(Flow::Break),
                        Err(Signal::Continue) => return Ok(Flow::Continue),
                        Err(Signal::Error(e)) => return Err(e),
                    };
                    let Value::Bool(l) = l else {
                        unreachable!("typechecker guarantees Bool operand")
                    };

                    if l {
                        return Ok(Flow::Value(Value::Bool(true))); // short-circuit: skip right entirely
                    }

                    let r = match self.eval_value(&right) {
                        Ok(v) => v,
                        Err(Signal::Break) => return Ok(Flow::Break),
                        Err(Signal::Continue) => return Ok(Flow::Continue),
                        Err(Signal::Error(e)) => return Err(e),
                    };
                    let Value::Bool(r) = r else {
                        unreachable!("typechecker guarantees Bool operand")
                    };
                    Ok(Flow::Value(Value::Bool(r)))
                }
            },
            TypedExprKind::Decl { name, value } => {
                let v = match self.eval_value(&value) {
                    Ok(v) => v,
                    Err(Signal::Break) => return Ok(Flow::Break),
                    Err(Signal::Continue) => return Ok(Flow::Continue),
                    Err(Signal::Error(e)) => return Err(e),
                };

                self.value_scope.declare(name.clone(), v);

                Ok(Flow::Value(v))
            }
        }
    }

    fn eval_value(&mut self, expr: &TypedExpr) -> Result<Value, Signal> {
        match self.eval(expr)? {
            Flow::Value(v) => Ok(v),
            Flow::Break => Err(Signal::Break),
            Flow::Continue => Err(Signal::Continue),
        }
    }
}
