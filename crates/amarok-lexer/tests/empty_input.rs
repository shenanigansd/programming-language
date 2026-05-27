use amarok_lexer::{Token, tokenize};

#[test]
fn empty_input_produces_only_end_of_file() {
    let tokens = tokenize("");
    assert_eq!(tokens, vec![Token::EndOfFile]);
}
