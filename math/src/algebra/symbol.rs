use crate::utils::fequals;
use std::iter::zip;
use std::ops::{Add, AddAssign, Mul, Neg};

#[derive(Debug, Clone)]
pub struct Symbol(pub Vec<(char, f64)>, pub f64);

use Symbol as S;

impl std::fmt::Display for S {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.into_string())
    }
}

impl PartialEq for S {
    fn eq(&self, other: &Self) -> bool {
        match (self.len(), other.len()) {
            (0, 0) => fequals(self.1, other.1, 13),
            (0, a) | (a, 0) => false,
            (a, b) => {
                self.len() == self.len()
                    && zip(self.0.iter(), other.0.iter())
                        .filter(|((c1, v1), (c2, v2))| c1 == c2 && v1 == v2)
                        .collect::<Vec<(&(char, f64), &(char, f64))>>()
                        .len()
                        == self.len()
                    && fequals(self.1, other.1, 13)
            }
        }
    }
}

impl Add for S {
    type Output = S;

    fn add(mut self, rhs: Self) -> Self::Output {
        match (self.len(), rhs.len()) {
            (0, 0) => S(vec![], self.1 + rhs.1),
            (0, a) => S(rhs.0, self.1 + rhs.1),
            (a, 0) => S(self.0, self.1 + rhs.1),
            (a, b) => {
                self.0.extend(rhs.0.iter());
                S(self.0, self.1 + rhs.1).prettify()
            }
        }
    }
}

impl AddAssign for S {
    fn add_assign(&mut self, rhs: Self) {
        self.1 += rhs.1;
        if self.len() != 0 && rhs.len() != 0 {
            self.0.extend(rhs.0.iter());
        } else if self.len() == 0 || rhs.len() == 0 {
            if rhs.len() != 0 {
                self.0 = rhs.0;
            }
        }
    }
}

impl Neg for S {
    type Output = S;

    fn neg(self) -> Self::Output {
        self.scale_by(-1.)
    }
}

impl Mul for S {
    type Output = S;

    fn mul(self, rhs: Self) -> Self::Output {
        match (self.len(), rhs.len()) {
            (0, 0) => S(vec![], self.1 * rhs.1),
            (0, a) => rhs.scale_by(self.1),
            (a, 0) => self.scale_by(rhs.1),
            (a, b) => {
                println!("Multiplying two symbols incorrectly!");
                panic!()
            }
        }
    }
}

impl Symbol {
    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn scale_by(&self, s: f64) -> Self {
        match self.len() {
            0 => S(vec![], self.1 * s),
            a => S(
                self.0
                    .iter()
                    .map(|(c, v)| (*c, s * v))
                    .collect::<Vec<(char, f64)>>(),
                self.1 * s,
            ),
        }
    }

    fn strip_zeros(self) -> Self {
        match self.len() {
            0 => S(vec![], self.1),
            a => S(
                self.0
                    .iter()
                    .filter(|(c, v)| !fequals(*v, 0., 13))
                    .map(|&x| x)
                    .collect::<Vec<(char, f64)>>(),
                self.1,
            ),
        }
    }

    fn compact(mut self) -> Self {
        match self.len() {
            0 => S(vec![], self.1),
            a => {
                self.0.sort_by(|(c1, v1), (c2, v2)| c1.cmp(c2));
                let mut i = 0;
                // println!("{:?}", self.0);
                while (i + 1) < self.0.len() {
                    if self.0[i].0 == self.0[i + 1].0 {
                        self.0[i].1 += self.0[i + 1].1;
                        self.0.remove(i + 1);
                    } else {
                        i += 1;
                    }
                }
                S(self.0, self.1)
            }
        }
    }

    pub fn prettify(self) -> Symbol {
        self.strip_zeros().compact()
    }

    pub fn into_string(&self) -> String {
        match self.len() {
            0 => String::from(format!("{}", self.1)),
            a => {
                let mut res = String::new();
                for (c, v) in self.0.iter() {
                    if fequals(*v, 1., 13) {
                        res.extend(format!("{} + ", c).chars())
                    } else if !fequals(*v, 0., 13) {
                        res.extend(format!("{}{} + ", v, c).chars())
                    }
                }
                if res.len() > 2 {
                    if let Some(c) = res.get(res.len() - 2..res.len() - 1) {
                        if c.chars().nth(0) == Some('+') {
                            res.pop();
                            res.pop();
                            res.pop();
                        }
                    }
                }
                if !fequals(self.1, 0., 13) {
                    if self.1 > 0. {
                        res.extend(format!(" + {}", self.1).chars());
                    } else {
                        res.extend(format!(" - {}", -self.1).chars());
                    }
                }
                res
            }
        }
    }
}
