use crate::{long::big_digit::BigDigit, utils::max_of_two_usize};
use std::cmp;
use std::cmp::Ordering;
use std::iter::zip;
use std::ops;
use std::process::Command;
use std::u64::MAX;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BigUInt(pub Vec<BigDigit>);

impl cmp::PartialOrd for BigUInt {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.0.len() > other.0.len() {
            return Some(Ordering::Greater);
        } else if self.0.len() < other.0.len() {
            return Some(Ordering::Less);
        }

        for i in zip(self.0.iter(), other.0.iter()).rev() {
            match i.0.partial_cmp(i.1) {
                Some(Ordering::Equal) => continue,
                Some(o) => {
                    return Some(o);
                }
                None => {
                    return None;
                }
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

macro_rules! sub_for_BigUInt {
    ($x:ty) => {
        impl ops::Sub for $x {
            type Output = Option<BigUInt>;

            fn sub(self, rhs: Self) -> Self::Output {
                let mut borrowed: u8 = 0;
                let mut d1: BigDigit;
                let mut d2: BigDigit;
                let mut temp1: BigDigit;
                let mut temp2: BigDigit;

                let max_len = max_of_two_usize(self.0.len(), rhs.0.len());

                let mut result = BigUInt(vec![BigDigit(0); max_len]);

                for i in 0..max_len {
                    d1 = self.0.get(i..i + 1).unwrap_or(&[BigDigit(0)])[0];
                    d2 = rhs.0.get(i..i + 1).unwrap_or(&[BigDigit(0)])[0];

                    // println!("{:?}  {:?}", self.0[i], rhs.0[i]);
                    if borrowed != 0 {
                        borrowed -= 1;

                        match d1 - BigDigit(1) {
                            Some(x) => {
                                temp1 = x;
                            }
                            None => {
                                temp1 = BigDigit(MAX);
                                borrowed += 1;
                            }
                        }
                    } else {
                        temp1 = d1;
                        // temp1 = self.0.get(i..i + 1).unwrap_or(&[BigDigit(0)]);
                    }

                    if temp1 < d2 {
                        borrowed += 1;
                        temp2 = ((BigDigit(MAX) - (d2 - temp1).unwrap()).unwrap() + BigDigit(1)).0;
                    } else {
                        temp2 = (temp1 - d2).unwrap();
                    }
                    println!("{:?}  {:?}", temp1, temp2);

                    result.0[i] = temp2;
                }

                if !(borrowed == 0) {
                    return None;
                }

                for i in (1..max_len).rev() {
                    if result.0[i] == BigDigit(0) {
                        // let v1 = vec![1, 2, 3];
                        // let r1 = result.0[] ;
                        result.0.pop();
                    } else {
                        break;
                    }
                }
                return Some(result);
            }
        }
    };
}

sub_for_BigUInt!(BigUInt);
sub_for_BigUInt!(&BigUInt);

impl ops::Mul for BigUInt {
    type Output = BigUInt;

    fn mul(self, rhs: Self) -> Self::Output {
        let mut d1: BigDigit;
        let mut d2: BigDigit;
        let max_len = max_of_two_usize(self.0.len(), rhs.0.len());
        let mut res = BigUInt(vec![BigDigit(0); max_len]);

        for i in (0..max_len - 1) {
            // d1 =
            // res +=
        }

        // for
        // // for
        res
    }
}

// impl BigUInt {
//     pub fn mult_by_digit(self: &Self, d: BigDigit) -> BigUInt {
//         let len = self.0.len();
//         let mut temp1: BigDigit;
//         // let mut temp1: (BigDigit, u64);
//         let mut temp2: (BigDigit, bool);
//         let mut res = BigUInt(vec![BigDigit(0); len]);
//         let mut carry: u128 = 0;
//
//         for i in 0..len {
//             // (temp1, carry) = res.0[i] * d;
//             // temp2 = temp1.0 + carry;
//             // carry += temp2.1 as u128;
//             // res.0[i] = temp2.0;
//             // (res[i], carry) = (temp.0 + BigDigit(carry))
//         }
//
//         if carry != 0 {}
//
//         res
//     }
// }

#[derive(Debug)]
pub enum Operation {
    Plus,
    Minus,
}

#[cfg(test)]
mod tests {
    use std::u64::MAX;

    use crate::long::big_uint::{BigDigit, BigUInt};

    #[test]
    fn test_sub() {
        let n1: BigUInt = BigUInt(vec![BigDigit(1)]);
        let n2: BigUInt = BigUInt(vec![BigDigit(2)]);
        let n3: BigUInt = BigUInt(vec![BigDigit(3)]);
        let n4: BigUInt = BigUInt(vec![BigDigit(1), BigDigit(1)]);
        let n5: BigUInt = BigUInt(vec![BigDigit(1), BigDigit(2)]);
        let n6: BigUInt = BigUInt(vec![BigDigit(2), BigDigit(3)]);
        let n7: BigUInt = BigUInt(vec![BigDigit(2), BigDigit(1)]);
        let n8: BigUInt = BigUInt(vec![BigDigit(2), BigDigit(1), BigDigit(1)]);
        let n9: BigUInt = BigUInt(vec![BigDigit(3), BigDigit(2), BigDigit(1)]);
        let n10: BigUInt = BigUInt(vec![BigDigit(1), BigDigit(1), BigDigit(2)]);
        // let N11: BigUInt = BigUInt(vec![BigDigit(1), BigDigit(1), BigDigit(1)]);
        // let N12: BigUInt = BigUInt(vec![BigDigit(1), BigDigit(1), BigDigit(1)]);
        // let N13: BigUInt = BigUInt(vec![BigDigit(1), BigDigit(1), BigDigit(1)]);
        // let N14: BigUInt = BigUInt(vec![BigDigit(1), BigDigit(1), BigDigit(1)]);
        // let N15: BigUInt = BigUInt(vec![BigDigit(1), BigDigit(1), BigDigit(1)]);

        let res1 = &n2 - &n1;
        let res2 = &n3 - &n2;
        let res3 = &n4 - &n3;
        let res4 = &assert_eq!(res1, Some(BigUInt(vec![BigDigit(1)])));
        assert_eq!(res2, Some(BigUInt(vec![BigDigit(MAX)])));
        assert_eq!(res3, Some(BigUInt(vec![BigDigit(0), BigDigit(1)])));
        // unimplemented!();
    }
}
