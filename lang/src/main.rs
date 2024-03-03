#![allow(dead_code)]
// pub mod interpreter;
pub mod lexer;
pub mod lexer1;
pub mod parser;

use lexer::lex;
// use lexer1::lex;

fn main() {
    println!();
    // let toks1 = lex("-. + 3 - 10^3/2");
    // println!("{:?}", toks1);

    // let toks = tokenize("10 + 3 - 27 * 2.5 / .6 : < > <= >= == != = , . ( ) [ ] && || |> >> mynaem fn for if do let x 'mystr' ");

    let toks = lex(
        "10 + 3 - 27 * 2.5 / 0.6 - .7  * 1. : < > <= >= == != = , . ( ) [ ] && || |> >> fn re x =>",
        4,
    );
    println!("{:?}", toks);
    println!();
}
