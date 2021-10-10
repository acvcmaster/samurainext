use crate::{
    assignment::parse_assignment,
    space::parse_space,
    token::{Error, Token},
};

pub fn parse_declaration(slice: &str) -> Token {
    let mut consumed = 0;
    let keyword = "定義";
    let empty = Token::Declaration {
        assignment: None,
        consumed: 0,
    };

    /* Keyword */
    if !slice.starts_with(keyword) {
        return empty;
    } else {
        consumed = consumed + keyword.len();
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

pub fn generate_code_declaration(token: &Token) -> Result<String, Error> {
    if let Token::Declaration { assignment, .. } = token {
        match assignment {
            Some(value) => match value.generate_code() {
                Ok(assignment_code) => Ok(format!("int {};", assignment_code)),
                Err(error) => Err(error),
            },
            None => Err(token.code_gen_token_missing_assignment()),
        }
    } else {
        Err(token.code_gen_invalid_token())
    }
}
