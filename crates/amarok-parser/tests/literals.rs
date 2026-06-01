use amarok_lexer::Token;
use amarok_parser::parse;
use amarok_syntax::Expression;

#[test]
fn parses_a_number_literal() {
    let tokens = vec![Token::NumberLiteral(42.0), Token::EndOfFile];
    assert_eq!(parse(tokens), Ok(Expression::NumberLiteral(42.0)));
}

#[test]
fn parses_a_string_literal() {
    let tokens = vec![
        Token::StringLiteral(String::from("hello")),
        Token::EndOfFile,
    ];
    assert_eq!(
        parse(tokens),
        Ok(Expression::StringLiteral(String::from("hello"))),
    );
}

#[test]
fn parses_the_true_literal() {
    let tokens = vec![Token::True, Token::EndOfFile];
    assert_eq!(parse(tokens), Ok(Expression::BooleanLiteral(true)));
}

#[test]
fn parses_the_false_literal() {
    let tokens = vec![Token::False, Token::EndOfFile];
    assert_eq!(parse(tokens), Ok(Expression::BooleanLiteral(false)));
}

#[test]
fn parses_the_nil_literal() {
    let tokens = vec![Token::Nil, Token::EndOfFile];
    assert_eq!(parse(tokens), Ok(Expression::NilLiteral));
}

#[test]
fn an_empty_token_stream_is_an_error() {
    let tokens = vec![Token::EndOfFile];
    assert!(parse(tokens).is_err());
}
