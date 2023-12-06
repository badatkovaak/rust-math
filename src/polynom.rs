#[derive(Debug)]
pub struct Polynomial(pub Vec<CAlg>);

use crate::constants::PI;
use crate::utils::is_power_of_n;
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

        Polynomial(zip(op1, op2).map(|(x, y)| x + y).collect())
    }
}

impl ops::Sub for Polynomial {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        self + (-rhs)
    }
}

// impl ops::Mul for Polynomial {
//     type Output = Polynomial;
//
//     fn mul(self, rhs: Self) -> Self::Output {
//         fft_mul(self, rhs)
//     }
// }

pub fn mult_values(c1: Vec<CAlg>, c2: Vec<CAlg>) -> Option<Vec<CAlg>> {
    println!("mult_values : \n{:?}\n{:?}\n", c1, c2);
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

impl Polynomial {
    pub fn eval(self, point: CAlg) -> CAlg {
        self.0
            .iter()
            .enumerate()
            .map(|(i, v)| (*v) * (point.pow(i as i64)))
            .sum::<CAlg>()
    }
}

// pub fn fft_mul(p1: Polynomial, p2: Polynomial) -> Polynomial {
//     let res = mult_values(fft(p1.0, false), fft(p2.0, false));
//     Polynomial(fft(res.unwrap(), true))
// }
