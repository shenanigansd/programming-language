use amarok_lexer::Token;
use amarok_parser::parse;

#[test]
fn trailing_tokens_after_a_complete_expression_are_an_error() {
    // 1 2  — the 2 has no business being there
    let tokens = vec![
        Token::NumberLiteral(1.0),
        Token::NumberLiteral(2.0),
        Token::EndOfFile,
    ];
    assert!(parse(tokens).is_err());
}

#[test]
fn an_expression_that_uses_every_token_succeeds() {
    // 1 + 2 — consumes everything, lands cleanly on EndOfFile
    let tokens = vec![
        Token::NumberLiteral(1.0),
        Token::Plus,
        Token::NumberLiteral(2.0),
        Token::EndOfFile,
    ];
    assert!(parse(tokens).is_ok());
}
