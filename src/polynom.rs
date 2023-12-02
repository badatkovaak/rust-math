#[derive(Debug)]
pub struct Polynomial(pub Vec<f64>);

use std::iter::zip;
use std::ops;

use crate::complex_nums::c_algebraic::CAlgebraic;
use crate::utils::max_of_two_usize;

impl ops::Neg for Polynomial {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Polynomial(self.0.iter().map(|x| -x).collect::<Vec<f64>>())
    }
}

impl ops::Add for Polynomial {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        if self.0.len() == rhs.0.len() {
            return Polynomial(zip(self.0, rhs.0).map(|(x, y)| x + y).collect());
        }

        let mlen = max_of_two_usize(self.0.len(), rhs.0.len());
        let mut op1: Vec<f64>;
        let mut op2: Vec<f64>;

        if self.0.len() < mlen {
            op1 = self.0.clone();
            while op1.len() < mlen {
                op1.push(0.);
            }
            op2 = rhs.0.clone();
        } else {
            op2 = self.0.clone();
            while op2.len() < mlen {
                op2.push(0.);
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

impl ops::Mul for Polynomial {
    type Output = Polynomial;

    fn mul(self, rhs: Self) -> Self::Output {
        fft_mul(self, rhs)
    }
}

pub fn fft_mul(p1: Polynomial, p2: Polynomial) -> Polynomial {
    fn fft(coefs: Vec<f64>, omega: CAlgebraic) -> Vec<CAlgebraic> {
        let n = coefs.len();
        if n == 0 {
            return coefs.iter().map(|x| CAlgebraic(*x, 0.)).collect();
        }

        let mut p_e: Vec<f64> = vec![];
        let mut p_o: Vec<f64> = vec![];

        for i in 0..n {
            if i % 2 == 0 {
                p_e.push(coefs[i]);
            } else {
                p_o.push(coefs[i]);
            }
        }

        let y_e = fft(p_e, omega);
        let y_o = fft(p_o, omega);
        let mut res = vec![CAlgebraic(0., 0.); n];

        for i in 0..n / 2 {
            res[i] = y_e[i] + omega.pow(i as i64) * y_o[i];
            res[i + n / 2] = y_e[i] - omega.pow(i as i64) * y_o[i];
        }
        res
    }
    return p1;
}
