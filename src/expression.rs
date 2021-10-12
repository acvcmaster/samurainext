use crate::{
    operator::parse_operator,
    space::parse_space,
    term::parse_term,
    token::{Error, Token},
};

pub fn parse_expression(slice: &str) -> Token {
    let mut consumed = 0;
    let empty = Token::Expression {
        left: None,
        right: None,
        operator: None,
        consumed: 0,
    };

    /* Space */
    let space = parse_space(&slice[consumed..]);
    consumed = consumed + space.get_consumed();

    /* Left */
    let left = parse_term(&slice[consumed..]);
    let current_consumed = left.get_consumed();
    if current_consumed == 0 {
        return empty;
    } else {
        consumed = consumed + current_consumed;
    }

    /* Space */
    let space = parse_space(&slice[consumed..]);
    consumed = consumed + space.get_consumed();


    /* Operator */
    let operator = parse_operator(&slice[consumed..]);
    let current_consumed = operator.get_consumed();
    if current_consumed == 0 {
        return Token::Expression {
            left: Some(Box::new(left)),
            right: None,
            operator: None,
            consumed,
        };
    } else if let Token::Operator { preceding, .. } = operator {
        match preceding {
            Some(false) => consumed = consumed + current_consumed,
            _ => return empty,
        };
    }

    /* Space */
    let space = parse_space(&slice[consumed..]);
    let current_consumed = space.get_consumed();
    if current_consumed == 0 {
        return empty;
    } else {
        consumed = consumed + current_consumed;
    }

    /* Right */
    let right = parse_term(&slice[consumed..]);
    let current_consumed = right.get_consumed();
    if current_consumed == 0 {
        return empty;
    } else {
        consumed = consumed + current_consumed;
    }

    Token::Expression {
        left: Some(Box::new(left)),
        right: Some(Box::new(right)),
        operator: Some(Box::new(operator)),
        consumed,
    }
}

pub fn generate_code_expression(token: &Token) -> Result<String, Error> {
    if let Token::Expression {
        left,
        right,
        operator,
        ..
    } = token
    {
        match (left, right, operator) {
            (Some(le), Some(ri), Some(op)) => {
                let le_code = le.generate_code();
                let op_code = op.generate_code();
                let ri_code = ri.generate_code();

                match (le_code, op_code, ri_code) {
                    (Ok(l), Ok(o), Ok(r)) => Ok(format!("({} {} {})", l, o, r)),
                    _ => Err(token.code_gen_token_missing_value()),
                }
            }
            (Some(le), ..) => {
                let le_code = le.generate_code();

                match le_code {
                    Ok(l) => Ok(format!("{}", l)),
                    Err(error) => Err(error),
                }
            }
            _ => Err(token.code_gen_token_missing_value()),
        }
    } else {
        Err(token.code_gen_invalid_token())
    }
}
