use crate::{
    group::parse_group,
    number::parse_number,
    space::parse_space,
    token::{Error, Token},
    variable::parse_variable,
};

pub fn parse_factor(slice: &str) -> Token {
    let mut consumed = 0;
    let empty = Token::Factor {
        value: None,
        consumed: 0,
    };

    /* Space */
    let space = parse_space(slice);
    let current_consumed = space.get_consumed();
    consumed = consumed + current_consumed;

    let number = parse_number(&slice[consumed..]);
    let number_consumed = number.get_consumed();
    if let Token::Number { value, .. } = number {
        if value.is_some() {
            return Token::Factor {
                value: Some(Box::new(number)),
                consumed: consumed + number_consumed,
            };
        } else {
            let variable = parse_variable(&slice[consumed..]);
            let variable_consumed = variable.get_consumed();
            if let Token::Variable { ref value, .. } = variable {
                if value.is_some() {
                    return Token::Factor {
                        value: Some(Box::new(variable)),
                        consumed: consumed + variable_consumed,
                    };
                } else {
                    let group = parse_group(&slice[consumed..]);
                    let group_consumed = group.get_consumed();
                    if let Token::Group { ref value, .. } = group {
                        if value.is_some() {
                            return Token::Factor {
                                value: Some(Box::new(group)),
                                consumed: consumed + group_consumed,
                            };
                        } else {
                            return empty;
                        }
                    } else {
                        return empty;
                    }
                }
            } else {
                return empty;
            }
        }
    } else {
        return empty;
    }
}

pub fn generate_code_factor(token: &Token) -> Result<String, Error> {
    if let Token::Factor { value, .. } = token {
        if let Some(factor) = value {
            factor.generate_code()
        } else {
            Err(token.code_gen_token_missing_value())
        }
    } else {
        Err(token.code_gen_invalid_token())
    }
}
