// use std::ops;
// use std::ops::{Add, Mul};
// use std::slice;
//
// trait VectorSpace {
//     type Scalar;
//
//     fn add(self: &Self, rhs: &Self) -> Self
//     where
//         Self: Sized;
//
//     fn scalar_mult(self: &Self, scalar: &Self::Scalar) -> Self
//     where
//         Self: Sized;
// }

// trait Convertable<T> {
//     fn to_f64(self: &Self) -> f64;
// }

// pub struct VectorGen<T>(pub Vec<T>);
//
// impl<T: Clone> Iterator for VectorGen<T> {
//     type Item = T;
//
//     fn next(&mut self) -> Option<Self::Item> {
//         self.0.iter().next().cloned()
//     }
// }
//
// impl<T: Clone> Clone for VectorGen<T> {
//     fn clone(&self) -> Self {
//         Self(self.0.clone())
//     }
// }
//
// impl<T> FromIterator<T> for VectorGen<T> {
//     fn from_iter<T1: IntoIterator<Item = T>>(iter: T1) -> Self {
//         Self(Vec::from_iter(iter))
//     }
// }

// impl<T> ops::Neg for VectorGen<T>
// // where
// // T: From,
// {
//     type Output = Self;
//
//     fn neg(self) -> Self::Output {
//         self.scalar_mult(-1)
//     }
// }

// impl<T> VectorGen<T>
// where
//     for<'a> &'a T: Mul<f64, Output = T>,
//     // T: Mul<T, Output = f64>,
//     // T: Add<T, Output = f64>,
// {
//     pub fn iter(self: &Self) -> slice::Iter<'_, T> {
//         self.0.iter()
//     }
//
//     pub fn iter_mut(self: &mut Self) -> slice::IterMut<'_, T> {
//         self.0.iter_mut()
//     }
//
//     pub fn scalar_mult(self: &Self, s: f64) -> Self {
//         let mut result: Vec<T> = Vec::new();
//         for item in self.iter() {
//             result.push(item * s);
//         }
//         VectorGen(result)
//     }
//
//     // pub fn dot_product(v1: &Vector, v2: &Vector) -> f64 {
//     //     v1.iter().zip(v2.iter()).map(|(x, y)| x * y).sum()
//     // }
// }
