use crate::typeid::TypeId;
use crate::typescope::TypeScope;
use crate::varscope::VarScope;

pub(crate) struct TypeChecker {
    type_scope: TypeScope,
    var_scope: VarScope,
    int_ty: TypeId,
    bool_ty: TypeId,
    unit_ty: TypeId,
    never_ty: TypeId,
}
