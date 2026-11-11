pub(crate) struct TypeId(usize);

impl TypeId {
    pub(crate) fn new(id: usize) -> Self {
        Self(id)
    }
}
