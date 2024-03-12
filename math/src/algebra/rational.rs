#[derive(Debug, Clone, Copy, Eq)]
pub struct Rational(pub i64, pub i64);

use std::cmp::{Eq, Ord, PartialEq, PartialOrd};
use std::ops::{Add, AddAssign, Div, Mul, Neg, Sub};
use Rational as R;

use crate::utils::{gcd_euclid, lcm_euclid};

impl std::fmt::Display for R {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}/{}", self.0, self.1)
    }
}

impl PartialEq for R {
    fn eq(&self, other: &Self) -> bool {
        let x1 = self.simplify();
        let x2 = other.simplify();
        x1.0 == x2.0 && x1.1 == x2.1
    }
}

impl Neg for R {
    type Output = R;

    fn neg(self) -> Self::Output {
        self.scale_by(-1)
    }
}

impl Add for R {
    type Output = R;

    fn add(self, rhs: Self) -> Self::Output {
        // println!("Add : {} {}", self, rhs);
        let lcm = lcm_euclid(self.1 as u64, rhs.1 as u64) as i64;
        R(self.0 * (self.1 / lcm) + rhs.0 * (rhs.1 / lcm), lcm)
    }
}

impl AddAssign for R {
    fn add_assign(&mut self, rhs: Self) {
        // println!("AddAss : {} {}", self, rhs);
        let lcm = lcm_euclid(self.1 as u64, rhs.1 as u64) as i64;
        self.0 = self.0 * (self.1 / lcm) + rhs.0 * (rhs.1 / lcm);
        self.1 = lcm;
    }
}

impl Sub for R {
    type Output = R;

    fn sub(self, rhs: Self) -> Self::Output {
        self + (-rhs)
    }
}

impl Mul for R {
    type Output = R;

    fn mul(self, rhs: Self) -> Self::Output {
        R(self.0 * rhs.0, self.1 * rhs.1)
    }
}

impl Div for R {
    type Output = R;

    fn div(self, rhs: Self) -> Self::Output {
        self * rhs.inverse()
    }
}

impl R {
    pub fn scale_by(&self, s: i64) -> R {
        R(self.0 * s, self.1)
    }

    pub fn inverse(&self) -> R {
        R(self.1, self.0)
    }

    pub fn simplify(self) -> R {
        let gcd = gcd_euclid(self.0 as u64, self.1 as u64) as i64;
        if gcd == 1 {
            return self;
        }
        R(self.0 / gcd, self.1 / gcd)
    }
}
