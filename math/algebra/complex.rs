use crate::algebra::c_polar;
use crate::utils::fequals;
use std::cmp;
use std::ops;

#[derive(Debug, Clone, Copy)]
pub struct Complex(pub f64, pub f64);

impl std::fmt::Display for Complex {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} + {}i", self.0, self.1)
    }
}

impl cmp::PartialEq for Complex {
    fn eq(&self, other: &Self) -> bool {
        fequals(self.0, other.0, 13) && fequals(self.1, other.1, 13)
    }
}

impl ops::Neg for Complex {
    type Output = Complex;
    fn neg(self) -> Self::Output {
        self.scale(-1.)
    }
}

impl ops::Add for Complex {
    type Output = Complex;

    fn add(self, rhs: Self) -> Self::Output {
        Complex(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl ops::AddAssign for Complex {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl ops::Sub for Complex {
    type Output = Complex;

    fn sub(self, rhs: Self) -> Self::Output {
        Complex(self.0 - rhs.0, self.1 - rhs.1)
    }
}

impl ops::Mul for Complex {
    type Output = Complex;

    fn mul(self, rhs: Self) -> Self::Output {
        Self(
            self.0 * rhs.0 - self.1 * rhs.1,
            self.0 * rhs.1 + self.1 * rhs.0,
        )
    }
}

impl ops::Div for Complex {
    type Output = Complex;

    fn div(self, rhs: Self) -> Self::Output {
        self.scale(1. / (rhs.0 * rhs.0 + rhs.1 * rhs.1)) * rhs.conjugate()
    }
}

impl ops::Neg for &Complex {
    type Output = Complex;
    fn neg(self) -> Self::Output {
        self.scale(-1.)
    }
}

impl ops::Add for &Complex {
    type Output = Complex;

    fn add(self, rhs: Self) -> Self::Output {
        Complex(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl ops::Sub for &Complex {
    type Output = Complex;

    fn sub(self, rhs: Self) -> Self::Output {
        Complex(self.0 - rhs.0, self.1 - rhs.1)
    }
}

impl ops::Mul for &Complex {
    type Output = Complex;

    fn mul(self, rhs: Self) -> Self::Output {
        Complex(
            self.0 * rhs.0 - self.1 * rhs.1,
            self.0 * rhs.1 + self.1 * rhs.0,
        )
    }
}

impl ops::Div for &Complex {
    type Output = Complex;

    fn div(self, rhs: Self) -> Self::Output {
        self.scale(1. / (rhs.0 * rhs.0 + rhs.1 * rhs.1)) * rhs.conjugate()
    }
}

impl std::iter::Sum for Complex {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.reduce(|x, y| x + y).unwrap_or(Complex(0., 0.))
    }
}

// impl std::iter::Sum for &CAlg {
//     fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
//         // let a: CAlg = iter.reduce(|x, y| (*x) + (*y)).unwrap_or(CAlg(0., 0.));
//         // let mut res = CAlg(0., 0.);
//         // for i in iter {
//         //     res = res + *i;
//         // }
//         // &res
//     }
// }

impl Complex {
    pub fn magnitude(self: &Self) -> f64 {
        f64::sqrt(self.0 * self.0 + self.1 * self.1)
    }

    pub fn conjugate(self: &Self) -> Self {
        Complex(self.0, -self.1)
    }

    pub fn scale(self: &Self, s: f64) -> Self {
        Complex(self.0 * s, self.1 * s)
    }

    pub fn algebraic_to_polar(self: &Self) -> c_polar::CPolar {
        c_polar::CPolar(self.0 / self.magnitude(), self.1 / self.magnitude())
    }

    pub fn pow(self, e: i64) -> Complex {
        let mut res = Complex(1., 0.);

        if e == 0 {
            return res;
        }

        for _ in 0..(e.abs() as u64) {
            res = res * self;
        }
        if e > 0 {
            return res;
        } else {
            return Complex(1., 0.) / res;
        }
    }

    fn to_string(&self) -> String {
        format!("{} {}i", self.0, self.1)
    }
}
