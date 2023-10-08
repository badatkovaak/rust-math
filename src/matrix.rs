use std::slice::{Iter, IterMut};

#[derive(Debug)]
pub struct Matrix(Vec<Vec<f64>>);

impl Iterator for Matrix {
    type Item = Vec<f64>;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.iter().next().cloned()
    }
}

impl FromIterator<Vec<f64>> for Matrix {
    fn from_iter<T: IntoIterator<Item = Vec<f64>>>(iter: T) -> Self {
        Self(Vec::from_iter(iter))
    }
}

impl std::ops::Neg for Matrix {
    type Output = Matrix;

    fn neg(self) -> Self::Output {
        self.scalar_mult(-1.0)
    }
}

impl std::ops::Add for Matrix {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        // return self::Matrix::myadd(self, rhs);
        self.iter()
            .zip(rhs.iter())
            .map(|(x, y)| x.iter().zip(y.iter()).map(|(x1, y1)| x1 + y1).collect())
            .collect()
    }
}

impl std::ops::Sub for Matrix {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        self + (-rhs)
    }
}

impl std::ops::Mul for Matrix {
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

// impl std::ops::Div for Matrix {
//     type Output = Option<Self>;
//
//     fn div(self, rhs: Self) -> Self::Output {}
// }

impl Matrix {
    pub fn iter(&self) -> Iter<'_, Vec<f64>> {
        self.0.iter()
    }

    pub fn iter_mut(&mut self) -> IterMut<'_, Vec<f64>> {
        self.0.iter_mut()
    }

    pub fn to_vec(&mut self) -> Self {
        Self(self.0.to_vec())
    }

    pub fn scalar_mult(self, s: f64) -> Matrix {
        self.iter()
            .map(|x| x.iter().map(|y| y * s).collect())
            .collect()
    }

    // pub fn myadd(self, m2: Matrix) -> Matrix {
    //     return self
    //         .iter()
    //         .zip(m2.iter())
    //         .map(|(x, y)| x.iter().zip(y.iter()).map(|(x1, y1)| x1 + y1).collect())
    //         .collect();

    pub fn transpose(mut self) -> Matrix {
        for i in 0..self.0.len() {
            for j in 0..self.0[i].len() {
                self.0[i][j] = self.0[j][i];
            }
        }
        return self;
    }

    pub fn make_into_step_form(&mut self) -> Matrix {
        todo!()
        // return *m;
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
