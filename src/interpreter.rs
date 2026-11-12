use crate::flow::Flow;
use crate::ops::{AssignOp, BinaryOp, UnaryOp};
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
            TypedExprKind::Assign { target, op, value } => {
                let new_value = match self.eval_value(&value) {
                    Ok(v) => v,
                    Err(Signal::Break) => return Ok(Flow::Break),
                    Err(Signal::Continue) => return Ok(Flow::Continue),
                    Err(Signal::Error(e)) => return Err(e),
                };

                let result = match op {
                    AssignOp::Old => self.value_scope.lookup(&target),
                    AssignOp::New => new_value,
                };

                self.value_scope.set(&target, new_value);

                Ok(Flow::Value(result))
            }
            TypedExprKind::Block { exprs, trailing } => {
                self.value_scope.push();

                for e in exprs {
                    match self.eval(&e) {
                        Ok(Flow::Value(_)) => {} // discard, per block semantics — keep going
                        Ok(Flow::Break) => {
                            self.value_scope.pop();
                            return Ok(Flow::Break);
                        }
                        Ok(Flow::Continue) => {
                            self.value_scope.pop();
                            return Ok(Flow::Continue);
                        }
                        Err(e) => {
                            self.value_scope.pop();
                            return Err(e);
                        }
                    }
                }

                let result = match trailing {
                    Some(t) => match self.eval(&t) {
                        Ok(Flow::Value(v)) => Flow::Value(v),
                        Ok(Flow::Break) => {
                            self.value_scope.pop();
                            return Ok(Flow::Break);
                        }
                        Ok(Flow::Continue) => {
                            self.value_scope.pop();
                            return Ok(Flow::Continue);
                        }
                        Err(e) => {
                            self.value_scope.pop();
                            return Err(e);
                        }
                    },
                    None => Flow::Value(Value::Unit),
                };

                self.value_scope.pop();
                Ok(result)
            }
            TypedExprKind::If {
                condition,
                then_branch,
                else_branch,
            } => {
                let cond = match self.eval_value(&condition) {
                    Ok(v) => v,
                    Err(Signal::Break) => return Ok(Flow::Break),
                    Err(Signal::Continue) => return Ok(Flow::Continue),
                    Err(Signal::Error(e)) => return Err(e),
                };

                let Value::Bool(cond) = cond else {
                    unreachable!("typechecker guarantees Bool condition")
                };

                if cond {
                    self.eval(&then_branch)
                } else {
                    self.eval(&else_branch)
                }
            }
            TypedExprKind::When { condition, body } => {
                let cond = match self.eval_value(&condition) {
                    Ok(v) => v,
                    Err(Signal::Break) => return Ok(Flow::Break),
                    Err(Signal::Continue) => return Ok(Flow::Continue),
                    Err(Signal::Error(e)) => return Err(e),
                };

                let Value::Bool(cond) = cond else {
                    unreachable!("typechecker guarantees Bool condition")
                };

                if !cond {
                    return Ok(Flow::Value(Value::Unit));
                }

                match self.eval(&body)? {
                    Flow::Value(_) => Ok(Flow::Value(Value::Unit)), // discard body's value, When is always Unit
                    Flow::Break => Ok(Flow::Break),                 // propagate
                    Flow::Continue => Ok(Flow::Continue),           // propagate
                }
            }
            TypedExprKind::While { condition, body } => {
                loop {
                    let cond = match self.eval_value(&condition) {
                        Ok(v) => v,
                        Err(Signal::Break) => return Ok(Flow::Break), // condition itself broke out of an OUTER loop
                        Err(Signal::Continue) => return Ok(Flow::Continue), // same, for an outer loop
                        Err(Signal::Error(e)) => return Err(e),
                    };

                    let Value::Bool(cond) = cond else {
                        unreachable!("typechecker guarantees Bool condition")
                    };

                    if !cond {
                        break;
                    }

                    match self.eval(&body)? {
                        Flow::Value(_) => {}        // normal iteration, keep looping
                        Flow::Break => break,       // THIS loop's break — caught here, stop looping
                        Flow::Continue => continue, // THIS loop's continue — caught here, skip to next condition check
                    }
                }

                Ok(Flow::Value(Value::Unit)) // While always evaluates to Unit
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
