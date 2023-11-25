use nom::bytes::complete::take_while;
use nom::character::complete::char;
// use nom::error::Error;
use nom::error::ParseError;
use nom::multi::many_m_n;
use nom::{
    error::{Error, ErrorKind},
    Err, IResult,
};
use std::collections::HashMap;

#[derive(Debug)]
pub enum Token {
    LParen,      // '('
    RParen,      // ')'
    Plus,        // '+'
    Minus,       // '-'
    Div,         // '/'
    Mult,        // '*'
    Number(f64), // -?[0-9]*\.[0-9]*
}

pub struct TokenStream(String);

impl std::iter::Iterator for TokenStream {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        fn sp<'a, E: ParseError<&'a str>>(input: &'a str) -> IResult<&'a str, &'a str, E> {
            let chars = " \t\r\n";
            take_while(move |c| chars.contains(c))(input)
        }

        // fn minus<E: ParseError<char>>(input: char) -> IResult<char, char, E> {
        //     many_m_n(0, 1, char('-'))(input)
        // }

        // let minus = char::<'a, ParseError<&'a str>>('-');
        // let minus = many_m_n(0, 1, char::<char, Error<char>>('-'));
        None
    }
}

// #[derive(Debug, PartialEq, Eq)]
// pub enum State {
//     Start,
//     Minus,
//     Digit1,
//     Dot,
//     Digit2,
//     End,
// }
//
// pub fn dumb_fa(input: String, transitions: HashMap<char, &dyn Fn(State, char) -> State>) -> bool {
//     let mut state: State = State::Start;
//
//     for i in input.chars() {
//
//     }
//
//     if state == State::End {
//         return true;
//     }
//     return false;
// }

// pub fn eat_one(input: String) -> (Option<Token>, String) {
//     let mut curr_char: char = ' ';
//     for i in input.chars() {}
//     return (None, String::from(""));
// }
