use std::ops;

#[derive(Debug)]
pub struct BigString(String);

impl ops::Neg for BigString {
    type Output = BigString;

    fn neg(mut self) -> Self::Output {
        if self.0.get(0..1) == Some("-") {
            self.0.remove(0);
        } else {
            self.0.insert(0, '-');
        }
        return BigString(self.0.clone());
    }
}

// impl ops::Add for BigString {}

impl BigString {
    pub fn is_negative(self: &Self) -> bool {
        self.0.get(0..1) == Some("-")
    }

    pub fn construct(s: String) -> Option<BigString> {
        let cond: bool = s
            .chars()
            .filter(|x| ((*x as u8) < 58) && ((*x as u8) > 47))
            // .filter(|x| *x)
            .collect::<Vec<char>>()
            .len()
            != s.len();

        if !s.is_ascii() || cond {
            return None;
        }
        Some(BigString(s.chars().rev().collect::<String>()))
    }

    pub fn print(self: &Self) {
        println!(
            "{}",
            // self.is_negative()
            "-".to_owned()
                + &self
                    .0
                    .get((self.is_negative() as usize)..self.0.len())
                    .unwrap_or("")
                    .chars()
                    .rev()
                    .collect::<String>()
        );
    }
}
