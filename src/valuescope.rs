use crate::value::Value;
use std::collections::HashMap;

pub(crate) struct ValueScope {
    scopes: Vec<HashMap<String, Value>>,
}

impl ValueScope {
    pub(crate) fn new() -> Self {
        Self {
            scopes: vec![HashMap::new()],
        }
    }

    pub(crate) fn push(&mut self) {
        self.scopes.push(HashMap::new());
    }

    pub(crate) fn pop(&mut self) {
        self.scopes.pop();
    }

    pub(crate) fn declare(&mut self, name: String, value: Value) {
        self.scopes
            .last_mut()
            .expect("at least one scope always exists")
            .insert(name, value);
    }

    pub(crate) fn lookup(&self, name: &str) -> Value {
        self.scopes
            .iter()
            .rev()
            .find_map(|scope| scope.get(name))
            .copied()
            .expect("typechecker guarantees variable exists")
    }

    pub(crate) fn set(&mut self, name: &str, value: Value) {
        for scope in self.scopes.iter_mut().rev() {
            if let Some(slot) = scope.get_mut(name) {
                *slot = value;
                return;
            }
        }
        unreachable!("typechecker guarantees variable exists for assignment")
    }
}
