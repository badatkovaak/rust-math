#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Token {
    LParen,
    RParen,
    Plus,
    Minus,
    Mult,
    Div,
    Caret,
    Assign,
    Comma,
    // NewLine,
    Let,
    Semicolon,
    // Function,
    Symbol(char),
    // Id(Rc<str>),
    IntLiteral(i64),
    FloatLiteral(f64),
}

pub fn equal_kind(t1: T, t2: T) -> bool {
    match (t1, t2) {
        (T::IntLiteral(_), T::IntLiteral(_)) => true,
        (T::FloatLiteral(_), T::FloatLiteral(_)) => true,
        (T::Symbol(_), T::Symbol(_)) => true,
        (a, b) => a == b,
    }
}

use std::char;
use std::collections::HashMap;
use std::rc::Rc;

use Token as T;

#[derive(Debug)]
pub enum LexerError {
    Err,
}
use LexerError as LE;

pub fn lex(input: &str) -> Result<Vec<Token>, LexerError> {
    let mut toks = vec![];
    let mut chars = input.chars().peekable();
    let keywords = HashMap::from([("let", T::Let)]);

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
            '=' => {
                toks.push(T::Assign);
                chars.next();
            }
            ',' => {
                toks.push(T::Comma);
                chars.next();
            }
            ';' => {
                toks.push(T::Semicolon);
                chars.next();
            }
            ' ' | '\t' | '\n' => {
                chars.next();
            }
            a if a.is_ascii_alphabetic() => {
                let mut name = String::from(a);

                chars.next();

                while let Some(&d) = chars.peek() {
                    match d {
                        a if a.is_ascii_alphabetic() => {
                            name.push(a);
                            chars.next();
                        }
                        _ => {
                            break;
                        }
                    }
                    if name.len() >= 65 {
                        return Err(LE::Err);
                    }
                }
                if let Some(x) = keywords.get(name.as_str()) {
                    toks.push(x.clone());
                } else {
                    for i in name.chars() {
                        toks.push(T::Symbol(i));
                    }
                }
            }
            // a if a.is_ascii_alphabetic() => {
            //     toks.push(T::Symbol(c));
            //     chars.next();
            // }
            _ => {
                return Err(LE::Err);
            }
        }
    }

    Ok(toks)
}
