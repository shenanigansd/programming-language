use amarok_lexer::{Token, tokenize};

#[test]
fn single_slash_is_division() {
    let tokens = tokenize("/");
    assert_eq!(tokens, vec![Token::Slash, Token::EndOfFile]);
}

#[test]
fn line_comment_until_end_of_input_produces_no_tokens() {
    let tokens = tokenize("// this is a comment");
    assert_eq!(tokens, vec![Token::EndOfFile]);
}

#[test]
fn line_comment_ends_at_newline_and_following_tokens_still_appear() {
    let tokens = tokenize("// comment\n+");
    assert_eq!(tokens, vec![Token::Plus, Token::EndOfFile]);
}

#[test]
fn empty_comment_with_nothing_after_the_double_slash() {
    let tokens = tokenize("//");
    assert_eq!(tokens, vec![Token::EndOfFile]);
}

#[test]
fn division_between_other_tokens() {
    let tokens = tokenize("(/)");
    assert_eq!(
        tokens,
        vec![
            Token::LeftParenthesis,
            Token::Slash,
            Token::RightParenthesis,
            Token::EndOfFile,
        ],
    );
}
