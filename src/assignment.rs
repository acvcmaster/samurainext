use crate::{
    boolean::parse_boolean,
    left_arrow::parse_left_arrow,
    number::parse_number,
    space::parse_space,
    token::{Error, Token},
    variable::parse_variable,
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
    let bool = parse_boolean(&slice[consumed..]);

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

pub fn generate_code_assignment(token: &Token) -> Result<String, Error> {
    if let Token::Assignment { left, right, .. } = token {
        if let Some(variable) = left {
            match variable.generate_code() {
                Ok(variable_code) => {
                    if let Some(value) = right {
                        match value.generate_code() {
                            Ok(value_code) => Ok(format!("{} = {}", variable_code, value_code)),
                            Err(error) => Err(error),
                        }
                    } else {
                        Err(token.code_gen_token_missing_value())
                    }
                }
                Err(error) => Err(error),
            }
        } else {
            Err(token.code_gen_token_missing_value())
        }
    } else {
        Err(token.code_gen_invalid_token())
    }
}
