use crate::{
    expression::parse_expression,
    space::parse_space,
    token::{Error, Token},
};

pub fn parse_group(slice: &str) -> Token {
    let mut consumed = 0;
    let empty = Token::Group {
        value: None,
        consumed: 0,
    };

    /* Space */
    let space = parse_space(slice);
    consumed = consumed + space.get_consumed();

    let has_parentheses = slice.starts_with("(");
    if has_parentheses {
        consumed = consumed + 1;
    }

    /* Expression */
    let expression = parse_expression(&slice[consumed..]);
    let current_consumed = expression.get_consumed();
    if current_consumed == 0 {
        return empty;
    } else {
        consumed = consumed + current_consumed;
    }

    /* Space */
    let space = parse_space(&slice[consumed..]);
    consumed = consumed + space.get_consumed();

    match (has_parentheses, (&slice[consumed..]).starts_with(")")) {
        (true, true) => Token::Group {
            value: Some(Box::new(expression)),
            consumed: consumed + 1,
        },
        (false, false) => Token::Group {
            value: Some(Box::new(expression)),
            consumed,
        },
        _ => empty,
    }
}

pub fn generate_code_group(token: &Token) -> Result<String, Error> {
    if let Token::Group { value, .. } = token {
        if let Some(expression) = value {
            expression.generate_code()
        } else {
            Err(token.code_gen_token_missing_value())
        }
    } else {
        Err(token.code_gen_invalid_token())
    }
}
