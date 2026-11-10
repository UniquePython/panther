#[derive(Debug, Clone, Copy)]
pub(crate) struct Span {
    start: usize,
    end: usize,
}

impl Span {
    pub(crate) fn new(start: usize, end: usize) -> Option<Self> {
        if end < start {
            return None;
        }

        Some(Self { start, end })
    }

    pub(crate) fn at(at: usize) -> Self {
        Self { start: at, end: at }
    }
}
