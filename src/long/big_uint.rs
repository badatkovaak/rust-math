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

impl ops::Sub for BigUInt {
    type Output = Option<BigUInt>;

    fn sub(self, rhs: Self) -> Self::Output {
        let mut borrowed: u8 = 0;
        let mut temp1: BigDigit;
        let mut temp2: BigDigit;

        let max_len = max_of_two_usize(self.0.len(), rhs.0.len());

        let mut result = BigUInt(vec![BigDigit(0); max_len]);

        for i in 0..max_len {
            // println!("{:?}  {:?}", self.0[i], rhs.0[i]);
            if borrowed != 0 {
                borrowed -= 1;

                match self.0[i] - BigDigit(1) {
                    Some(x) => {
                        temp1 = x;
                    }
                    None => {
                        temp1 = BigDigit(MAX);
                        borrowed += 1;
                    }
                }
            } else {
                temp1 = self.0[i];
            }

            if temp1 < rhs.0[i] {
                borrowed += 1;
                temp2 = (BigDigit(MAX) - (rhs.0[i] - temp1).unwrap()).unwrap();
            } else {
                temp2 = (temp1 - rhs.0[i]).unwrap();
            }
            println!("{:?}  {:?}", temp1, temp2);

            result.0[i] = temp2;
        }

        if !(borrowed == 0) {
            return None;
        }

        return Some(result);
    }
}

// impl BigUInt {
//     pub fn convert_to_bc(self) -> String {
//
//     }
// }

#[derive(Debug)]
pub enum Operation {
    Plus,
    Minus,
}

// pub fn test_with_bc(n1: BigUInt, n2: BigUInt, op: Operation) -> bool {
//     fn call_bc(args: &str) -> String {
//         let output = Command::new("echo")
//             .arg(args.to_owned() + "| temp.txt")
//             .output().and_then(|x| Command::new("bc").args());
//         String::from("Hi")
//         // Command::new("bc").arg()
//     }
//     call_bc("10 + 20");
//     true
// }

#[cfg(test)]
mod tests {

    // #[test]
    // fn test_big_uint() {
    //     unimplemented!();
    // }
}
