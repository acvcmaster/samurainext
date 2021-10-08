use token::Token;

use crate::space::Space;

pub mod token;
pub mod number;
pub mod variable;
pub mod left_arrow;
pub mod space;

fn main() {
    let a = Space::parse("    a2222");
    println!("{:?}", a);
}
