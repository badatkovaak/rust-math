use std::cmp;
use std::ops;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct BigDigit(pub u64);

// impl cmp::Eq

impl ops::Add for BigDigit {
    type Output = (BigDigit, bool);

    fn add(self, rhs: Self) -> Self::Output {
        let x = self.0.overflowing_add(rhs.0);
        return (BigDigit(x.0), x.1);
    }
}

impl ops::Sub for BigDigit {
    type Output = Option<BigDigit>;

    fn sub(self, rhs: Self) -> Self::Output {
        let x = self.0.checked_sub(rhs.0);

        match x {
            Some(a) => Some(BigDigit(a)),
            None => None,
        }
    }
}

impl ops::Mul for BigDigit {
    type Output = (BigDigit, bool);

    fn mul(self, rhs: Self) -> Self::Output {
        let x = self.0.overflowing_mul(rhs.0);
        return (BigDigit(x.0), x.1);
    }
}
