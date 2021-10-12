use crate::{declaration::parse_declaration, expression::parse_expression, factor::parse_factor, operator::parse_operator, term::parse_term};

pub mod assignment;
pub mod boolean;
pub mod declaration;
pub mod left_arrow;
pub mod number;
pub mod operator;
pub mod space;
pub mod token;
pub mod variable;
pub mod factor;
pub mod term;
pub mod expression;
pub mod group;

fn main() {
    let a = parse_expression("a + b + c");
    println!("{:?}", a);
    println!("{:?}", a.generate_code());
}
