use crate::complex_nums::c_polar;
use std::ops;

pub struct CAlgebraic(pub (f64, f64));

impl ops::Neg for CAlgebraic {
    type Output = CAlgebraic;
    fn neg(self) -> Self::Output {
        Self((self.0 .0 * (-1.0), self.0 .1 * (-1.0)))
        // self.
    }
}

impl ops::Add for CAlgebraic {
    type Output = CAlgebraic;

    fn add(self, rhs: Self) -> Self::Output {
        CAlgebraic((self.0 .0 + rhs.0 .0, self.0 .1 + self.0 .1))
    }
}

impl ops::Sub for CAlgebraic {
    type Output = CAlgebraic;

    fn sub(self, rhs: Self) -> Self::Output {
        self + (-rhs)
    }
}

impl ops::Mul for CAlgebraic {
    type Output = CAlgebraic;

    fn mul(self, rhs: Self) -> Self::Output {
        Self((
            self.0 .0 * rhs.0 .0 - self.0 .1 * rhs.0 .1,
            self.0 .0 * rhs.0 .1 + self.0 .1 * rhs.0 .0,
        ))
    }
}

impl ops::Div for CAlgebraic {
    type Output = CAlgebraic;

    fn div(self, rhs: Self) -> Self::Output {
        self * (CAlgebraic((rhs.0 .0, -rhs.0 .1)))
    }
}

pub fn conjugate(c: &CAlgebraic) -> CAlgebraic {
    return CAlgebraic((c.0 .0, -c.0 .1));
}

pub fn algebraic_to_polar(c: &CAlgebraic) -> c_polar::CPolar {
    let r = f64::sqrt(c.0 .0 * c.0 .0 + c.0 .1 * c.0 .1);
    return c_polar::CPolar((c.0 .0 / r, c.0 .1 / r));
}

// pub fn polar_to_algebraic(c: &ComplexPolar) -> ComplexAlg {
//     return ();
// }
