use crate::varinfo::VarInfo;
use std::collections::HashMap;

pub(crate) struct VarScope {
    scopes: Vec<HashMap<String, VarInfo>>,
}

impl VarScope {
    pub(crate) fn new() -> Self {
        Self {
            scopes: vec![HashMap::new()], // always at least one (top-level) scope
        }
    }

    pub(crate) fn push(&mut self) {
        self.scopes.push(HashMap::new());
    }

    pub(crate) fn pop(&mut self) {
        self.scopes.pop();
    }

    pub(crate) fn declare(&mut self, name: String, info: VarInfo) -> Result<(), ()> {
        let current = self
            .scopes
            .last_mut()
            .expect("at least one scope always exists");
        if current.contains_key(&name) {
            return Err(());
        }
        current.insert(name, info);
        Ok(())
    }

    pub(crate) fn lookup(&self, name: &str) -> Option<&VarInfo> {
        self.scopes.iter().rev().find_map(|scope| scope.get(name))
    }
}
