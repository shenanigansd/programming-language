use amarok_lexer::{Token, tokenize};

#[test]
fn each_single_character_punctuation_produces_its_token() {
    let cases = [
        ("(", Token::LeftParenthesis),
        (")", Token::RightParenthesis),
        ("{", Token::LeftBrace),
        ("}", Token::RightBrace),
        (",", Token::Comma),
        (".", Token::Dot),
        ("-", Token::Minus),
        ("+", Token::Plus),
        (";", Token::Semicolon),
        ("*", Token::Star),
    ];

    for (source, expected) in cases {
        let (tokens, diagnostics) = tokenize(source);
        assert_eq!(tokens, vec![expected, Token::EndOfFile], "input {source:?}");
        assert!(
            diagnostics.is_empty(),
            "input {source:?} produced unexpected diagnostics"
        );
    }
}
