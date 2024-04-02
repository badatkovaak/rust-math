use crate::lexer::equal_kind;

use super::lexer::Token as T;
use std::iter::Peekable;
use std::slice::Iter;

#[derive(Debug, Clone, Copy)]
pub enum ParserError {
    Err,
    DidntParse,
    OutOfToks,
    InvalidToken,
}
use ParserError as PE;

pub fn parse_token(t: T, toks: &mut Peekable<Iter<T>>) -> Result<T, PE> {
    if let Some(&d) = toks.peek() {
        match d {
            a if equal_kind(*a, t) => {
                toks.next();
                return Ok(*a);
            }
            _ => {
                return Err(PE::InvalidToken);
            }
        }
    }
    Err(PE::OutOfToks)
}

pub fn parse_tokens(t: &[T], toks: &mut Peekable<Iter<T>>) -> Result<Box<[T]>, PE> {
    let mut r = Vec::with_capacity(t.len());
    for i in t.iter() {
        r.push(parse_token(*i, toks)?);
    }
    Ok(r.into_boxed_slice())
}

pub fn parse_tokens_s(t: &[T], toks: &mut Peekable<Iter<T>>) -> Result<(), PE> {
    if t.len() == 0 {
        return Err(PE::Err);
    }
    for i in t.iter().enumerate() {
        let r = parse_token(*(i.1), toks);
        match r {
            Err(_) if i.0 == 0 => {
                return Err(PE::DidntParse);
            }
            _ => {
                r?;
            }
        }
    }
    Ok(())
}

pub fn parse_option(t: T, toks: &mut Peekable<Iter<T>>) -> Option<T> {
    if let Some(&d) = toks.peek() {
        match d {
            a if equal_kind(*a, t) => {
                toks.next();
                return Some(*a);
            }
            _ => {
                return None;
            }
        }
    }
    None
}

pub fn parse_mult_option(t: &[T], toks: &mut Peekable<Iter<T>>) -> Result<Vec<T>, PE> {
    let r = t
        .iter()
        .map(|&x| parse_token(x, toks))
        .collect::<Vec<Result<T, PE>>>();
    if r[0].is_ok() {
        let mut res = Vec::with_capacity(r.len());
        for x in r.iter() {
            res.push((*x)?);
        }
        return Ok(res);
    }
    Err(PE::Err)
}

pub fn parse_either_s(t: &[T], toks: &mut Peekable<Iter<T>>) -> Result<(), PE> {
    if let Some(&d) = toks.peek() {
        for i in t.iter() {
            if equal_kind(*d, *i) {
                toks.next();
                return Ok(());
            }
        }
        return Err(PE::InvalidToken);
    }
    Err(PE::OutOfToks)
}

pub fn parse_either(t: &[T], toks: &mut Peekable<Iter<T>>) -> Result<T, PE> {
    if let Some(&d) = toks.peek() {
        for i in t.iter() {
            if equal_kind(*d, *i) {
                toks.next();
                return Ok(*d);
            }
        }
        return Err(PE::InvalidToken);
    }
    Err(PE::OutOfToks)
}

pub fn parse_many(t: T, min: u64, toks: &mut Peekable<Iter<T>>) -> Result<Box<[T]>, PE> {
    let mut res = vec![];
    while let Some(&d) = toks.peek() {
        match d {
            a if equal_kind(*a, t) => {
                toks.next();
                res.push(*a);
            }
            _ => {
                if res.len() >= min as usize {
                    return Ok(res.into_boxed_slice());
                }
                return Err(PE::Err);
            }
        }
    }
    Err(PE::Err)
}

pub fn parse_many_s(t: T, min: u64, toks: &mut Peekable<Iter<T>>) -> Result<u64, PE> {
    let mut count = 0;
    while let Some(&d) = toks.peek() {
        match d {
            a if equal_kind(*a, t) => {
                toks.next();
                count += 1;
            }
            _ => {
                if count >= min {
                    return Ok(count);
                }
                return Err(PE::Err);
            }
        }
    }
    if count > 0 {
        return Ok(count);
    }
    Err(PE::Err)
}

pub fn parse_mult_many_s(t: &[T], min: u64, toks: &mut Peekable<Iter<T>>) -> Result<u64, PE> {
    let mut count = 0;
    loop {
        let r = parse_tokens_s(t, toks);
        match r {
            Ok(_) => {
                count += 1;
            }
            Err(PE::DidntParse) => {
                break;
            }
            Err(e) => {
                return Err(e);
            }
        }
    }
    if count >= min {
        return Ok(count);
    }
    Err(PE::Err)
}

pub fn parse_literal(res: &mut Vec<T>, toks: &mut Peekable<Iter<T>>) -> Result<(), PE> {
    res.push(parse_either(
        vec![T::IntLiteral(0), T::FloatLiteral(0.)].as_slice(),
        toks,
    )?);
    Ok(())
}

pub fn parse_func(res: &mut Vec<T>, toks: &mut Peekable<Iter<T>>) -> Result<(), PE> {
    let _ = parse_token(T::Symbol('a'), toks);
    let p1 = parse_option(T::LParen, toks);
    if p1.is_some() {
        let _ = parse_literal(res, toks);
        // let _ = parse_mult_many_s(vec![T::Semicolon], 0, toks);
    }
    Err(PE::Err)
}

// pub fn parse_pr_expr(res: &mut Vec<T>, toks: &mut Peekable<Iter<T>>) -> Result<(), PE> {
//     if let Some(&d) = toks.peek() {
//         match d {
//             T::Let => {
//                 let res = parse_func(res, toks);
//             }
//             T::Symbol(_) | T::IntLiteral(_) | T::FloatLiteral(_) | T::LParen => {}
//             _ => {
//                 return Err(PE::Err);
//             }
//         }
//     }
//     Err(PE::Err)
// }
