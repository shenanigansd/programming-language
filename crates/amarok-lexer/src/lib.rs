pub use amarok_diagnostics::{Diagnostic, SourcePosition};

/// The lexical category of a token, plus any payload it carries.
#[derive(Debug, PartialEq, Clone)]
pub enum TokenKind {
    LeftParenthesis,
    RightParenthesis,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Slash,
    Star,
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,
    StringLiteral(String),
    NumberLiteral(f64),
    Identifier(String),
    And,
    Or,
    Not,
    If,
    Else,
    While,
    For,
    Let,
    Fun,
    Return,
    True,
    False,
    Nil,
    EndOfFile,
}

/// A token: its kind, plus where in the source it began.
#[derive(Debug, PartialEq, Clone)]
pub struct Token {
    pub kind: TokenKind,
    pub position: SourcePosition,
}

struct Lexer {
    characters: Vec<char>,
    current: usize,
}

impl Lexer {
    fn new(source: &str) -> Self {
        Self {
            characters: source.chars().collect(),
            current: 0,
        }
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.characters.len()
    }

    fn advance(&mut self) -> char {
        let character = self.characters[self.current];
        self.current += 1;
        character
    }

    fn peek(&self) -> Option<char> {
        self.characters.get(self.current).copied()
    }

    fn peek_next(&self) -> Option<char> {
        self.characters.get(self.current + 1).copied()
    }

    /// If the next character equals `expected`, consume it and return true.
    /// Otherwise leave the cursor where it is and return false.
    fn match_next(&mut self, expected: char) -> bool {
        if self.peek() == Some(expected) {
            self.current += 1;
            true
        } else {
            false
        }
    }

    #[allow(clippy::too_many_lines)]
    fn run(mut self) -> (Vec<Token>, Vec<Diagnostic>) {
        let mut tokens = Vec::new();
        let mut diagnostics = Vec::new();
        while !self.is_at_end() {
            let start_position = SourcePosition {
                character_index: self.current,
            };
            let character = self.advance();
            let token = match character {
                '(' => TokenKind::LeftParenthesis,
                ')' => TokenKind::RightParenthesis,
                '{' => TokenKind::LeftBrace,
                '}' => TokenKind::RightBrace,
                ',' => TokenKind::Comma,
                '.' => TokenKind::Dot,
                '-' => TokenKind::Minus,
                '+' => TokenKind::Plus,
                ';' => TokenKind::Semicolon,
                '*' => TokenKind::Star,
                '!' => {
                    if self.match_next('=') {
                        TokenKind::BangEqual
                    } else {
                        TokenKind::Bang
                    }
                }
                '=' => {
                    if self.match_next('=') {
                        TokenKind::EqualEqual
                    } else {
                        TokenKind::Equal
                    }
                }
                '<' => {
                    if self.match_next('=') {
                        TokenKind::LessEqual
                    } else {
                        TokenKind::Less
                    }
                }
                '>' => {
                    if self.match_next('=') {
                        TokenKind::GreaterEqual
                    } else {
                        TokenKind::Greater
                    }
                }
                '/' => {
                    if self.match_next('/') {
                        while let Some(next_character) = self.peek() {
                            if next_character == '\n' {
                                break;
                            }
                            self.advance();
                        }
                        continue;
                    }
                    TokenKind::Slash
                }
                '"' => {
                    let mut content = String::new();
                    loop {
                        match self.peek() {
                            None => {
                                // Recovery: record the error, then emit a best-effort
                                // token containing whatever we managed to read.
                                diagnostics.push(Diagnostic::new(
                                    "unterminated string literal",
                                    start_position,
                                ));
                                break;
                            }
                            Some('"') => {
                                self.advance(); // consume the closing quote
                                break;
                            }
                            Some(other) => {
                                content.push(other);
                                self.advance();
                            }
                        }
                    }
                    TokenKind::StringLiteral(content)
                }
                digit @ '0'..='9' => {
                    let mut number_text = String::from(digit);
                    while let Some('0'..='9') = self.peek() {
                        number_text.push(self.advance());
                    }
                    if self.peek() == Some('.') && matches!(self.peek_next(), Some('0'..='9')) {
                        number_text.push(self.advance());
                        while let Some('0'..='9') = self.peek() {
                            number_text.push(self.advance());
                        }
                    }
                    let value: f64 = number_text.parse().expect(
                        "number_text contains only digits and at most one dot — must parse",
                    );
                    TokenKind::NumberLiteral(value)
                }
                first @ ('a'..='z' | 'A'..='Z' | '_') => {
                    let mut text = String::from(first);
                    while let Some(next) = self.peek() {
                        if !matches!(next, 'a'..='z' | 'A'..='Z' | '0'..='9' | '_') {
                            break;
                        }
                        text.push(self.advance());
                    }
                    match text.as_str() {
                        "and" => TokenKind::And,
                        "or" => TokenKind::Or,
                        "not" => TokenKind::Not,
                        "if" => TokenKind::If,
                        "else" => TokenKind::Else,
                        "while" => TokenKind::While,
                        "for" => TokenKind::For,
                        "let" => TokenKind::Let,
                        "fun" => TokenKind::Fun,
                        "return" => TokenKind::Return,
                        "true" => TokenKind::True,
                        "false" => TokenKind::False,
                        "nil" => TokenKind::Nil,
                        _ => TokenKind::Identifier(text),
                    }
                }
                ' ' | '\t' | '\r' | '\n' => continue,
                other => {
                    // Recovery: record the error and skip this one character.
                    diagnostics.push(Diagnostic::new(
                        format!("unexpected character: {other:?}"),
                        start_position,
                    ));
                    continue;
                }
            };
            tokens.push(Token {
                kind: token,
                position: start_position,
            });
        }
        tokens.push(Token {
            kind: TokenKind::EndOfFile,
            position: SourcePosition {
                character_index: self.current,
            },
        });
        (tokens, diagnostics)
    }
}

#[allow(clippy::must_use_candidate)]
pub fn tokenize(source: &str) -> (Vec<Token>, Vec<Diagnostic>) {
    Lexer::new(source).run()
}
