use crate::complex_nums::c_polar;
use std::ops;

#[derive(Debug, Clone, Copy)]
pub struct CAlgebraic(pub f64, pub f64);

impl std::fmt::Display for CAlgebraic {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} + {}i", self.0, self.1)
    }
}

impl ops::Neg for CAlgebraic {
    type Output = CAlgebraic;
    fn neg(self) -> Self::Output {
        self.scale(-1.)
    }
}

impl ops::Add for CAlgebraic {
    type Output = CAlgebraic;

    fn add(self, rhs: Self) -> Self::Output {
        CAlgebraic(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl ops::Sub for CAlgebraic {
    type Output = CAlgebraic;

    fn sub(self, rhs: Self) -> Self::Output {
        CAlgebraic(self.0 - rhs.0, self.1 - rhs.1)
    }
}

impl ops::Mul for CAlgebraic {
    type Output = CAlgebraic;

    fn mul(self, rhs: Self) -> Self::Output {
        Self(
            self.0 * rhs.0 - self.1 * rhs.1,
            self.0 * rhs.1 + self.1 * rhs.0,
        )
    }
}

impl ops::Div for CAlgebraic {
    type Output = CAlgebraic;

    fn div(self, rhs: Self) -> Self::Output {
        self.scale(1. / (rhs.0 * rhs.0 + rhs.1 * rhs.1)) * rhs.conjugate()
    }
}

impl CAlgebraic {
    pub fn magnitude(self: &Self) -> f64 {
        f64::sqrt(self.0 * self.0 + self.1 * self.1)
    }

    pub fn conjugate(self: &Self) -> Self {
        CAlgebraic(self.0, -self.1)
    }

    pub fn scale(self: &Self, s: f64) -> Self {
        CAlgebraic(self.0 * s, self.1 * s)
    }

    pub fn algebraic_to_polar(self: &Self) -> c_polar::CPolar {
        c_polar::CPolar(self.0 / self.magnitude(), self.1 / self.magnitude())
    }

    pub fn pow(self, e: i64) -> CAlgebraic {
        let mut res = CAlgebraic(1., 0.);

        if e == 0 {
            return res;
        }

        for i in 0..(e.abs() as u64) {
            res = res * self;
        }
        res
    }
}
