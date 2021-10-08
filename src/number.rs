use crate::token::Token;

#[derive(Debug, Clone)]
pub struct Number {
    value: i32,
    consumed: usize,
}

impl Token for Number {
    fn parse(slice: &str) -> Self {
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
            Ok(value) => Self {
                value,
                consumed: parsed,
            },
            Err(_) => Self {
                value: 0,
                consumed: 0,
            },
        }
    }
}
