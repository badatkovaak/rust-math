use super::lexer::Token;

// pub struct ()
#[derive(Debug)]
pub enum Value {
    Float(f64),
    Int(i64),
}

#[derive(Debug)]
pub enum Op {
    Plus,
    Minus,
    Mult,
    Div,
    Power,
}

#[derive(Debug)]
pub enum Expr {
    Parenthesized(Box<Expr>),
    Binary(Op, Box<Expr>, Box<Expr>),
    Val(Value),
}

#[derive(Debug)]
pub struct Program(Vec<Expr>);

pub enum Precs {
    Prec1 = 0,
    Prec2 = 1,
    MaxPrec = 2,
}

#[derive(Debug)]
pub enum ParserError {
    ParserError,
}
use Expr as E;
use ParserError as PE;
use Token as T;
use Value as V;

pub fn parse_program(input: Vec<Token>) -> Result<Program, ParserError> {
    let mut toks = input.iter().rev().peekable();
    while let Some(&t) = toks.peek() {}
    return Err(PE::ParserError);
}

pub fn parse_expr(input: Vec<Token>, prec: Precs) -> Result<Expr, ParserError> {
    let mut toks = input.iter().peekable();
    let Some(&c) = toks.peek() else {
        return Err(PE::ParserError);
    };
    match c {
        T::LParen => {
            let Ok(d) = parse_expr(input, prec) else {
                return Err(PE::ParserError);
            };
        }
        T::IntLiteral(a) => {
            return Ok(E::Val(V::Int(*a as i64)));
        }
        T::FloatLiteral(a) => {
            return Ok(E::Val(V::Float(*a)));
        }
        _ => {
            return Err(PE::ParserError);
        }
    }
    return Err(PE::ParserError);
}
