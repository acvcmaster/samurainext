pub trait Token {
    fn parse(slice: &str) -> Self;
}

// pub fn get_consumed(original: &str, a: dyn Token) -> &str {
//     original
// }