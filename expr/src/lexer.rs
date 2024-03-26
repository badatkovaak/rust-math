#[derive(Debug, Clone, Copy)]
pub enum Token {
    LParen,
    RParen,
    Plus,
    Minus,
    Mult,
    Div,
    Caret,
    // Assign,
    // Function,
    Symbol(char),
    // Id(Rc<str>),
    IntLiteral(i64),
    FloatLiteral(f64),
}
use std::{char, rc::Rc};

use Token as T;

#[derive(Debug)]
pub enum LexerError {
    Err,
}
use LexerError as LE;

pub fn lex(input: &str) -> Result<Vec<Token>, LexerError> {
    let mut toks = vec![];
    let mut chars = input.chars().peekable();

    'outer: while let Some(&c) = chars.peek() {
        match c {
            a if a.is_ascii_digit() || a == '.' || a == '-' => {
                chars.next();
                let mut num = String::from(a);
                let mut is_float = a == '.';
                let has_minus = a == '-';

                while let Some(&d) = chars.peek() {
                    match d {
                        a if a.is_ascii_digit() => {
                            num.push(a);
                        }
                        '.' => {
                            if !is_float {
                                is_float = true;
                                num.push('.');
                            } else {
                                return Err(LE::Err);
                            }
                        }
                        _ if has_minus && num.len() == 1 => {
                            toks.push(T::Minus);
                            continue 'outer;
                        }
                        _ => {
                            break;
                        }
                    }
                    chars.next();
                }

                if !is_float {
                    let x = num.parse::<i64>();
                    match x {
                        Ok(a) => {
                            toks.push(T::IntLiteral(a));
                        }
                        Err(_) => {
                            println!("Error during parsing Integer Literal");
                            println!("{}", num);
                            return Err(LE::Err);
                        }
                    }
                } else if num.len() > 1 {
                    if let Some(s) = num.get(0..1) {
                        if s == "." {
                            num.insert(0, '0');
                        }
                    }
                    let x = num.parse::<f64>();
                    match x {
                        Ok(a) => {
                            toks.push(T::FloatLiteral(a));
                        }
                        Err(_) => {
                            println!("Error during parsing of Float Literal");
                            println!("{}", num);
                            return Err(LE::Err);
                        }
                    }
                } else {
                    return Err(LE::Err);
                }
            }
            '+' => {
                toks.push(T::Plus);
                chars.next();
            }
            // '-' => {
            //     toks.push(T::Minus);
            //     chars.next();
            // }
            '/' => {
                toks.push(T::Div);
                chars.next();
            }
            '*' => {
                toks.push(T::Mult);
                chars.next();
            }
            '^' => {
                toks.push(T::Caret);
                chars.next();
            }
            '(' => {
                toks.push(T::LParen);
                chars.next();
            }
            ')' => {
                toks.push(T::RParen);
                chars.next();
            }

            ' ' | '\t' | '\n' => {
                chars.next();
            }
            a if a.is_ascii_alphabetic() => {
                toks.push(T::Symbol(c));
                chars.next();
            }
            _ => {
                return Err(LE::Err);
            }
        }
    }

    Ok(toks)
}
