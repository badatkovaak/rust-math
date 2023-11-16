#[derive(Debug, Clone, Copy)]
pub struct CPolar(pub f64, pub f64);

use crate::complex_nums::c_algebraic;
use std::ops;

impl ops::Neg for CPolar {
    type Output = CPolar;

    fn neg(self) -> Self::Output {
        CPolar(self.0, -self.1)
    }
}

impl ops::Mul for CPolar {
    type Output = CPolar;

    fn mul(self, rhs: Self) -> Self::Output {
        CPolar(self.0 * rhs.0, self.1 + rhs.1)
    }
}

impl ops::Div for CPolar {
    type Output = CPolar;

    fn div(self, rhs: Self) -> Self::Output {
        self * (-rhs)
    }
}

impl CPolar {
    fn polar_to_algebraic(z: CPolar) -> c_algebraic::CAlgebraic {
        let c = z.1.sin_cos();
        c_algebraic::CAlgebraic(z.0 * c.1, z.0 * c.0)
    }
}
