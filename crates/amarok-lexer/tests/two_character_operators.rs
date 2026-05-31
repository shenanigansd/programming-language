use amarok_lexer::{Token, tokenize};

#[test]
fn two_character_comparison_operators_produce_their_token() {
    let cases = [
        (">=", Token::GreaterEqual),
        ("<=", Token::LessEqual),
        ("==", Token::EqualEqual),
        ("!=", Token::BangEqual),
    ];
    for (source, expected) in cases {
        let (actual, diagnostics) = tokenize(source);
        let want = vec![expected, Token::EndOfFile];
        assert_eq!(actual, want, "input {source:?} produced the wrong tokens");
        assert!(diagnostics.is_empty());
    }
}

#[test]
fn one_character_comparison_operators_alone() {
    let cases = [
        (">", Token::Greater),
        ("<", Token::Less),
        ("=", Token::Equal),
        ("!", Token::Bang),
    ];
    for (source, expected) in cases {
        let (actual, diagnostics) = tokenize(source);
        let want = vec![expected, Token::EndOfFile];
        assert_eq!(actual, want, "input {source:?} produced the wrong tokens");
        assert!(diagnostics.is_empty());
    }
}

#[test]
fn greater_followed_by_non_equal_does_not_become_greater_equal() {
    // The exact case you flagged: ">(" should be Greater followed by
    // LeftParenthesis, not "swallow the paren as part of some > = thing."
    let (tokens, diagnostics) = tokenize(">(");
    assert_eq!(
        tokens,
        vec![Token::Greater, Token::LeftParenthesis, Token::EndOfFile],
    );
    assert!(diagnostics.is_empty());
}
