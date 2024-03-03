#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Token {
    IntLiteral(i64),
    FloatLiteral(f64),
    Id(char),
    Plus,
    Minus,
    Mult,
    Div,
    Caret,
    LParen,
    RParen,
    Semicolon,
    X,
}

use std::{collections::HashMap, rc::Rc};

use Token as T;

use crate::algebra::{sym_poly::PolySym, symbol::Symbol};

pub fn lex(input: &str) -> Option<Vec<Token>> {
    let mut toks = vec![];
    let mut chars = input.chars().peekable();

    while let Some(&c) = chars.peek() {
        println!("{:?}", toks);
        match c {
            a if a.is_ascii_digit() || a == '.' || a == '-' => {
                chars.next();
                let is_minus = a == '-';
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
                                return None;
                            }
                        }
                        _ => {
                            break;
                        }
                    }
                    chars.next();
                }

                if !is_float && !(is_minus && num.len() == 1) {
                    let x = num.parse::<i64>();
                    match x {
                        Ok(a) => {
                            toks.push(T::IntLiteral(a));
                        }
                        Err(_) => {
                            println!("Error during parsing of Integer Literal");
                            println!("{}", num);
                            return None;
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
                            return None;
                        }
                    }
                } else if is_minus && num.len() == 1 {
                    toks.push(T::Minus);
                } else {
                    return None;
                }
            }
            '+' => {
                toks.push(T::Plus);
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
            '^' => {
                toks.push(T::Caret);
                chars.next();
            }
            ';' => {
                toks.push(T::Semicolon);
                chars.next();
            }
            ' ' | '\t' | '\n' => {
                chars.next();
            }
            'x' | 'X' => {
                toks.push(T::X);
                chars.next();
            }
            a if a.is_ascii_alphabetic() => {
                toks.push(T::Id(a));
                chars.next();
            }
            _ => {
                return None;
            }
        }
    }
    Some(toks)
}

#[derive(Debug, Clone)]
pub enum PolyPow {
    Sym(SymExpr, u64),
    Fac(Option<(char, f64)>, f64),
}

#[derive(Debug, Clone)]
pub struct SymExpr(Vec<(char, f64)>, f64);

#[derive(Debug, Clone)]
pub struct Polynomial(Vec<PolyPow>);

#[derive(Debug, Clone)]
pub struct Parser {
    pub toks: Vec<T>,
    pub pos: usize,
}

impl Iterator for Parser {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let res: Option<T>;
        if self.pos < self.toks.len() {
            res = Some(self.toks[self.pos]);
            self.pos += 1;
        } else {
            res = None;
        }
        res
    }
}

impl Parser {
    fn peek(&mut self) -> Option<T> {
        let mut res: Option<T> = None;
        if self.pos < self.toks.len() {
            res = Some(self.toks[self.pos]);
        }
        res
    }
}

fn parse_token(parser: &mut Parser, token: T) -> Option<T> {
    if let Some(d) = parser.peek() {
        match d {
            t if t == token => {
                parser.next();
                return Some(t);
            }
            _ => return None,
        }
    } else {
        return None;
    }
}

fn parse_factor(parser: &mut Parser) -> Option<f64> {
    if let Some(d) = parser.peek() {
        match d {
            T::IntLiteral(a) => {
                parser.next();
                Some(a as f64)
            }
            T::FloatLiteral(f) => {
                parser.next();
                Some(f)
            }
            _ => None,
        }
    } else {
        return None;
    }
}

fn parse_sym_expr(parser: &mut Parser) -> Option<SymExpr> {
    fn parse_fact_or_fact_id(parser: &mut Parser) -> Option<(Option<char>, f64)> {
        let Some(d) = parser.peek() else {
            println!("Path5");
            return None;
        };
        match d {
            T::IntLiteral(f) => {
                parser.next();
                if let Some(a) = parser.peek() {
                    match a {
                        T::Id(c) => {
                            parser.next();
                            return Some((Some(c), f as f64));
                        }
                        _ => return Some((None, f as f64)),
                    }
                } else {
                    return Some((None, f as f64));
                }
            }
            T::FloatLiteral(f) => {
                parser.next();
                if let Some(a) = parser.peek() {
                    match a {
                        T::Id(c) => {
                            parser.next();
                            return Some((Some(c), f));
                        }
                        _ => return Some((None, f)),
                    }
                } else {
                    return Some((None, f));
                }
            }
            T::Id(c) => {
                parser.next();
                return Some((Some(c), 1.));
            }
            _ => {
                println!("Path4");
                return None;
            }
        }
    }

    let mut is_minus = false;
    let mut fac_accum = 0.;
    let mut ids: Vec<(char, f64)> = vec![];
    loop {
        match parse_fact_or_fact_id(parser) {
            Some((Some(c), f)) => {
                ids.push((
                    c,
                    if is_minus {
                        is_minus = !is_minus;
                        -f
                    } else {
                        f
                    },
                ));
            }
            Some((None, f)) => {
                fac_accum += if is_minus {
                    is_minus = !is_minus;
                    -f
                } else {
                    f
                };
            }
            None => return None,
        }
        if let Some(d) = parser.peek() {
            match d {
                T::Plus => {
                    parser.next();
                }
                T::Minus => {
                    parser.next();
                    is_minus = true;
                }
                _ => break,
            }
        } else {
            return Some(SymExpr(ids, fac_accum));
        }
    }
    // println!("End");
    return Some(SymExpr(ids, fac_accum));
}

