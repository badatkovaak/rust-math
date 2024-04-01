use super::lexer::Token as T;
use std::iter::Peekable;
use std::slice::Iter;

#[derive(Debug, Clone, Copy)]
pub enum ParserError {
    Err,
    OutOfToks,
    InvalidToken,
}
use ParserError as PE;

pub fn parse_token(t: T, toks: &mut Peekable<Iter<T>>) -> Result<T, PE> {
    if let Some(&d) = toks.peek() {
        match d {
            a if *a == t => {
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
    for i in t.iter().enumerate() {
        let r = parse_token(*(i.1), toks);
        match r {
            Err(_) if i.0 == 0 => {
                return Ok(());
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
            a if *a == t => {
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

pub fn parse_many(t: T, min: u64, toks: &mut Peekable<Iter<T>>) -> Result<Box<[T]>, PE> {
    let mut res = vec![];
    while let Some(&d) = toks.peek() {
        match d {
            a if *a == t => {
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
            a if *a == t => {
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

// pub fn parse_mult_many_s(t: &[T], min: u64, toks: &mut Peekable<Iter<T>>) -> Result<u64, PE> {
//     let mut count = 0;
//     // while let Ok(_) = parse_tokens_s(t, toks) {
//     //     count += 1;
//     // }
//     loop {
//         let r = parse_tokens_s(t, toks);
//         match r {
//             Ok(_) => {},
//             Err(PE::OutOfToks) => {
//                 return Err(PE::OutOfToks);
//             }
//             Err(PE::InvalidToken) => {
//
//             }
//         }
//     }
//     if count >= min {
//         return Ok(count);
//     }
//     Err(PE::Err)
// }

// pub fn parse_func(res: &mut Vec<T>, toks: &mut Peekable<Iter<T>>) -> Result<(), PE> {
//     // if let Some(&t1) = toks.peek() {
//     //     match t1
//     // }
//     let l = toks.next().ok_or(PE::Err)?;
//     let i1 = toks.next().ok_or(PE::Err)?;
//     let e = toks.peek();
//     Err(PE::Err)
// }

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
