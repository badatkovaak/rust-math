use std::ops;
use std::slice::{Iter, IterMut};

#[derive(Debug)]
pub struct Matrix(pub Vec<Vec<f64>>);

impl Iterator for Matrix {
    type Item = Vec<f64>;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.iter().next().cloned()
    }
}

impl Clone for Matrix {
    fn clone(&self) -> Self {
        return Self(self.0.clone());
    }
}

impl FromIterator<Vec<f64>> for Matrix {
    fn from_iter<T: IntoIterator<Item = Vec<f64>>>(iter: T) -> Self {
        Self(Vec::from_iter(iter))
    }
}

impl ops::Neg for Matrix {
    type Output = Matrix;

    fn neg(self) -> Self::Output {
        self.scalar_mult(-1.0)
    }
}

impl ops::Add for Matrix {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        self.iter()
            .zip(rhs.iter())
            .map(|(x, y)| x.iter().zip(y.iter()).map(|(x1, y1)| x1 + y1).collect())
            .collect()
    }
}

impl ops::Sub for Matrix {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        self + (-rhs)
    }
}

impl ops::Mul for Matrix {
    type Output = Option<Self>;

    fn mul(self, rhs: Self) -> Self::Output {
        if self.0[0].len() != rhs.0.len() {
            return None;
        }
        let mut res = vec![vec![0.0; self.0.len()]; rhs.0[0].len()];
        for i in 0..self.0.len() {
            for k in 0..rhs.0[0].len() {
                for j in 0..rhs.0.len() {
                    res[i][j] += self.0[i][k] * rhs.0[k][j];
                }
            }
        }
        return Some(Matrix(res));
    }
}

impl Matrix {
    pub fn iter(&self) -> Iter<'_, Vec<f64>> {
        self.0.iter()
    }

    pub fn iter_mut(&mut self) -> IterMut<'_, Vec<f64>> {
        self.0.iter_mut()
    }

    pub fn to_vec(&mut self) -> Matrix {
        Self(self.0.to_vec())
    }

    pub fn scalar_mult(self, s: f64) -> Matrix {
        self.iter()
            .map(|x| x.iter().map(|y| y * s).collect())
            .collect()
    }

    pub fn transpose(mut self) -> Matrix {
        for i in 0..self.0.len() {
            for j in 0..self.0[i].len() {
                self.0[i][j] = self.0[j][i];
            }
        }
        return self;
    }

    pub fn get_dimensions(&self) -> (usize, usize) {
        (self.0.len(), self.0[0].len())
    }

    pub fn elem_transform_1(self, i: u32, j: u32) -> Matrix {
        let cond = |k, l| {
            let c1 = k == l && k != j && k != i;
            let c2 = k == i && l == j;
            let c3 = k == j && l == i;
            (c1 || c2 || c3) as u32
        };
        let dims = self.get_dimensions();
        let elem_matrix: Vec<Vec<f64>> = vec![vec![0.0; dims.1]; dims.0]
            .iter_mut()
            .enumerate()
            .map(|(i, z)| {
                z.iter()
                    .enumerate()
                    .map(|(j, v)| v + cond(i as u32, j as u32) as f64)
                    .collect()
            })
            .collect();

        (self * Matrix(elem_matrix)).unwrap()
    }

    pub fn elem_transform_2(self, i: u32, j: u32, m: f64) -> Matrix {
        let cond = |k, l| match (k == l, k == i) {
            (true, true) => m,
            (true, false) => 1.0,
            (false, _) => 0.0,
        };
        let dims = self.get_dimensions();
        let elem_matrix: Vec<Vec<f64>> = vec![vec![0.0; dims.1]; dims.0]
            .iter_mut()
            .enumerate()
            .map(|(k, z)| {
                z.iter()
                    .enumerate()
                    .map(|(l, v)| v + cond(k as u32, l as u32) as f64)
                    .collect()
            })
            .collect();

        (self * Matrix(elem_matrix)).unwrap()
    }
    pub fn elem_transform_3(self, i: u32, j: u32, m: f64) -> Matrix {
        let cond = |k, l| match (k == l, k == i, l == j) {
            (_, true, true) => m,
            (true, _, _) => 1.0,
            (_, _, _) => 0.0,
        };
        let dims = self.get_dimensions();
        let elem_matrix: Vec<Vec<f64>> = vec![vec![0.0; dims.1]; dims.0]
            .iter_mut()
            .enumerate()
            .map(|(k, z)| {
                z.iter()
                    .enumerate()
                    .map(|(l, v)| v + cond(k as u32, l as u32) as f64)
                    .collect()
            })
            .collect();

        (self * Matrix(elem_matrix)).unwrap()
    }

    pub fn make_into_step_form(&mut self) -> Matrix {
        todo!()
    }

    // pub fn get_inverse(&mut self) -> Self {}

    pub fn append_row(mut self, row: Vec<f64>) -> Matrix {
        for i in 0..row.len() {
            self.0[i].push(row[i]);
        }
        return self;
    }
}

pub fn get_id_matrix(dim: u32) -> Matrix {
    let mut res = vec![vec![0.0; dim as usize]; dim as usize];
    for i in 0..dim {
        res[i as usize][i as usize] = 1.0;
    }
    return Matrix(res);
}

pub fn scalar_to_matrix(value: f64, dim: u32) -> Matrix {
    get_id_matrix(dim).scalar_mult(value)
}
