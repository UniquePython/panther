use crate::flow::Flow;
use crate::runtimeerror::RuntimeError;
use crate::typedexpr::TypedExpr;
use crate::typedexprkind::TypedExprKind;
use crate::value::Value;
use crate::valuescope::ValueScope;

struct Interpreter {
    value_scope: ValueScope,
}

impl Interpreter {
    fn new() -> Self {
        Self {
            value_scope: ValueScope::new(),
        }
    }

    fn eval(&mut self, expr: &TypedExpr) -> Result<Flow, RuntimeError> {
        match *expr.kind() {
            TypedExprKind::IntLiteral(v) => Ok(Flow::Value(Value::Int(v))),
            TypedExprKind::BoolLiteral(v) => Ok(Flow::Value(Value::Bool(v))),
            TypedExprKind::UnitLiteral => Ok(Flow::Value(Value::Unit)),
        }
    }
}
