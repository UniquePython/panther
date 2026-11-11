use crate::flow::Flow;
use crate::runtimeerror::RuntimeError;
use crate::typedexpr::TypedExpr;
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
        todo!()
    }
}
