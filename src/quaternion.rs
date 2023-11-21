pub(crate) use std::ops;

pub struct Quaternion(pub f64, pub f64, pub f64, pub f64);

impl std::fmt::Display for Quaternion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} + {}i + {}j + {}k", self.0, self.1, self.2, self.3)
    }
}

impl ops::Neg for Quaternion {
    type Output = Quaternion;

    fn neg(self) -> Self::Output {
        Quaternion(-self.0, -self.1, -self.2, -self.3)
    }
}

impl ops::Add for Quaternion {
    type Output = Quaternion;

    fn add(self, rhs: Self) -> Self::Output {
        Quaternion(
            self.0 + rhs.0,
            self.1 + rhs.1,
            self.2 + rhs.2,
            self.3 + rhs.3,
        )
    }
}

// impl ops::Sub for Quaternion {
//   type Output = Quaternion;
//

impl ops::Sub for Quaternion {
    type Output = Quaternion;

    fn sub(self, rhs: Self) -> Self::Output {
        Quaternion(
            self.0 - rhs.0,
            self.1 - rhs.1,
            self.2 - rhs.2,
            self.3 - rhs.3,
        )
    }
}

impl ops::Mul for Quaternion {
    type Output = Quaternion;

    fn mul(self, rhs: Self) -> Self::Output {
        Quaternion(
            self.0*rhs.0 - self.1*rhs.1 - self.2*rhs.2 - self.3*rhs.3,
            self.0*rhs.1 + self.1*rhs.0 + self.2*rhs.3 -self.3*rhs.2,
            self.0*rhs.2-self.1*rhs.3 + self.2*rhs.0 + self.3*rhs.1,
            self.0*rhs.3 + self.1*rhs.2 - self.2*rhs.1 + self.3*rhs.0
        )
    }
}

impl ops::Div for Quaternion {
    type Output = Quaternion;

    fn div(self, rhs: Self) -> Self::Output {
        let r = rhs.conjugate().scalar_mult( 1.0 / (rhs.0*rhs.0 + rhs.1*rhs.1+rhs.2*rhs.2+rhs.3*rhs.3));
        self*r
    }
}

impl Quaternion {
    pub fn modulus(self: &Self) -> f64 {
        f64::sqrt(self.0 * self.0 + self.1*self.1 + self.2*self.2 + self.3*self.3)
    }

    pub fn conjugate(self: &Self) -> Quaternion {
        Quaternion(self.0,-self.1, -self.2, -self.3)
    }

    pub fn scalar_mult(self, s: f64) -> Quaternion {
        Quaternion(self.0 * s, self.1*s, self.2 * s, self.3 * s)
    }
}

// };
