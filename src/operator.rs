use regex::Regex;

use crate::token::Token;

#[derive(Debug, Clone)]
pub enum OperatorType {
    Add, // 足す
    Sub, // 引く
    Mul, // 掛ける
    Div, // 割る
}

pub fn parse_operator(slice: &str) -> Token {
    let empty = Token::Operator {
        value: None,
        consumed: 0,
    };

    match Regex::new(r"(足す|引く|掛ける|割る)") {
        Ok(regex) => match regex.captures(slice) {
            Some(captures) => match captures.get(0) {
                Some(capture) => {
                    let result = capture.as_str();
                    match slice.starts_with(result) {
                        true => Token::Operator {
                            value: match result {
                                "足す" => Some(OperatorType::Add),
                                "引く" => Some(OperatorType::Sub),
                                "掛ける" => Some(OperatorType::Mul),
                                "割る" => Some(OperatorType::Div),
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
