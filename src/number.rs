use crate::token::Token;

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
