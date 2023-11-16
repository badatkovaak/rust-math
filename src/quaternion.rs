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

// };
