use crate::{
    assignment::generate_code_assignment, boolean::generate_code_boolean,
    declaration::generate_code_declaration, expression::generate_code_expression,
    factor::generate_code_factor, group::generate_code_group, number::generate_code_number,
    operator::generate_code_operator, operator::OperatorType, term::generate_code_term,
    variable::generate_code_variable,
};

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
        preceding: Option<bool>,
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
    Factor {
        value: Option<Box<Token>>,
        consumed: usize,
    },
    Term {
        left: Option<Box<Token>>,
        right: Option<Box<Token>>,
        operator: Option<Box<Token>>,
        consumed: usize,
    },
    Expression {
        left: Option<Box<Token>>,
        right: Option<Box<Token>>,
        operator: Option<Box<Token>>,
        consumed: usize,
    },
    Group {
        value: Option<Box<Token>>,
        consumed: usize,
    },
}

#[derive(Debug, Clone)]
pub struct Error {
    message: String,
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
            Token::Factor { consumed, .. } => *consumed,
            Token::Term { consumed, .. } => *consumed,
            Token::Expression { consumed, .. } => *consumed,
            Token::Group { consumed, .. } => *consumed,
        }
    }

    pub fn generate_code(&self) -> Result<String, Error> {
        let empty = String::new();

        match &self {
            Token::LeftArrow { .. } => Ok(empty),
            Token::Space { .. } => Ok(empty),
            Token::Variable { .. } => generate_code_variable(&self),
            Token::Number { .. } => generate_code_number(&self),
            Token::Boolean { .. } => generate_code_boolean(&self),
            Token::Operator { .. } => generate_code_operator(&self),
            Token::Assignment { .. } => generate_code_assignment(&self),
            Token::Declaration { .. } => generate_code_declaration(&self),
            Token::Factor { .. } => generate_code_factor(&self),
            Token::Term { .. } => generate_code_term(&self),
            Token::Expression { .. } => generate_code_expression(&self),
            Token::Group { .. } => generate_code_group(&self),
        }
    }

    pub fn code_gen_invalid_token(&self) -> Error {
        Error {
            message: format!("CODE_GEN_INVALID_TOKEN: {:?}", self),
        }
    }

    pub fn code_gen_token_missing_value(&self) -> Error {
        Error {
            message: format!("CODE_GEN_TOKEN_MISSING_VALUE: {:?}", self),
        }
    }

    pub fn code_gen_token_missing_assignment(&self) -> Error {
        Error {
            message: format!("CODE_GEN_TOKEN_MISSING_ASSIGNMENT: {:?}", self),
        }
    }

    pub fn code_gen_token_invalid_assignment(&self) -> Error {
        Error {
            message: format!("CODE_GEN_TOKEN_INVALID_ASSIGNMENT: {:?}", self),
        }
    }
}
