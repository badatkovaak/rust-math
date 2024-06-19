#[derive(Debug, Clone)]
pub struct Lexer(String);

impl Lexer {
    pub fn new(input: String) -> Lexer {
        Lexer(input)
    }

    pub fn new_from_str(input: &str) -> Lexer {
        Lexer(input.to_string())
    }
}
