#[derive(Debug, Clone, PartialEq)]
pub enum TokenKind {
    Identifier,
    Number,
    Let,
    Plus,
    Minus,
    Star,
    Slash,
    Equal,
    Semicolon,
    LeftParenthesis,
    RightParenthesis,
    EndOfFile,
}

#[derive(Debug, Clone)]
pub struct Token {
    pub kind: TokenKind,
    pub text: String,
    pub line_number: usize,
    pub column_number: usize,
}
