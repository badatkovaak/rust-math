use std::ops;

use super::big_uint::BigUInt;
use crate::long::sign::Sign;

#[derive(Debug, Clone)]
pub struct BigInt(Sign, BigUInt);

// impl ops::Add for BigInt {
//     type Output = BigInt;
// }

// #[derive(Debug)]
// pub struct BigInt(Vec<u64>);
//
// impl std::fmt::Display for BigInt {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         return write!(f, "{}", self.to_decimal_string());
//     }
// }
//
// impl ops::Neg for BigInt {
//     type Output = BigInt;
//
//     fn neg(self) -> Self::Output {
//         BigInt(self.0.iter().map(|x| !x).collect()) + BigInt(vec![1])
//     }
// }

// impl ops::Add for BigInt {
//     type Output = BigInt;
//
//     fn add(self, rhs: Self) -> Self::Output {
//         let mut rem: u64 = 0;
//         let mut temp: u64;
//         let max_len = utils::max_of_two_usize(self.0.len(), rhs.0.len());
//         let mut res = BigInt(vec![0; max_len + 1]);
//         for i in 0..=max_len {
//             temp = *self.0.get(i).unwrap_or(&0) + *rhs.0.get(i).unwrap_or(&0) + rem;
//             // rem = temp / 256;
//             res.0[i] = temp as u64;
//             // println!("{} {} {}", temp, temp / 256, temp % 256);
//         }
//         return res;
//     }
// }

// impl ops::Add for &BigInt {
//     type Output = BigInt;
//
//     fn add(self, rhs: Self) -> Self::Output {
//         let mut rem: u16 = 0;
//         let mut temp: u16;
//         let max_len = utils::max_of_two_usize(self.0.len(), rhs.0.len());
//         let mut res = BigInt(vec![0; max_len + 1]);
//         for i in 0..=max_len {
//             temp = *self.0.get(i).unwrap_or(&0) as u16 + *rhs.0.get(i).unwrap_or(&0) as u16 + rem;
//             // rem = temp / 256;
//             // res.0[i] = (temp % 256) as u8;
//             // println!("{} {} {}", temp, temp / 256, temp % 256);
//         }
//         return res;
//     }
// }

// impl BigInt {
//     pub fn to_decimal_u(&self) -> u128 {
//         let mut res: u128 = 0;
//         for i in 0..self.0.len() {
//             res += (self.0[i] as u128) * (256 as u128).pow(i as u32);
//         }
//         res
//     }
//
//     pub fn to_decimal_string(&self) -> String {
//         // for i in self.iter() {
//         //     let mut x = i;
//         //     let res = String::new();
//         //     while
//         // }
//         return String::from("");
//     }
//
//     pub fn construct(s: String) -> Option<BigInt> {
//         let cond: bool = s
//             .chars()
//             .filter(|x| ((*x as u8) < 58) && ((*x as u8) > 47))
//             // .filter(|x| *x)
//             .collect::<Vec<char>>()
//             .len()
//             != s.len() + (s.get(0..1) == Some("-")) as usize;
//
//         if !s.is_ascii() || cond {
//             return None;
//         }
//
//         for i in ((s.get(0..1) == Some("-")) as usize)..s.len() {}
//         return Some(BigInt(vec![1]));
//         // for i in
//     }
// }
