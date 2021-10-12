use regex::Regex;

use crate::token::{Error, Token};

pub fn parse_variable(slice: &str) -> Token {
    let empty = Token::Variable {
        value: None,
        consumed: 0,
    };

    match Regex::new(r"([a-zA-Z]{1}[a-zA-Z0-9]*)") {
        Ok(regex) => match regex.captures(slice) {
            Some(captures) => match captures.get(0) {
                Some(capture) => {
                    let result = capture.as_str();
                    match slice.starts_with(result) {
                        true => Token::Variable {
                            value: Some(result.to_owned()),
                            consumed: result.len(),
                        },
                        false => empty,
                    }
                }
                None => empty,
            },
            None => empty,
        },
        Err(_) => empty,
    }
}

pub fn generate_code_variable(token: &Token) -> Result<String, Error> {
    if let Token::Variable { value, .. } = token {
        match value {
            Some(name) => Ok(format!("{}", *name)),
            None => Err(token.code_gen_token_missing_value()),
        }
    } else {
        Err(token.code_gen_invalid_token())
    }
}
