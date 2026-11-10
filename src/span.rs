#[derive(Debug, Clone, Copy)]
pub(crate) struct Span {
    start: usize,
    end: usize,
}

impl Span {
    pub(crate) fn new(start: usize, end: usize) -> Self {
        debug_assert!(start <= end);
        Self { start, end }
    }

    pub(crate) fn at(at: usize) -> Self {
        Self::new(at, at)
    }
}
