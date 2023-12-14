// use mymath::utils;
use crate::utils;
use std::ops;
use std::slice;

pub struct Vector(pub Vec<f64>);

impl Iterator for Vector {
    type Item = f64;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.iter().next().cloned()
    }
}

impl Clone for Vector {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl FromIterator<f64> for Vector {
    fn from_iter<T: IntoIterator<Item = f64>>(iter: T) -> Self {
        Self(Vec::from_iter(iter))
    }
}

impl ops::Neg for Vector {
    type Output = Self;

    fn neg(self) -> Self::Output {
        self.scalar_mult(-1.0)
    }
}

impl ops::Add for Vector {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        self.iter().zip(rhs.iter()).map(|(x, y)| x + y).collect()
    }
}

impl ops::Mul for Vector {
    type Output = f64;

    fn mul(self, rhs: Self) -> Self::Output {
        self.iter().zip(rhs.iter()).map(|(x, y)| x * y).sum()
    }
}

impl Vector {
    pub fn iter(self: &Self) -> slice::Iter<'_, f64> {
        self.0.iter()
    }

    pub fn iter_mut(self: &mut Self) -> slice::IterMut<'_, f64> {
        self.0.iter_mut()
    }
    pub fn scalar_mult(self: &Self, s: f64) -> Vector {
        let mut result: Vec<f64> = Vec::new();
        for item in self.iter() {
            result.push(item * s as f64);
        }
        Vector(result)
    }

    pub fn get_length(self: &Self) -> f64 {
        f64::sqrt(self.iter().map(|x| x * x).sum())
    }

    pub fn normalize(self: &Self) -> Vector {
        Self::scalar_mult(self, utils::one_over_sqrt(self.get_length()))
    }
}

#[cfg(test)]
mod tests {
    // use super::*;
    // use crate::utils::fequals;

    #[test]
    fn test_get_length() {
        // let v1 = Vector(vec![1.]);
        // let v2 = Vector(vec![1., 2.]);
        // let v3 = Vector(vec![1., 2., 3.]);
        // let v4 = Vector(vec![1., 2., 3., 4.]);
        // let v5 = Vector(vec![1., 2., 3., 4., 5.]);

        // assert_eq!(fequals(v1.get_length(), 1.0000000, None), true);
        // assert_eq!(fequals(v2.get_length(), 2.2360679, None), true);
        // assert_eq!(fequals(v3.get_length(), 3.7416573, None), true);
        // assert_eq!(fequals(v4.get_length(), 5.4772255, None), true);
        // assert_eq!(fequals(v5.get_length(), 7.4161984, None), true);
    }
}
