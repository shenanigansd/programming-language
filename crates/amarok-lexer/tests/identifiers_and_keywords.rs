use amarok_lexer::{Token, tokenize};

#[test]
fn simple_identifier() {
    let tokens = tokenize("hello");
    assert_eq!(
        tokens,
        vec![Token::Identifier(String::from("hello")), Token::EndOfFile],
    );
}

#[test]
fn identifier_can_contain_digits_after_the_first_character() {
    let tokens = tokenize("var1");
    assert_eq!(
        tokens,
        vec![Token::Identifier(String::from("var1")), Token::EndOfFile],
    );
}

#[test]
fn identifier_can_contain_underscores() {
    let tokens = tokenize("my_variable");
    assert_eq!(
        tokens,
        vec![
            Token::Identifier(String::from("my_variable")),
            Token::EndOfFile
        ],
    );
}

#[test]
fn a_lone_underscore_is_an_identifier() {
    let tokens = tokenize("_");
    assert_eq!(
        tokens,
        vec![Token::Identifier(String::from("_")), Token::EndOfFile],
    );
}

#[test]
fn identifier_cannot_start_with_a_digit() {
    // "1foo" should be a number followed by an identifier, not one token.
    let tokens = tokenize("1foo");
    assert_eq!(
        tokens,
        vec![
            Token::NumberLiteral(1.0),
            Token::Identifier(String::from("foo")),
            Token::EndOfFile,
        ],
    );
}

#[test]
fn all_keywords_produce_their_keyword_token() {
    let cases = [
        ("and", Token::And),
        ("or", Token::Or),
        ("not", Token::Not),
        ("if", Token::If),
        ("else", Token::Else),
        ("while", Token::While),
        ("for", Token::For),
        ("let", Token::Let),
        ("fun", Token::Fun),
        ("return", Token::Return),
        ("true", Token::True),
        ("false", Token::False),
        ("nil", Token::Nil),
    ];
    for (source, expected) in cases {
        let tokens = tokenize(source);
        assert_eq!(
            tokens,
            vec![expected, Token::EndOfFile],
            "keyword {source:?} did not produce the expected token",
        );
    }
}

#[test]
fn keyword_prefix_followed_by_other_letters_is_an_identifier() {
    // "ifx" starts with "if" but it should be the single identifier "ifx",
    // not Token::If followed by Token::Identifier("x").
    let tokens = tokenize("ifx");
    assert_eq!(
        tokens,
        vec![Token::Identifier(String::from("ifx")), Token::EndOfFile],
    );
}

#[test]
fn identifier_containing_a_keyword_as_a_substring_is_one_identifier() {
    let tokens = tokenize("x_if_y");
    assert_eq!(
        tokens,
        vec![Token::Identifier(String::from("x_if_y")), Token::EndOfFile],
    );
}
