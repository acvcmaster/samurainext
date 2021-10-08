use regex::Regex;

use crate::token::Token;

#[derive(Debug, Clone)]
pub struct Variable {
    value: String,
    consumed: usize,
}

impl Token for Variable {
    fn parse(slice: &str) -> Self {
        let empty = Self {
            value: format!(""),
            consumed: 0,
        };

        match Regex::new(r"([^\d ][^ ]*)") {
            Ok(regex) => match regex.captures(slice) {
                Some(captures) => match captures.get(0) {
                    Some(capture) => {
                        let result = capture.as_str();
                        match slice.starts_with(result) {
                            true => Self {
                                value: result.to_owned(),
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
}
