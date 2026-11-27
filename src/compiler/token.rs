//! Tokens de la sintaxis de expresiones propuesta para WTZ.
//!
//! El código fuente debe ser ASCII. El futuro lexer deberá rechazar cualquier
//! byte no ASCII antes de producir tokens; usar `&[u8]` no lo garantiza por sí solo.

/// Intervalo de bytes en el código fuente: inicio incluido y fin excluido.
///
/// En código ASCII, cada byte corresponde a un carácter. Para el final del archivo,
/// ambos extremos corresponden a la longitud del código fuente.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Span {
    pub start: usize,
    pub end: usize,
}

/// Elemento léxico junto con su ubicación en el archivo fuente.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Token<'src> {
    pub kind: TokenKind<'src>,
    pub span: Span,
}

/// Clases de elementos que reconocerá el lexer.
///
/// Los comentarios y espacios se omiten, pero los saltos de línea se conservan
/// para delimitar sentencias. El signo negativo es un token `Minus` separado.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TokenKind<'src> {
    Identifier(&'src [u8]),
    /// Dígitos originales, sin convertir aún a INT ni comprobar su rango.
    IntegerLiteral(&'src [u8]),
    /// Representación original, sin convertir aún a FLOAT.
    FloatLiteral(&'src [u8]),
    /// Bytes originales entre comillas, con escapes aún sin decodificar.
    StringLiteral(&'src [u8]),

    Let,
    Set,
    Print,
    Read,
    If,
    Else,
    End,
    While,
    Func,
    Return,
    True,
    False,

    Int,
    Float,
    Bool,
    String,

    Plus,
    Minus,
    Star,
    Slash,
    Percent,
    Equal,
    EqualEqual,
    BangEqual,
    Less,
    LessEqual,
    Greater,
    GreaterEqual,
    LeftParen,
    RightParen,
    Colon,
    Comma,
    Newline,
    Eof,
}
