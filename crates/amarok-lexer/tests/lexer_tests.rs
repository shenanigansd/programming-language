use amarok_lexer::lexer::Lexer;
use amarok_lexer::token::TokenType;

fn token_types(source: &str) -> Vec<TokenType> {
    Lexer::new(source)
        .scan_tokens()
        .into_iter()
        .map(|token| token.token_type)
        .collect()
}

fn token_lexemes(source: &str) -> Vec<String> {
    Lexer::new(source)
        .scan_tokens()
        .into_iter()
        .map(|token| token.lexeme)
        .collect()
}

#[test]
fn scans_end_of_file_token() {
    let types = token_types("");
    assert_eq!(types, vec![TokenType::EndOfFile]);
}

#[test]
fn scans_single_character_tokens() {
    let types = token_types("(){},.-+;*/");

    assert_eq!(
        types,
        vec![
            TokenType::LeftParenthesis,
            TokenType::RightParenthesis,
            TokenType::LeftBrace,
            TokenType::RightBrace,
            TokenType::Comma,
            TokenType::Dot,
            TokenType::Minus,
            TokenType::Plus,
            TokenType::Semicolon,
            TokenType::Star,
            TokenType::Slash,
            TokenType::EndOfFile,
        ]
    );
}

#[test]
fn scans_one_or_two_character_tokens() {
    let types = token_types("! != = == < <= > >=");

    assert_eq!(
        types,
        vec![
            TokenType::Bang,
            TokenType::BangEqual,
            TokenType::Equal,
            TokenType::EqualEqual,
            TokenType::Less,
            TokenType::LessEqual,
            TokenType::Greater,
            TokenType::GreaterEqual,
            TokenType::EndOfFile,
        ]
    );
}

#[test]
fn skips_whitespace_and_tracks_lines() {
    let tokens = Lexer::new("(\n)\n").scan_tokens();

    assert_eq!(tokens[0].token_type, TokenType::LeftParenthesis);
    assert_eq!(tokens[0].line_number, 1);

    assert_eq!(tokens[1].token_type, TokenType::RightParenthesis);
    assert_eq!(tokens[1].line_number, 2);

    assert_eq!(tokens[2].token_type, TokenType::EndOfFile);
    assert_eq!(tokens[2].line_number, 3);
}

#[test]
fn skips_line_comments() {
    let types = token_types("// hello world\n+");

    assert_eq!(types, vec![TokenType::Plus, TokenType::EndOfFile]);
}

#[test]
fn scans_string_literal_token_and_lexeme_includes_quotes() {
    let tokens = Lexer::new("\"hello\"").scan_tokens();

    assert_eq!(tokens[0].token_type, TokenType::String);
    assert_eq!(tokens[0].lexeme, "\"hello\"");
    assert_eq!(tokens[1].token_type, TokenType::EndOfFile);
}

#[test]
fn scans_number_literal_integer() {
    let tokens = Lexer::new("123").scan_tokens();

    assert_eq!(tokens[0].token_type, TokenType::Number);
    assert_eq!(tokens[0].lexeme, "123");
    assert_eq!(tokens[1].token_type, TokenType::EndOfFile);
}

#[test]
fn scans_number_literal_fractional() {
    let tokens = Lexer::new("123.45").scan_tokens();

    assert_eq!(tokens[0].token_type, TokenType::Number);
    assert_eq!(tokens[0].lexeme, "123.45");
    assert_eq!(tokens[1].token_type, TokenType::EndOfFile);
}

#[test]
fn dot_after_number_is_not_part_of_number_without_following_digit() {
    let lexemes = token_lexemes("123.");

    // Should become: Number("123"), Dot("."), EOF
    assert_eq!(
        lexemes,
        vec!["123".to_string(), ".".to_string(), "".to_string()]
    );
    let types = token_types("123.");
    assert_eq!(
        types,
        vec![TokenType::Number, TokenType::Dot, TokenType::EndOfFile]
    );
}

#[test]
fn scans_identifier() {
    let tokens = Lexer::new("hello_world").scan_tokens();

    assert_eq!(tokens[0].token_type, TokenType::Identifier);
    assert_eq!(tokens[0].lexeme, "hello_world");
    assert_eq!(tokens[1].token_type, TokenType::EndOfFile);
}

#[test]
fn scans_identifier_then_number_as_separate_tokens() {
    let types = token_types("abc123");

    assert_eq!(types, vec![TokenType::Identifier, TokenType::EndOfFile]);

    // Lexeme includes the digits because identifiers allow alphanumeric after the first letter.
    let lexemes = token_lexemes("abc123");
    assert_eq!(lexemes, vec!["abc123".to_string(), "".to_string()]);
}

#[test]
fn scans_mixed_expression() {
    let types = token_types("x = 10 + 20;");

    assert_eq!(
        types,
        vec![
            TokenType::Identifier,
            TokenType::Equal,
            TokenType::Number,
            TokenType::Plus,
            TokenType::Number,
            TokenType::Semicolon,
            TokenType::EndOfFile,
        ]
    );
}

#[test]
fn converts_keywords_from_identifiers() {
    let tokens = Lexer::new("var print true false nil").scan_tokens();

    let types: Vec<TokenType> = tokens.into_iter().map(|token| token.token_type).collect();

    assert_eq!(
        types,
        vec![
            TokenType::Var,
            TokenType::Print,
            TokenType::True,
            TokenType::False,
            TokenType::Nil,
            TokenType::EndOfFile,
        ]
    );
}

#[test]
fn keeps_non_keywords_as_identifiers() {
    let tokens = Lexer::new("variable printer truly").scan_tokens();

    let types: Vec<TokenType> = tokens.into_iter().map(|token| token.token_type).collect();

    assert_eq!(
        types,
        vec![
            TokenType::Identifier,
            TokenType::Identifier,
            TokenType::Identifier,
            TokenType::EndOfFile,
        ]
    );
}

#[test]
fn keywords_are_case_sensitive() {
    let tokens = Lexer::new("Var PRINT True").scan_tokens();

    let types: Vec<TokenType> = tokens.into_iter().map(|token| token.token_type).collect();

    assert_eq!(
        types,
        vec![
            TokenType::Identifier,
            TokenType::Identifier,
            TokenType::Identifier,
            TokenType::EndOfFile,
        ]
    );
}
