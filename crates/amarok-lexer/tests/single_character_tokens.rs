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
        let actual = tokenize(source);
        let want = vec![expected, Token::EndOfFile];
        assert_eq!(actual, want, "input {source:?} produced the wrong tokens");
    }
}
