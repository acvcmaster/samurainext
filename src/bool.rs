use regex::Regex;

use crate::token::Token;

pub fn parse_bool(slice: &str) -> Token {
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
