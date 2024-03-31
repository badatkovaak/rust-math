use super::lexer::Token as T;
use std::iter::Peekable;
use std::slice::Iter;

#[derive(Debug)]
pub enum ParserError {
    Err,
}
use ParserError as PE;

pub fn parse_func(res: &mut Vec<T>, toks: &mut Peekable<Iter<T>>) -> Result<(), PE> {
    // if let Some(&t1) = toks.peek() {
    //     match t1
    // }
    let l = toks.next().ok_or(PE::Err)?;
    let i1 = toks.next().ok_or(PE::Err)?;
    let e = toks.peek();
    Err(PE::Err)
}

pub fn parse_pr_expr(res: &mut Vec<T>, toks: &mut Peekable<Iter<T>>) -> Result<(), PE> {
    if let Some(&d) = toks.peek() {
        match d {
            T::Let => {
                let res = parse_func(res, toks);
            }
            T::Symbol(_) | T::IntLiteral(_) | T::FloatLiteral(_) | T::LParen => {}
            _ => {
                return Err(PE::Err);
            }
        }
    }
    Err(PE::Err)
}
