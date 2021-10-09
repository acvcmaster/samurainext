use crate::token::Token;

pub fn parse_space(slice: &str) -> Token {
    let chars = slice.chars();
    let mut consumed = 0;

    for char in chars {
        if char == ' ' || char == '\t' {
            consumed = consumed + 1;
        } else {
            break;
        }
    }

    Token::Space { consumed }
}
