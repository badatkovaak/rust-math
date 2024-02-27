#[derive(Debug, Clone, Copy)]
pub enum ElementaryFunc {
    Ln,
    Sin,
    Cos,
    Tg,
    Ctg,
    Arcsin,
    Arccos,
    Arctg,
    Arcctg,
}

use ElementaryFunc as EF;

#[derive(Debug, Clone)]
pub enum Token {
    // Let,
    // Id(Rc<str>),
    IntLiteral(u64),
    FloatLiteral(f64),
    // ElemFunc(EF),
    Assign,
    Equal,
    LessThen,
    GreaterThen,
    LessTEq,
    GreaterTEq,
    Plus,
    Minus,
    Mult,
    Div,
    Caret,
    LParen,
    RParen,
    Semicolon,
}

#[derive(Debug)]
pub enum LexerError {
    IllegalCharacter,
    LexErr,
}

use std::{collections::HashMap, rc::Rc};

use LexerError as LE;
use Token as T;

pub fn lex(input: &str) -> Result<Vec<Token>, LexerError> {
    // let keywords: HashMap<&str, Token> = HashMap::from([("let", T::Let)]);

    let elem_func: HashMap<&str, EF> = HashMap::from([
        ("ln", EF::Ln),
        ("sin", EF::Sin),
        ("cos", EF::Cos),
        ("tg", EF::Tg),
        ("ctg", EF::Ctg),
        ("arcsin", EF::Arcsin),
        ("arccos", EF::Arccos),
        ("arctg", EF::Arctg),
        ("arcctg", EF::Arcctg),
    ]);

    let mut toks = vec![];
    let mut chars = input.chars().peekable();

    while let Some(&c) = chars.peek() {
        println!("{:?}", toks);
        match c {
            a if a.is_ascii_digit() || a == '.' => {
                chars.next();
                let mut num = String::from(a);
                let mut is_float = a == '.';

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
                                return Err(LE::IllegalCharacter);
                            }
                        }
                        _ => {
                            break;
                        }
                    }
                    chars.next();
                }

                if !is_float {
                    let x = num.parse::<u64>();
                    match x {
                        Ok(a) => {
                            toks.push(T::IntLiteral(a));
                        }
                        Err(_) => {
                            println!("Error during parsing of Integer Literal");
                            println!("{}", num);
                            return Err(LE::LexErr);
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
                            return Err(LE::LexErr);
                        }
                    }
                } else {
                    return Err(LE::LexErr);
                }
            }
            '+' => {
                toks.push(T::Plus);
                chars.next();
            }
            '-' => {
                toks.push(T::Minus);
                chars.next();
            }
            '/' => {
                toks.push(T::Div);
                chars.next();
            }
            '*' => {
                toks.push(T::Mult);
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
            '<' => {
                chars.next();
                if let Some(&d) = chars.peek() {
                    match d {
                        '=' => {
                            toks.push(T::LessTEq);
                            chars.next();
                        }
                        _ => {
                            toks.push(T::LessThen);
                        }
                    }
                }
            }
            '>' => {
                chars.next();
                if let Some(&d) = chars.peek() {
                    match d {
                        '=' => {
                            toks.push(T::GreaterTEq);
                            chars.next();
                        }
                        _ => {
                            toks.push(T::GreaterThen);
                        }
                    }
                }
            }
            '=' => {
                chars.next();
                if let Some(&d) = chars.peek() {
                    match d {
                        '=' => {
                            toks.push(T::Equal);
                            chars.next();
                        }
                        _ => {
                            toks.push(T::Assign);
                        }
                    }
                }
                // toks.push(T::Equal);
            }
            ';' => {
                toks.push(T::Semicolon);
                chars.next();
            }
            ' ' | '\t' | '\n' => {
                chars.next();
            }
            a if a.is_ascii_alphabetic() || a == '_' => {
                return Err(LE::LexErr);
                // let mut name = String::from(a);

                // chars.next();
                //
                // while let Some(&d) = chars.peek() {
                //     match d {
                //         a if a.is_ascii_alphanumeric() || a == '_' => {
                //             name.push(a);
                //             chars.next();
                //         }
                //         _ => {
                //             break;
                //         }
                //     }
                //     if name.len() >= 65 {
                //         return Err(LE::LexErr);
                //     }
                // }
                // if let Some(x) = keywords.get(name.as_str()) {
                //     toks.push(x.clone());
                // // } else if let Some(&y) = elem_func.get(name.as_str()) {
                // //     toks.push(T::ElemFunc(y));
                // } else {
                //     toks.push(T::Id(Rc::from(name)));
                // }
            }
            _ => {
                return Err(LE::LexErr);
            }
        }
    }
    Ok(toks)
}
