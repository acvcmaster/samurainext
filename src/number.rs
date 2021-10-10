use crate::token::{Error, Token};

pub fn parse_number(slice: &str) -> Token {
    let mut parsed = 0;
    let mut result_array: Vec<char> = vec![];

    for char in slice.chars() {
        if char.is_numeric() {
            parsed = parsed + 1;
            result_array.push(char);
        }
    }

    let result: String = result_array.iter().collect();

    match result.parse::<i32>() {
        Ok(value) => Token::Number {
            value: Some(value),
            consumed: parsed,
        },
        Err(_) => Token::Number {
            value: None,
            consumed: 0,
        },
    }
}

pub fn generate_code_number(token: &Token) -> Result<String, Error> {
    if let Token::Number { value, .. } = token {
        match value {
            Some(number) => Ok(format!("{}", *number)),
            None => Err(token.code_gen_token_missing_value()),
        }
    } else {
        Err(token.code_gen_invalid_token())
    }
}
