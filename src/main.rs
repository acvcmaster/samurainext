use crate::declaration::parse_declaration;

pub mod assignment;
pub mod boolean;
pub mod declaration;
pub mod left_arrow;
pub mod number;
pub mod operator;
pub mod space;
pub mod token;
pub mod variable;

fn main() {
    let a = parse_declaration("定義 teste <- 3");
    println!("{:?}", a.generate_code());
}