fn parse_poly_pow(parser: &mut Parser) -> Option<PolyPow> {
    let d = parse_factor(parser);
    if let Some(a) = parser.peek() {
        // println!("Br1");
        match a {
            T::Id(c) => {
                parser.next();
                if let Some(b) = parser.peek() {
                    if b == T::Plus {
                        if let Some(f) = parse_factor(parser) {
                            return Some(PolyPow::Fac(Some((c, d.unwrap_or(1.))), f));
                        } else {
                            return Some(PolyPow::Fac(Some((c, d.unwrap_or(1.))), 0.));
                        }
                    }
                } else {
                    return Some(PolyPow::Fac(Some((c, d.unwrap_or(1.))), 0.));
                }
            }
            _ => {}
        }
    } else {
        if d.is_some() {
            return Some(PolyPow::Fac(None, d.unwrap_or(0.)));
        }
    }

    if let Some(d1) = parse_token(parser, T::X) {
        // parser.next();
        // println!("Br2");
        let e = parse_token(parser, T::Caret);
        let f = {
            if let Some(d2) = parser.peek() {
                match d2 {
                    T::IntLiteral(a) => {
                        parser.next();
                        Some(a as u64)
                    }
                    _ => None,
                }
            } else {
                None
            }
        };
        if (e.is_some() && f.is_some()) || (e.is_none() && f.is_none()) {
            return Some(PolyPow::Sym(SymExpr(vec![], 1.), f.unwrap_or(1)));
        } else {
            // println!("Hi");
            println!("Path3");
            return None;
        }
    }

    let x1 = parse_token(parser, T::LParen)?;
    let x2 = parse_sym_expr(parser)?;
    let x3 = parse_token(parser, T::RParen)?;
    let x4 = parse_token(parser, T::X)?;

    let x5 = parse_token(parser, T::Caret);
    let x6 = {
        if let Some(d) = parser.peek() {
            match d {
                T::IntLiteral(a) => {
                    parser.next();
                    Some(a as u64)
                }
                _ => None,
            }
        } else {
            None
        }
    };

    if (x5.is_some() && x6.is_some()) || (x5.is_none() && x6.is_none()) {
        Some(PolyPow::Sym(x2, x6.unwrap_or(1)))
    } else {
        println!("Path2");
        None
    }
}

pub fn parse_poly(parser: &mut Parser) -> Option<Polynomial> {
    let mut res = vec![];
    let mut is_minus = false;
    while let Some(d) = parse_poly_pow(parser) {
        println!("{:?}", d);
        if is_minus {
            match d {
                PolyPow::Sym(SymExpr(ref s, v), p) => {
                    res.push(PolyPow::Sym(
                        SymExpr(
                            s.iter()
                                .map(|(a, b)| (*a, -b))
                                .collect::<Vec<(char, f64)>>(),
                            -v,
                        ),
                        p,
                    ));
                }
                PolyPow::Fac(Some((c, v)), f) => {
                    res.push(PolyPow::Fac(Some((c, -v)), -f));
                }
                PolyPow::Fac(None, f) => res.push(PolyPow::Fac(None, -f)),
            }
        } else {
            res.push(d);
        }
        if let Some(b) = parser.peek() {
            match b {
                T::Plus => {
                    parser.next();
                }
                T::Minus => {
                    parser.next();
                    is_minus = true;
                }
                _ => break,
            }
        }
        // let x = parse_token(parser, T::Plus);
    }
    if res.len() == 0 {
        println!("Path1");
        return None;
    }
    println!("{:?}", Polynomial(res.clone()));
    return Some(Polynomial(res));
}

pub fn poly_to_polysym(p: Polynomial) -> PolySym {
    let mut s = vec![Symbol(vec![], 0.)];
    for i in p.0.iter() {
        println!("{:?}", i);
        match i {
            PolyPow::Sym(SymExpr(a, b), c) => {
                if s.len() <= (*c as usize) {
                    s.extend(vec![Symbol(vec![], 0.); (*c as usize) - s.len() + 2]);
                }
                if s[*c as usize] != Symbol(vec![], 0.) {
                    s[*c as usize].1 += *b;
                    s[*c as usize].0.extend(a.iter());
                    s[*c as usize] = s[*c as usize + 1].clone().prettify();
                } else {
                    s[*c as usize] = Symbol(a.clone(), *b);
                }
            }
            PolyPow::Fac(Some((a, b)), f) => {
                s[0] += Symbol(vec![(*a, *b)], *f);
            }
            PolyPow::Fac(None, f) => {
                s[0] += Symbol(vec![], *f);
            }
        }
    }
    // println!("poly : {}", )
    PolySym(s)
}

pub fn run_through(input: &str) -> Option<PolySym> {
    Some(poly_to_polysym(parse_poly(&mut Parser {
        toks: lex(&input.chars().chain(" ".chars()).collect::<String>())?,
        pos: 0,
    })?))
}
