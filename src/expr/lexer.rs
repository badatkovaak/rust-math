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
        None
    }
}

pub fn eat_one(input: String) -> (Option<Token>, String) {
    let mut curr_char: char = ' ';
    for i in input.chars() {}
    return (None, String::from(""));
}
