pub mod interpreter;
pub mod lexer;
pub mod parser;
pub mod parser1;

use crate::lexer::lex;
// use crate::parser::parse;

fn main() {
    let l = lex("let a = 10 ; 20 + a").unwrap();
    println!("{:?}", l);
    // let t = parse(l).unwrap();
    // println!("{:?}", t);
}
