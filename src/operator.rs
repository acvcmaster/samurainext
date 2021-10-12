use regex::Regex;

use crate::token::{Error, Token};

#[derive(Debug, Clone)]
pub enum OperatorType {
    Add, // +
    Sub, // -
    Mul, // *
    Div, // /
}

pub fn parse_operator(slice: &str) -> Token {
    let empty = Token::Operator {
        value: None,
        preceding: None,
        consumed: 0,
    };

    match Regex::new(r"(\+|-|\*|/)") {
        Ok(regex) => match regex.captures(slice) {
            Some(captures) => match captures.get(0) {
                Some(capture) => {
                    let result = capture.as_str();
                    match slice.starts_with(result) {
                        true => match result {
                            "+" => Token::Operator {
                                value: Some(OperatorType::Add),
                                preceding: Some(false),
                                consumed: result.len(),
                            },
                            "-" => Token::Operator {
                                value: Some(OperatorType::Sub),
                                preceding: Some(false),
                                consumed: result.len(),
                            },
                            "*" => Token::Operator {
                                value: Some(OperatorType::Mul),
                                preceding: Some(true),
                                consumed: result.len(),
                            },
                            "/" => Token::Operator {
                                value: Some(OperatorType::Div),
                                preceding: Some(true),
                                consumed: result.len(),
                            },
                            _ => empty,
                        },
                        false => empty,
                    }
                }
                None => empty,
            },
            None => empty,
        },
        Err(t) => {
            println!("{:?}", t);
            empty
        }
    }
}

pub fn generate_code_operator(token: &Token) -> Result<String, Error> {
    if let Token::Operator { value, .. } = token {
        match value {
            Some(operator_type) => match operator_type {
                OperatorType::Add => Ok(format!("+")),
                OperatorType::Sub => Ok(format!("-")),
                OperatorType::Mul => Ok(format!("*")),
                OperatorType::Div => Ok(format!("/")),
            },
            None => Err(token.code_gen_token_missing_value()),
        }
    } else {
        Err(token.code_gen_invalid_token())
    }
}
