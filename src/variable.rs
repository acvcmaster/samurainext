use regex::Regex;

use crate::token::Token;

pub fn parse_variable(slice: &str) -> Token {
    let empty = Token::Variable {
        value: None,
        consumed: 0,
    };

    match Regex::new(r"([^\d ][^ ]*)") {
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
