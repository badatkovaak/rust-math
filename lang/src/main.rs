#![allow(dead_code)]
// pub mod interpreter;
// pub mod grammar;
// pub mod lcombine;
// pub mod lexer;
// pub mod lexer1;
// pub mod lnom;
// pub mod parser;

use std::io::{self, BufRead, Write};

use lrlex::{lrlex_mod, DefaultLexeme};
use lrpar::{lrpar_mod, Node};

lrlex_mod!("grammar.l");
lrpar_mod!("grammar.y");

fn pretty_print(n: &Node<DefaultLexeme, u32>) {
    match n {
        Node::Term { lexeme: t } => println!("{:?}", t),
        Node::Nonterm { ridx: _r, nodes } => {
            for node in nodes {
                pretty_print(node);
            }
        }
    }
}

fn main() {
    let lexerdef = grammar_l::lexerdef();
    let stdin = io::stdin();
    loop {
        print!(">>> ");
        io::stdout().flush().ok();
        match stdin.lock().lines().next() {
            Some(Ok(ref l)) => {
                if l.trim().is_empty() {
                    continue;
                }
                let lexer = lexerdef.lexer(l);
                let (res, errs) = grammar_y::parse(&lexer);
                for e in errs {
                    println!("{}", e.pp(&lexer, &grammar_y::token_epp));
                }
                match res {
                    Some(r) => pretty_print(&r),
                    _ => eprintln!("Unable to evaluate expression."),
                }
            }
            _ => break,
        }
    }
}
