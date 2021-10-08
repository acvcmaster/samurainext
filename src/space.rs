use crate::token::Token;

#[derive(Debug, Clone)]
pub struct Space {
    consumed: usize,
}

impl Token for Space {
    fn parse(slice: &str) -> Self {
        let chars = slice.chars();
        let mut consumed = 0;

        for char in chars {
            if char == ' ' || char == '\t' {
                consumed = consumed + 1;
            } else {
                break;
            }
        }

        Self { consumed }
    }
}
