#[derive(Debug, Clone)]
pub enum Token {
    Illegal,
    Break,
    Const,
    Continue,
    Do,
    Else,
    // Enum,
    False,
    Float,
    // Func,
    For,
    If,
    Int,
    Let,
    Return,
    // Struct,
    Then,
    True,
    Id(Rc<str>),
    IntLiteral(u64),
    FloatLiteral(f64),
    StringLiteral(Rc<str>),
    Pipe,
    Composition,
    FatArrow,
    ThinArrow,
    Assign,
    Equal,
    NotEqual,
    LessThen,
    GreaterThen,
    LessTEq,
    GreaterTEq,
    And,
    Or,
    Plus,
    Minus,
    Mult,
    Div,
    LParen,
    RParen,
    LSquare,
    RSquare,
    Colon,
    Comma,
    Dot,
    Indent,
    Deindent,
}

#[derive(Debug)]
pub enum LexerError {
    IllegalCharacter,
    LexErr,
}

use std::{collections::HashMap, rc::Rc};

use LexerError as LE;
use Token as T;

// const KWORDS: &[&str] = &[
//     "break", "const", "continue", "do", "else", "enum", "false", "float", "fn", "for", "if",
//     "import", "int", "let", // "match",
//     "return", "struct", "then", "true",
// ];

pub fn lex(input: &str, spaces_per_indent: u64) -> Result<Vec<Token>, LexerError> {
    let keywords: HashMap<&str, Token> = HashMap::from([
        ("break", T::Break),
        ("const", T::Const),
        ("continue", T::Continue),
        ("do", T::Do),
        ("else", T::Else),
        // ("enum", T::Enum),
        ("false", T::False),
        ("float", T::Float),
        // ("fn", T::Func),
        ("for", T::For),
        ("if", T::If),
        ("int", T::Int),
        ("let", T::Let), // "match",
        ("return", T::Return),
        // ("struct", T::Struct),
        ("then", T::Then),
        ("true", T::True),
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
                    toks.push(T::Dot);
                }
            }
            '+' => {
                toks.push(T::Plus);
                chars.next();
            }
            '-' => {
                chars.next();
                if let Some(&d) = chars.peek() {
                    match d {
                        '>' => {
                            toks.push(T::ThinArrow);
                            chars.next();
                        }
                        _ => {
                            toks.push(T::Minus);
                            // chars.next();
                        }
                    }
                }
            }
            '/' => {
                toks.push(T::Div);
                chars.next();
            }
            '*' => {
                toks.push(T::Mult);
                chars.next();
            }
            ',' => {
                toks.push(T::Comma);
                chars.next();
            }
            '.' => {
                toks.push(T::Dot);
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
            '[' => {
                toks.push(T::LSquare);
                chars.next();
            }
            ']' => {
                toks.push(T::RSquare);
                chars.next();
            }
            ':' => {
                toks.push(T::Colon);
                chars.next();
            }
            '&' => {
                chars.next();
                if let Some(&d) = chars.peek() {
                    if d == '&' {
                        toks.push(T::And);
                        chars.next();
                    } else {
                        return Err(LE::IllegalCharacter);
                    }
                }
            }
            '|' => {
                chars.next();
                if let Some(&d) = chars.peek() {
                    match d {
                        '|' => {
                            toks.push(T::Or);
                            chars.next();
                        }
                        '>' => {
                            toks.push(T::Pipe);
                            chars.next();
                        }
                        _ => {
                            return Err(LE::IllegalCharacter);
                        }
                    }
                }
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
                        '>' => {
                            toks.push(T::Composition);
                            chars.next();
                        }
                        _ => {
                            toks.push(T::GreaterThen);
                        }
                    }
                }
            }
            '!' => {
                chars.next();
                if let Some(&d) = chars.peek() {
                    if d == '=' {
                        toks.push(T::NotEqual);
                        chars.next();
                    } else {
                        return Err(LE::IllegalCharacter);
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
                        '>' => {
                            toks.push(T::FatArrow);
                            chars.next();
                        }
                        _ => {
                            toks.push(T::Assign);
                        }
                    }
                }
            }
            ' ' => {
                chars.next();
            }
            a if a.is_ascii_alphabetic() || a == '_' => {
                let mut name = String::from(a);

                chars.next();

                while let Some(&d) = chars.peek() {
                    match d {
                        a if a.is_ascii_alphanumeric() || a == '_' => {
                            name.push(a);
                            chars.next();
                        }
                        _ => {
                            break;
                        }
                    }
                    if name.len() >= 65 {
                        return Err(LE::LexErr);
                    }
                    // chars.next();
                }
                if let Some(x) = keywords.get(name.as_str()) {
                    toks.push(x.clone());
                } else {
                    toks.push(T::Id(Rc::from(name)));
                }
            }
            _ => {
                return Err(LE::LexErr);
            }
        }
    }
    Ok(toks)
}
