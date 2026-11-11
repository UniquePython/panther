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

    pub(crate) fn join(&self, other: Span) -> Self {
        debug_assert!(self.start <= other.start);
        debug_assert!(other.end >= self.end);
        Self {
            start: self.start,
            end: other.end,
        }
    }
}
