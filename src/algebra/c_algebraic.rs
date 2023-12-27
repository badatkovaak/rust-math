use crate::algebra::c_polar;
use crate::utils::fequals;
use std::cmp;
use std::ops;

#[derive(Debug, Clone, Copy)]
pub struct CAlg(pub f64, pub f64);

impl std::fmt::Display for CAlg {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} + {}i", self.0, self.1)
    }
}

impl cmp::PartialEq for CAlg {
    fn eq(&self, other: &Self) -> bool {
        fequals(self.0, other.0, 13) && fequals(self.1, other.1, 13)
    }
}

impl ops::Neg for CAlg {
    type Output = CAlg;
    fn neg(self) -> Self::Output {
        self.scale(-1.)
    }
}

impl ops::Neg for &CAlg {
    type Output = CAlg;
    fn neg(self) -> Self::Output {
        self.scale(-1.)
    }
}

impl ops::Add for CAlg {
    type Output = CAlg;

    fn add(self, rhs: Self) -> Self::Output {
        CAlg(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl ops::AddAssign for CAlg {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl ops::Sub for CAlg {
    type Output = CAlg;

    fn sub(self, rhs: Self) -> Self::Output {
        CAlg(self.0 - rhs.0, self.1 - rhs.1)
    }
}

impl ops::Mul for CAlg {
    type Output = CAlg;

    fn mul(self, rhs: Self) -> Self::Output {
        Self(
            self.0 * rhs.0 - self.1 * rhs.1,
            self.0 * rhs.1 + self.1 * rhs.0,
        )
    }
}

impl ops::Div for CAlg {
    type Output = CAlg;

    fn div(self, rhs: Self) -> Self::Output {
        self.scale(1. / (rhs.0 * rhs.0 + rhs.1 * rhs.1)) * rhs.conjugate()
    }
}

impl ops::Add for &CAlg {
    type Output = CAlg;

    fn add(self, rhs: Self) -> Self::Output {
        CAlg(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl ops::Sub for &CAlg {
    type Output = CAlg;

    fn sub(self, rhs: Self) -> Self::Output {
        CAlg(self.0 - rhs.0, self.1 - rhs.1)
    }
}

impl ops::Mul for &CAlg {
    type Output = CAlg;

    fn mul(self, rhs: Self) -> Self::Output {
        CAlg(
            self.0 * rhs.0 - self.1 * rhs.1,
            self.0 * rhs.1 + self.1 * rhs.0,
        )
    }
}

impl ops::Div for &CAlg {
    type Output = CAlg;

    fn div(self, rhs: Self) -> Self::Output {
        self.scale(1. / (rhs.0 * rhs.0 + rhs.1 * rhs.1)) * rhs.conjugate()
    }
}

impl std::iter::Sum for CAlg {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.reduce(|x, y| x + y).unwrap_or(CAlg(0., 0.))
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

impl CAlg {
    pub fn magnitude(self: &Self) -> f64 {
        f64::sqrt(self.0 * self.0 + self.1 * self.1)
    }

    pub fn conjugate(self: &Self) -> Self {
        CAlg(self.0, -self.1)
    }

    pub fn scale(self: &Self, s: f64) -> Self {
        CAlg(self.0 * s, self.1 * s)
    }

    pub fn algebraic_to_polar(self: &Self) -> c_polar::CPolar {
        c_polar::CPolar(self.0 / self.magnitude(), self.1 / self.magnitude())
    }

    pub fn pow(self, e: i64) -> CAlg {
        let mut res = CAlg(1., 0.);

        if e == 0 {
            return res;
        }

        for _ in 0..(e.abs() as u64) {
            res = res * self;
        }
        if e > 0 {
            return res;
        } else {
            return CAlg(1., 0.) / res;
        }
    }

    fn to_string(&self) -> String {
        format!("{} {}i", self.0, self.1)
    }
}
