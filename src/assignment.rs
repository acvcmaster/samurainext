use crate::{
    bool::parse_bool, left_arrow::parse_left_arrow, number::parse_number, space::parse_space,
    token::Token, variable::parse_variable,
};

pub fn parse_assignment(slice: &str) -> Token {
    let mut consumed = 0;
    let empty = Token::Assignment {
        left: None,
        right: None,
        consumed: 0,
    };

    /* Variable */
    let variable = parse_variable(slice);
    let current_consumed = variable.get_consumed();
    if current_consumed == 0 {
        return empty;
    } else {
        consumed = consumed + current_consumed;
    }

    /* Space */
    let space = parse_space(&slice[consumed..]);
    let current_consumed = space.get_consumed();
    if current_consumed == 0 {
        return empty;
    } else {
        consumed = consumed + current_consumed;
    }

    /* Left arrow */
    let left_arrow = parse_left_arrow(&slice[consumed..]);
    let current_consumed = left_arrow.get_consumed();
    if current_consumed == 0 {
        return empty;
    } else {
        consumed = consumed + current_consumed;
    }


    /* Space */
    let space = parse_space(&slice[consumed..]);
    let current_consumed = space.get_consumed();
    if current_consumed == 0 {
        return empty;
    } else {
        consumed = consumed + current_consumed;
    }

    let number = parse_number(&slice[consumed..]);
    let bool = parse_bool(&slice[consumed..]);

    let right = match (number.get_consumed(), bool.get_consumed()) {
        (_, 0) => Some(number),
        (0, _) => Some(bool),
        _ => None,
    };

    if let Some(value) = right {
        consumed = consumed + value.get_consumed();

        Token::Assignment {
            left: Some(Box::new(variable)),
            right: Some(Box::new(value)),
            consumed,
        }
    } else {
        empty
    }
}
