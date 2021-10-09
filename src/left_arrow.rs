use crate::token::Token;

pub fn parse_left_arrow(slice: &str) -> Token {
    if slice.starts_with("<-") {
        Token::LeftArrow { consumed: 2 }
    } else {
        Token::LeftArrow { consumed: 0 }
    }
}
