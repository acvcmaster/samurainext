use crate::{assignment::parse_assignment, space::parse_space, token::Token};

pub fn parse_declaration(slice: &str) -> Token {
    let mut consumed = 0;
    let keyword = "定義";
    let empty = Token::Declaration {
        assignment: None,
        consumed: 0,
    };

    /* Keyword */
    if slice.starts_with(keyword) {
        consumed = consumed + keyword.len();
    } else {
        return empty;
    }

    /* Space */
    let space = parse_space(&slice[consumed..]);
    let current_consumed = space.get_consumed();
    if current_consumed == 0 {
        return empty;
    } else {
        consumed = consumed + current_consumed;
    }

    /* Assignment */
    let assignment = parse_assignment(&slice[consumed..]);
    let current_consumed = assignment.get_consumed();
    if current_consumed == 0 {
        return empty;
    } else {
        consumed = consumed + current_consumed;
    }

    Token::Declaration {
        assignment: Some(Box::new(assignment)),
        consumed,
    }
}
