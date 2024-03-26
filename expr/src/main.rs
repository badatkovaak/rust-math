pub mod interpreter;
pub mod lexer;
pub mod parser;

use crate::lexer::lex;
use crate::parser::parse;

fn main() {
    let l = lex("10a + 3 * 4 ^ 5 - 9b / 7c").unwrap();
    println!("{:?}", l);
    let t = parse(l).unwrap();
    println!("{:?}", t);
}
