use crate::runtimeerror::RuntimeError;

#[derive(Debug, Clone, Copy)]
pub(crate) enum Signal {
    Break,
    Continue,
    Error(RuntimeError),
}

impl From<RuntimeError> for Signal {
    fn from(e: RuntimeError) -> Self {
        Signal::Error(e)
    }
}
