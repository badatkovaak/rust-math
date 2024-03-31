use super::lexer::Token as T;
use std::iter::Peekable;
use std::slice::Iter;

#[derive(Debug)]
pub enum ParserError {
    Err,
}
use ParserError as PE;

pub fn get_prec(t: &T) -> Option<u8> {
    match t {
        T::Plus | T::Minus => Some(0),
        T::Mult | T::Div => Some(1),
        T::Caret => Some(2),
        _ => None,
    }
}

pub fn parse_factor(res: &mut Vec<T>, toks: &mut Peekable<Iter<T>>) -> Result<(), PE> {
    if let Some(&t) = toks.peek() {
        match t {
            T::IntLiteral(a) => {
                toks.next();
                if let Some(&t) = toks.peek() {
                    match t {
                        &T::Symbol(b) => {
                            toks.next();
                            res.push(T::IntLiteral(*a));
                            res.push(T::Symbol(b));
                            res.push(T::Mult);
                            return Ok(());
                        }
                        _ => {
                            res.push(T::IntLiteral(*a));
                            return Ok(());
                        }
                    }
                } else {
                    res.push(T::IntLiteral(*a));
                    return Ok(());
                }
            }
            T::FloatLiteral(a) => {
                toks.next();
                if let Some(&t) = toks.peek() {
                    match t {
                        &T::Symbol(b) => {
                            toks.next();
                            res.push(T::FloatLiteral(*a));
                            res.push(T::Symbol(b));
                            res.push(T::Mult);
                            return Ok(());
                        }
                        _ => {
                            res.push(T::FloatLiteral(*a));
                            return Ok(());
                        }
                    }
                } else {
                    res.push(T::FloatLiteral(*a));
                    return Ok(());
                }
            }
            T::Symbol(c) => {
                toks.next();
                res.push(T::Symbol(*c));
                return Ok(());
            }
            _ => unreachable!(),
        }
    }
    Err(PE::Err)
}

pub fn parse_term(res: &mut Vec<T>, toks: &mut Peekable<Iter<T>>) -> Result<(), PE> {
    if let Some(&t) = toks.peek() {
        match t {
            T::LParen => {
                toks.next();
            }
            T::IntLiteral(_) | T::FloatLiteral(_) | T::Symbol(_) => {
                let r = parse_factor(res, toks);
                return r;
            }
            _ => {
                return Err(PE::Err);
            }
        }
    }
    Err(PE::Err)
}

pub fn parse_expr(res: &mut Vec<T>, toks: &mut Peekable<Iter<T>>, prec: u8) -> Result<(), PE> {
    let _res1: ();
    if prec <= 1 {
        _res1 = parse_expr(res, toks, prec + 1)?;
    } else {
        _res1 = parse_term(res, toks)?;
    }
    if let Some(&t) = toks.peek() {
        match get_prec(t) {
            Some(p) => {
                if p >= prec {
                    toks.next();
                    let _res2 = parse_expr(res, toks, prec)?;
                    res.push(t.clone());
                    return Ok(());
                } else {
                    return Ok(());
                }
            }
            None => {
                return Ok(());
            }
        }
    }
    Ok(())
}

pub fn parse(input: Vec<T>) -> Result<Vec<T>, ParserError> {
    let mut toks = input.iter().peekable();
    let mut res = Vec::with_capacity(input.len());
    let _p = parse_expr(&mut res, &mut toks, 0)?;
    Ok(res)
}
