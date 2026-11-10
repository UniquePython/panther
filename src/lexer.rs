pub(crate) struct Lexer<'src> {
    src: &'src [u8],
    cursor: usize,
}

impl<'src> Lexer<'src> {
    pub(crate) fn new(src: &'src str) -> Self {
        Self {
            src: src.as_bytes(),
            cursor: 0,
        }
    }
}
