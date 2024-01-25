#[derive(Debug, Clone)]
pub struct PolyC(pub Vec<Complex>);

use crate::constants::PI;
use crate::utils::{self, fequals, is_power_of_n};
use std::iter::zip;
use std::ops;

use crate::algebra::complex::Complex;
use crate::utils::max_of_two;

impl std::fmt::Display for PolyC {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.into_string())
    }
}

impl ops::Neg for PolyC {
    type Output = Self;

    fn neg(self) -> Self::Output {
        PolyC(self.0.iter().map(|x| -x).collect::<Vec<Complex>>())
    }
}

impl ops::Add for PolyC {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        if self.len() == rhs.len() {
            return PolyC(zip(self.0, rhs.0).map(|(x, y)| x + y).collect());
        }

        let mlen = max_of_two(self.len(), rhs.len());
        let mut op1: Vec<Complex>;
        let mut op2: Vec<Complex>;

        if self.len() < mlen {
            op1 = self.0.clone();
            while op1.len() < mlen {
                op1.push(Complex(0., 0.));
            }
            op2 = rhs.0.clone();
        } else {
            op2 = rhs.0.clone();
            while op2.len() < mlen {
                op2.push(Complex(0., 0.));
            }
            op1 = self.0.clone();
        }

        PolyC(zip(op1, op2).map(|(x, y)| x + y).collect()).prettify()
    }
}

impl ops::Sub for PolyC {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        self + (-rhs)
    }
}

impl ops::Mul for PolyC {
    type Output = PolyC;

    fn mul(self, rhs: Self) -> Self::Output {
        let mut res = vec![Complex(0., 0.); self.len() + rhs.len()];
        for i in 0..self.len() {
            for j in 0..rhs.len() {
                res[i + j] += self.0[i] * rhs.0[j];
            }
        }
        while match res.last() {
            Some(a) => *a == Complex(0., 0.),
            None => false,
        } {
            res.pop();
        }
        PolyC(res).prettify()
    }
}

impl ops::Div for PolyC {
    type Output = (PolyC, PolyC);

    fn div(self, rhs: Self) -> Self::Output {
        if self.len() < rhs.len() {
            return (PolyC(vec![Complex(0., 0.)]), self);
        }

        let p1 = self.0.clone();
        let mut p2 = rhs.0.clone();
        let p3: Vec<Complex>;
        let mut count = 0;

        while p1.len() > p2.len() {
            p2.insert(0, Complex(0., 0.));
            count += 1;
        }

        let mut q = PolyC(vec![Complex(0., 0.); count]);

        let (c1, c2) = (p1.last().unwrap(), p2.last().unwrap());

        if c1 != c2 {
            p3 = p2.iter().map(|x| x * c1 / (*c2)).collect::<Vec<Complex>>();
        } else {
            p3 = p2.clone();
        }

        q.0.push(c1 / c2);

        let r = PolyC(p1) - PolyC(p3);
        let (a, b) = r.clone().strip_zeros() / rhs.clone();
        ((q + a).prettify(), b.prettify())
    }
}

impl PolyC {
    fn into_string(&self) -> String {
        let mut res = String::new();
        for (i, v) in self.0.iter().enumerate().rev() {
            if self.len() == 1 {
                res.extend(format!("({})", v).chars());
            } else if i != 0 && *v != Complex(0., 0.) {
                res.extend(format!("({})x^{} + ", v, i).chars());
            } else if i == 0 && *v != Complex(0., 0.) {
                res.extend(format!("({})", v).chars());
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

    fn eval(&self, point: Complex) -> Complex {
        self.0
            .iter()
            .enumerate()
            .map(|(i, v)| (*v) * (point.pow(i as i64)))
            .sum::<Complex>()
    }

    fn scale_by(&mut self, s: Complex) {
        for i in self.0.iter_mut() {
            *i = (*i) * s;
        }
    }

    fn strip_zeros(mut self) -> Self {
        while let Some(&c) = self.0.last() {
            if c == Complex(0., 0.) && self.len() > 1 {
                self.0.pop();
            } else {
                break;
            }
        }
        self
    }

    fn normalize(mut self) -> Self {
        if let Some(&c) = self.0.last() {
            if c != Complex(1., 0.) && c != Complex(0., 0.) && self.len() > 0 {
                self.scale_by(Complex(1., 0.) / c);
            }
        }
        self
    }

    fn prettify(self) -> Self {
        self.strip_zeros().normalize()
    }

    fn len(&self) -> usize {
        self.0.len()
    }

    pub fn gcd(p1: PolyC, p2: PolyC) -> PolyC {
        let mut a: PolyC;
        let mut b: PolyC;

        if p1.len() > p2.len() {
            (a, b) = (p1, p2);
        } else {
            (a, b) = (p2, p1);
        }

        while a.len() > 1 && b.len() > 1 {
            (a, b) = (b.clone(), (a / b).1);
        }

        if a.len() == 1 && a.0[0] == Complex(0., 0.) {
            return b;
        } else if b.len() == 1 && b.0[0] == Complex(0., 0.) {
            return a;
        } else if a.len() == 1 {
            return a;
        } else if b.len() == 1 {
            return b;
        } else {
            return PolyC(vec![Complex(0., 0.)]);
        }
    }
}

// pub fn mult_values(c1: Vec<Complex>, c2: Vec<Complex>) -> Option<Vec<Complex>> {
//     if c1.len() != c2.len() {
//         return None;
//     }
//     let mut res = vec![Complex(1.0, 0.0); c1.len()];
//     for i in 0..c1.len() {
//         res[i] = c1[i] * c2[i];
//     }
//     Some(res)
// }

// pub fn fuse_together(v1: &Vec<Complex>, v2: &Vec<Complex>) -> Vec<Complex> {
//     if v1.len() != v2.len() {
//         panic!();
//     }
//     zip(v1, v2).map(|(x, y)| x + y).collect::<Vec<Complex>>()
// }
