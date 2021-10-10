use regex::Regex;

use crate::token::{Error, Token};

pub fn parse_boolean(slice: &str) -> Token {
    let empty = Token::Boolean {
        value: None,
        consumed: 0,
    };

    match Regex::new(r"(ああ|嗚呼|いな|否)") {
        Ok(regex) => match regex.captures(slice) {
            Some(captures) => match captures.get(0) {
                Some(capture) => {
                    let result = capture.as_str();
                    match slice.starts_with(result) {
                        true => Token::Boolean {
                            value: match result {
                                "ああ" | "嗚呼" => Some(true),
                                "いな" | "否" => Some(false),
                                _ => None,
                            },
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

pub fn generate_code_boolean(token: &Token) -> Result<String, Error> {
    if let Token::Boolean { value, .. } = token {
        match value {
            Some(boolean) => match boolean {
                true => Ok(format!("1")),
                false => Ok(format!("0")),
            },
            None => Err(token.code_gen_token_missing_value()),
        }
    } else {
        Err(token.code_gen_invalid_token())
    }
}
