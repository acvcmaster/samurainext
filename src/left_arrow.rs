use crate::token::Token;

#[derive(Debug, Clone)]
pub struct LeftArrow {
    consumed: usize,
}

impl Token for LeftArrow {
    fn parse(slice: &str) -> Self {
        if slice.starts_with("<-") {
            Self { consumed: 2 }
        } else {
            Self { consumed: 0 }
        }
    }
}
