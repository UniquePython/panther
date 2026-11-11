use crate::typedef::TypeDef;
use crate::typeid::TypeId;
use std::collections::HashMap;

pub(crate) struct TypeScope {
    names: HashMap<String, TypeId>,
    defs: Vec<TypeDef>,
}

impl TypeScope {
    pub(crate) fn new() -> Self {
        Self {
            names: HashMap::new(),
            defs: Vec::new(),
        }
    }

    pub(crate) fn insert(&mut self, name: String, def: TypeDef) -> Option<TypeId> {
        if self.names.contains_key(&name) {
            return None;
        }
        let id = TypeId(self.defs.len());
        self.defs.push(def);
        self.names.insert(name, id);
        Some(id)
    }

    pub(crate) fn insert_anonymous(&mut self, def: TypeDef) -> TypeId {
        let id = TypeId(self.defs.len());
        self.defs.push(def);
        id
    }

    pub(crate) fn lookup(&self, name: &str) -> Option<TypeId> {
        self.names.get(name).copied()
    }

    pub(crate) fn def(&self, id: TypeId) -> &TypeDef {
        &self.defs[id.0]
    }
}
