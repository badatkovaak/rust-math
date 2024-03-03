#[derive(Debug)]
pub enum Token {
    Number(f64),
    Power,
    Plus,
    Minus,
    Mult,
    Div,
    LParen,
    RParen,
}

pub fn lex(input: &str) -> Vec<Token> {
    let mut toks = vec![];
    let mut chars = input.chars().peekable();
    while let Some(&c) = chars.peek() {
        match c {
            '0'..='9' => {
                let mut digits = String::new();
                let mut has_dot = false;
                digits.push(c);
                chars.next();

                while let Some(&c1) = chars.peek() {
                    match c1 {
                        '.' | '0'..='9' => {
                            if c1 == '.' && !has_dot {
                                digits.push(c1);
                                has_dot = true;
                            } else if c1 == '.' && has_dot {
                                break;
                            } else {
                                digits.push(c1);
                            }
                            chars.next();
                        }
                        _ => {
                            break;
                        }
                    }
                }
                if digits.chars().last() == Some('.') {
                    digits.push('0');
                }
                toks.push(Token::Number(digits.parse::<f64>().unwrap()));
            }
            '.' => {
                let mut digits = String::new();

                digits.push('.');
                chars.next();

                while let Some(&c1) = chars.peek() {
                    match c1 {
                        '0'..='9' => {
                            digits.push(c1);
                            chars.next();
                        }
                        _ => {
                            break;
                        }
                    }
                }
                if digits.chars().last() == Some('.') {
                    digits.push('0');
                }
                toks.push(Token::Number(digits.parse::<f64>().unwrap()));
            }
            '-' => {
                chars.next();
                if let Some(&c1) = chars.peek() {
                    match c1 {
                        ' ' => {
                            toks.push(Token::Minus);
                        }
                        '.' | '0'..='9' => {
                            let mut digits = String::new();
                            let mut has_dot = false;
                            digits.push(c);

                            while let Some(&c1) = chars.peek() {
                                match c1 {
                                    '.' | '0'..='9' => {
                                        if c1 == '.' && !has_dot {
                                            digits.push(c1);
                                            has_dot = true;
                                        } else if c1 == '.' && has_dot {
                                            break;
                                        } else {
                                            digits.push(c1);
                                        }
                                        chars.next();
                                    }
                                    _ => {
                                        break;
                                    }
                                }
                            }
                            if digits.chars().last() == Some('.') {
                                digits.push('0');
                            }
                            toks.push(Token::Number(digits.parse::<f64>().unwrap()));
                        }
                        _ => {
                            break;
                        }
                    }
                }
            }
            '+' => {
                toks.push(Token::Plus);
                chars.next();
            }
            '*' => {
                toks.push(Token::Mult);
                chars.next();
            }
            '/' => {
                toks.push(Token::Div);
                chars.next();
            }
            '(' => {
                toks.push(Token::LParen);
                chars.next();
            }
            ')' => {
                toks.push(Token::RParen);
                chars.next();
            }
            '^' => {
                toks.push(Token::Power);
                chars.next();
            }
            _ => {
                chars.next();
            }
        }
    }
    toks
}
