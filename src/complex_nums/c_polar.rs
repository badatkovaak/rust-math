pub struct CPolar(pub (f64, f64));

use crate::complex_nums::c_algebraic;
use std::ops;

impl ops::Neg for CPolar {
    type Output = CPolar;

    fn neg(self) -> Self::Output {
        CPolar((self.0 .0, -self.0 .1))
    }
}

impl ops::Mul for CPolar {
    type Output = CPolar;

    fn mul(self, rhs: Self) -> Self::Output {
        CPolar((self.0 .0 * rhs.0 .0, self.0 .1 + rhs.0 .1))
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
        let c = z.0 .1.sin_cos();
        c_algebraic::CAlgebraic((z.0 .0 * c.1, z.0 .0 * c.0))
    }
}
