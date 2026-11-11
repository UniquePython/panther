use crate::mutability::Mutability;
use crate::typeid::TypeId;

#[derive(Debug, Clone, Copy)]
pub(crate) struct VarInfo {
    ty: TypeId,
    mutability: Mutability,
}

impl VarInfo {
    pub(crate) fn new(ty: TypeId, mutability: Mutability) -> Self {
        Self { ty, mutability }
    }
}
