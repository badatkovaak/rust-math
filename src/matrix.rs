use std::slice::{Iter, IterMut};

pub type Matrix2 = [[f64; 2]; 2];
// pub type Matrix3 = [[f64; 3]; 3];
// pub type Matrix = Vec<Vec<f64>>;
pub struct Matrix(Vec<Vec<f64>>);

impl Iterator for Matrix {
    type Item = Vec<f64>;
    fn next(&mut self) -> Option<Self::Item> {
        return self.0.iter().next().cloned();
    }
}

impl FromIterator<Vec<f64>> for Matrix {
    fn from_iter<T: IntoIterator<Item = Vec<f64>>>(iter: T) -> Self {
        return Self(Vec::from_iter(iter));
    }
}

impl std::ops::Add for Matrix {
    type Output = Matrix;

    fn add(self, rhs: Self) -> Self::Output {
        return self::Matrix::myadd(self, rhs);
    }
}

impl Matrix {
    pub fn iter(&self) -> Iter<'_, Vec<f64>> {
        return self.0.iter();
    }

    pub fn iter_mut(&mut self) -> IterMut<'_, Vec<f64>> {
        return self.0.iter_mut();
    }

    pub fn to_vec(&mut self) -> Self {
        return Self(self.0.to_vec());
    }

    pub fn scalar_mult(self, s: f64) -> Matrix {
        return self
            .iter()
            .map(|x| x.iter().map(|y| y * s).collect())
            .collect();
    }

    pub fn scalar_mult2(m: &Matrix2, s: f64) -> Matrix2 {
        return m.map(|x| x.map(|y| y * s));
    }

    pub fn add2(m1: &Matrix2, m2: &Matrix2) -> Matrix2 {
        return m1
            .iter()
            .zip(m2.iter())
            .map(|(x, y)| {
                x.iter()
                    .zip(y.iter())
                    .map(|(x1, y1)| x1 + y1)
                    .collect::<Vec<f64>>()
                    .try_into()
                    .unwrap()
            })
            .collect::<Vec<[f64; 2]>>()
            .try_into()
            .unwrap();
    }

    pub fn myadd(self, m2: Matrix) -> Matrix {
        return self
            .iter()
            .zip(m2.iter())
            .map(|(x, y)| x.iter().zip(y.iter()).map(|(x1, y1)| x1 + y1).collect())
            .collect();
    }

    pub fn make_into_step_form(m: &mut Matrix) -> Matrix {
        todo!()
        // return *m;
    }

    pub fn append_row(m: &mut Matrix, row: Vec<f64>) -> Matrix {
        for i in m.iter_mut() {
            i.push(row[0]);
            // <Vec<f64> as AsMut<Vec<f64>>>::as_mut(i);
        }
        return m.to_vec();
    }
}
//
// pub fn scalar_mult(m: &Matrix, s: f64) -> Matrix {
//     return m
//         .iter()
//         .map(|x| x.iter().map(|y| y * s).collect())
//         .collect();
// }
//
// pub fn scalar_mult2(m: &Matrix2, s: f64) -> Matrix2 {
//     return m.map(|x| x.map(|y| y * s));
// }
//
// pub fn add2(m1: &Matrix2, m2: &Matrix2) -> Matrix2 {
//     return m1
//         .iter()
//         .zip(m2.iter())
//         .map(|(x, y)| {
//             x.iter()
//                 .zip(y.iter())
//                 .map(|(x1, y1)| x1 + y1)
//                 .collect::<Vec<f64>>()
//                 .try_into()
//                 .unwrap()
//         })
//         .collect::<Vec<[f64; 2]>>()
//         .try_into()
//         .unwrap();
// }
//
// pub fn add(m1: Matrix, m2: Matrix) -> Matrix {
//     return m1
//         .iter()
//         .zip(m2.iter())
//         .map(|(x, y)| x.iter().zip(y.iter()).map(|(x1, y1)| x1 + y1).collect())
//         .collect();
// }
//
// pub fn make_into_step_form(m: &mut Matrix) -> Matrix {
//     todo!()
//     // return *m;
// }
//
// pub fn append_row(m: &mut Matrix, row: Vec<f64>) -> Matrix {
//     for i in m.iter_mut() {
//         i.push(row[0]);
//         // <Vec<f64> as AsMut<Vec<f64>>>::as_mut(i);
//     }
//     return m.to_vec();
// }
