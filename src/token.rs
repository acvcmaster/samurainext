use crate::operator::OperatorType;

#[derive(Debug, Clone)]
pub enum Token {
    LeftArrow {
        consumed: usize,
    },
    Space {
        consumed: usize,
    },
    Variable {
        value: Option<String>,
        consumed: usize,
    },
    Number {
        value: Option<i32>,
        consumed: usize,
    },
    Boolean {
        value: Option<bool>,
        consumed: usize,
    },
    Operator {
        value: Option<OperatorType>,
        consumed: usize,
    },
    Assignment {
        left: Option<Box<Token>>,
        right: Option<Box<Token>>,
        consumed: usize,
    },
    Declaration {
        assignment: Option<Box<Token>>,
        consumed: usize,
    },
}

impl Token {
    pub fn get_consumed(&self) -> usize {
        match self {
            Token::LeftArrow { consumed } => *consumed,
            Token::Space { consumed } => *consumed,
            Token::Variable { consumed, .. } => *consumed,
            Token::Number { consumed, .. } => *consumed,
            Token::Boolean { consumed, .. } => *consumed,
            Token::Operator { consumed, .. } => *consumed,
            Token::Assignment { consumed, .. } => *consumed,
            Token::Declaration { consumed, .. } => *consumed,
        }
    }
}
