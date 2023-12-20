#[derive(Debug, Clone)]
pub struct Polynomial(pub Vec<CAlg>);

use crate::constants::PI;
use crate::utils::{self, fequals, is_power_of_n};
use std::iter::zip;
use std::ops;

use crate::complex_nums::c_algebraic::CAlg;
use crate::utils::max_of_two;

impl ops::Neg for Polynomial {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Polynomial(self.0.iter().map(|x| -x).collect::<Vec<CAlg>>())
    }
}

impl ops::Add for Polynomial {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        if self.0.len() == rhs.0.len() {
            return Polynomial(zip(self.0, rhs.0).map(|(x, y)| x + y).collect());
        }

        let mlen = max_of_two(self.0.len(), rhs.0.len());
        let mut op1: Vec<CAlg>;
        let mut op2: Vec<CAlg>;

        if self.0.len() < mlen {
            op1 = self.0.clone();
            while op1.len() < mlen {
                op1.push(CAlg(0., 0.));
            }
            op2 = rhs.0.clone();
        } else {
            op2 = self.0.clone();
            while op2.len() < mlen {
                op2.push(CAlg(0., 0.));
            }
            op1 = self.0.clone();
        }

        Polynomial(zip(op1, op2).map(|(x, y)| x + y).collect()).prettify()
    }
}

impl ops::Sub for Polynomial {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        println!("{:?}\n{:?}", self, rhs);
        self + (-rhs)
    }
}

impl ops::Mul for Polynomial {
    type Output = Polynomial;

    fn mul(self, rhs: Self) -> Self::Output {
        let mut res = vec![CAlg(0., 0.); self.0.len() + rhs.0.len()];
        for i in 0..self.0.len() {
            for j in 0..rhs.0.len() {
                res[i + j] += self.0[i] * rhs.0[j];
            }
        }
        while match res.last() {
            Some(a) => *a == CAlg(0., 0.),
            None => false,
        } {
            res.pop();
        }
        Polynomial(res).prettify()
    }
}

impl ops::Div for Polynomial {
    type Output = (Polynomial, Polynomial);

    fn div(self, rhs: Self) -> Self::Output {
        println!("{:?}\n{:?}", self, rhs);

        if self.0.len() < rhs.0.len() {
            return (Polynomial(vec![]), self);
        }

        let mut p1 = self.0.clone();
        let p2 = rhs.0.clone();
        let p3: Vec<CAlg>;
        let mut count = 0;

        while self.0.len() > rhs.0.len() {
            p1.insert(0, CAlg(0., 0.));
            count += 1;
        }

        let mut q = Polynomial(vec![CAlg(0., 0.); count]);
        // let r = Polynomial(vec![]);

        let (c1, c2) = (p1.last().unwrap(), p2.last().unwrap());

        if c1 != c2 {
            p3 = p2.iter().map(|x| x * c1 / (*c2)).collect::<Vec<CAlg>>();
        } else {
            p3 = p2.clone();
        }

        q.0.push(c1 / c2);

        let r = Polynomial(p1) - Polynomial(p3);
        // let a = r / rhs;
        println!("Hi");

        // (q + a.0, a.1)
        (q.prettify(), r.prettify())
        // (fuse_together(, v2)q, r)
    }
}

impl Polynomial {
    fn to_string(&self) -> String {
        let mut res = String::new();
        for (i, v) in self.0.iter().enumerate() {
            res.extend(format!("{}x^{} ", v, i).chars());
        }
        res.pop();
        res
    }

    fn eval(&self, point: CAlg) -> CAlg {
        self.0
            .iter()
            .enumerate()
            .map(|(i, v)| (*v) * (point.pow(i as i64)))
            .sum::<CAlg>()
    }

    fn scale_by(&mut self, s: CAlg) {
        for i in self.0.iter_mut() {
            *i = (*i) * s;
        }
    }

    fn strip_zeros(mut self) -> Self {
        while let Some(&c) = self.0.last() {
            if c == CAlg(0., 0.) {
                self.0.pop();
            } else {
                break;
            }
        }
        self
    }

    fn normalize(mut self) -> Self {
        if let Some(&c) = self.0.last() {
            if c != CAlg(1., 0.) && c != CAlg(0., 0.) {
                self.scale_by(CAlg(1., 0.) / c);
            }
        }
        self
    }

    fn prettify(self) -> Self {
        println!("{:?}", self);
        self.strip_zeros().normalize()
    }
}

pub fn mult_values(c1: Vec<CAlg>, c2: Vec<CAlg>) -> Option<Vec<CAlg>> {
    // println!("mult_values : \n{:?}\n{:?}\n", c1, c2);
    if c1.len() != c2.len() {
        return None;
    }
    let mut res = vec![CAlg(1.0, 0.0); c1.len()];
    for i in 0..c1.len() {
        res[i] = c1[i] * c2[i];
    }
    // println!("{:?}", res);
    Some(res)
}

pub fn fuse_together(v1: &Vec<CAlg>, v2: &Vec<CAlg>) -> Vec<CAlg> {
    if v1.len() != v2.len() {
        panic!();
    }
    zip(v1, v2).map(|(x, y)| x + y).collect::<Vec<CAlg>>()
    // let res = zip()
}
