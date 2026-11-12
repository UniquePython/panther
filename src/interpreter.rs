use crate::flow::Flow;
use crate::ops::UnaryOp;
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
