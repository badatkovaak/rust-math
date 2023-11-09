use crate::{arithmetic::big_digit::BigDigit, utils::max_of_two_usize};
use std::cmp;
use std::cmp::Ordering;
use std::iter::zip;
use std::ops;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BigUInt(pub Vec<BigDigit>);

impl cmp::PartialOrd for BigUInt {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.0.len() > other.0.len() {
            return Some(Ordering::Greater);
        } else if self.0.len() < other.0.len() {
            return Some(Ordering::Less);
        }

        for i in zip(self.0.iter(), other.0.iter()) {
            match i.0.partial_cmp(i.1) {
                Some(Ordering::Equal) => continue,
                Some(o) => Some(o),
                None => None,
            };
        }

        return Some(Ordering::Equal);
    }
}

impl ops::Add for BigUInt {
    type Output = BigUInt;

    fn add(self, rhs: Self) -> Self::Output {
        let mut carry = BigDigit(0);
        let mut temp1: (BigDigit, bool);
        let mut temp2: (BigDigit, bool);
        let max_len = max_of_two_usize(self.0.len(), rhs.0.len());
        let mut result = BigUInt(vec![BigDigit(0); max_len]);

        for i in 0..max_len {
            temp1 = self.0.get(i..i + 1).unwrap_or(&[BigDigit(0)])[0] + carry;
            temp2 = temp1.0 + rhs.0.get(i..i + 1).unwrap_or(&[BigDigit(0)])[0];
            carry = BigDigit(1 * ((temp2.1 as u64) + (temp1.1 as u64)));
            result.0[i] = temp2.0;
        }

        return result;
    }
}

impl ops::Sub for BigUInt {
    type Output = Option<BigUInt>;

    fn sub(self, rhs: Self) -> Self::Output {
        let borrow = BigDigit(0);
        let mut temp1: (BigDigit, bool);
        let mut temp2: (BigDigit, bool);
        let max_len = max_of_two_usize(self.0.len(), rhs.0.len());
        let mut result = BigUInt(vec![BigDigit(0); max_len]);

        for i in 0..max_len {
            // temp1 =
            // temp2 = temp1
        }

        return Some(result);
    }
}
