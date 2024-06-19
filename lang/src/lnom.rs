use n::bytes::complete::is_a;
use n::multi::{many0, many0_count};
use n::Parser;
use nom as n;
use nom::Err as E;
use nom::IResult;

pub fn space(input: &str) -> IResult<&str, usize> {
    many0_count(is_a(" ")).parse(input)
}

// pub fn lex_int(input: &str) -> IResult<&str, i64> {
//
// }
