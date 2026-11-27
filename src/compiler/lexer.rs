
pub struct Lexer<'src> {
    source: &'src [u8],
    position: usize,
}
