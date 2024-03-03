use crate::utils::{fequals, max_of_two};
use std::iter::zip;
use std::ops::{self, AddAssign};
use std::ops::{Add, Mul, Neg};
use std::rc::Rc;

use crate::algebra::symbol::Symbol as S;

#[derive(Debug, Clone)]
pub struct PolySym(pub Vec<S>);

impl std::fmt::Display for PolySym {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.into_string())
    }
}

impl Neg for PolySym {
    type Output = PolySym;

    fn neg(self) -> Self::Output {
        self.scale_by(-1.)
    }
}

impl Add for PolySym {
    type Output = PolySym;

    fn add(self, rhs: Self) -> Self::Output {
        println!("\nAdd:\n{}\n{}\n", self, rhs);
        if self.len() == rhs.len() {
            return PolySym(zip(self.0, rhs.0).map(|(x, y)| x + y).collect());
        }

        let mlen = max_of_two(self.len(), rhs.len());
        let mut op1: Vec<S>;
        let mut op2: Vec<S>;

        if self.len() < mlen {
            op1 = self.0.clone();
            while op1.len() < mlen {
                op1.push(S(vec![], 0.));
            }
            op2 = rhs.0.clone();
        } else {
            op2 = rhs.0.clone();
            while op2.len() < mlen {
                op2.push(S(vec![], 0.));
            }
            op1 = self.0.clone();
        }

        PolySym(zip(op1, op2).map(|(x, y)| x + y).collect()).sym_prettify()
        // .prettify()
    }
}

impl ops::Sub for PolySym {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        self + (-rhs)
    }
}

impl ops::Mul for PolySym {
    type Output = PolySym;

    fn mul(self, rhs: Self) -> Self::Output {
        let mut res = vec![S(vec![], 0.); self.len() + rhs.len()];
        for i in 0..self.len() {
            for j in 0..rhs.len() {
                res[i + j] += self.0[i].clone() * rhs.0[j].clone();
            }
        }
        while match res.last() {
            Some(a) => *a == S(vec![], 0.),
            None => false,
        } {
            res.pop();
        }
        PolySym(res).sym_prettify()
    }
}

impl PolySym {
    fn len(&self) -> usize {
        self.0.len()
    }

    fn scale_by(&self, s: f64) -> PolySym {
        PolySym(
            self.clone()
                .0
                .iter()
                .map(|x| match x.len() {
                    0 => S(vec![], x.1 * s),
                    a => S(
                        x.0.iter()
                            .map(|(c, v)| (*c, v * s))
                            .collect::<Vec<(char, f64)>>(),
                        x.1 * s,
                    ),
                })
                .collect::<Vec<S>>(),
        )
    }

    pub fn pow(&self, p: u64) -> PolySym {
        if p == 0 {
            return PolySym(vec![S(vec![], 1.)]);
        }
        let mut res = self.clone();
        for i in 0..p - 1 {
            res = res * self.clone();
        }
        res.sym_prettify()
    }

    fn into_string(&self) -> String {
        let mut res = String::new();
        for (i, v) in self.0.iter().enumerate().rev() {
            if self.len() == 1 {
                res.extend(format!("({})", v).chars());
            } else if i == 1 && *v != S(vec![], 0.) {
                res.extend(format!("({})x + ", v).chars());
            } else if i != 0 && *v != S(vec![], 0.) {
                res.extend(format!("({})x^{} + ", v, i).chars());
            } else if i == 0 && *v != S(vec![], 0.) {
                res.extend(format!("{}", v).chars());
            }
        }
        if res.len() > 2 {
            if let Some(c) = res.get(res.len() - 2..res.len() - 1) {
                if c.chars().nth(0) == Some('+') {
                    res.pop();
                    res.pop();
                }
            }
        }
        res
    }

    pub fn sym_prettify(&self) -> Self {
        PolySym(
            self.0
                .iter()
                .map(|x| x.clone().prettify())
                .collect::<Vec<S>>(),
        )
    }

    // fn simplify(&mut self) -> () {}
}
