use crate::utils;
use std::cmp::Ordering;
use std::iter::zip;
use std::ops;
use std::slice::{Iter, IterMut};

#[derive(Debug)]
pub struct Matrix(pub Vec<Vec<f64>>);

impl std::fmt::Display for Matrix {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

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

impl std::cmp::PartialEq for Matrix {
    fn eq(&self, other: &Self) -> bool {
        for i in self.0.iter().zip(other.0.iter()) {
            for j in 0..i.0.len() {
                if !utils::fequals(i.0[j], i.1[j], Some(0.0000000001)) {
                    return false;
                }
            }
        }
        return true;
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
        // self.iter()
        //     .zip(rhs.iter())
        //     .map(|(x, y)| x.iter().zip(y.iter()).map(|(x1, y1)| x1 + y1).collect())
        //     .collect()
        zip(self.iter(), rhs.iter())
            .map(|(x, y)| zip(x.iter(), y.iter()).map(|(x, y)| x + y).collect())
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
        let elem_matrix: Matrix = Matrix(
            vec![vec![0.0; dims.1]; dims.0]
                .iter_mut()
                .enumerate()
                .map(|(k, z)| {
                    z.iter()
                        .enumerate()
                        .map(|(l, v)| v + cond(k as u32, l as u32) as f64)
                        .collect()
                })
                .collect(),
        );
        // println!("{}", elem_matrix);
        (elem_matrix * self).unwrap()
    }

    pub fn elem_transform_2(self, i: u32, m: f64) -> Matrix {
        let cond = |k, l| match (k == l, k == i) {
            (true, true) => m,
            (true, false) => 1.0,
            (false, _) => 0.0,
        };
        let dims = self.get_dimensions();
        let elem_matrix: Matrix = Matrix(
            vec![vec![0.0; dims.1]; dims.0]
                .iter_mut()
                .enumerate()
                .map(|(k, z)| {
                    z.iter()
                        .enumerate()
                        .map(|(l, v)| v + cond(k as u32, l as u32) as f64)
                        .collect()
                })
                .collect(),
        );
        // println!("{}", elem_matrix);
        (elem_matrix * self).unwrap()
    }

    // #[inline]
    pub fn elem_transform_3(self, i: u32, j: u32, m: f64) -> Matrix {
        let cond = |k, l| match (k == l, k == i, l == j) {
            (_, true, true) => m,
            (true, _, _) => 1.0,
            (_, _, _) => 0.0,
        };
        let dims = self.get_dimensions();
        let elem_matrix: Matrix = Matrix(
            vec![vec![0.0; dims.1]; dims.0]
                .iter_mut()
                .enumerate()
                .map(|(k, z)| {
                    z.iter()
                        .enumerate()
                        .map(|(l, v)| v + cond(k as u32, l as u32) as f64)
                        .collect()
                })
                .collect(),
        );
        // println!("{}", elem_matrix);
        (elem_matrix * self).unwrap()
    }

    // #[inline]
    pub fn count_elems(self: &Matrix) -> u64 {
        println!("{}", self);
        self.iter()
            .map(|x| {
                x.iter()
                    .map(|y| (!utils::fequals(*y, 0.0, None) as u64) * 1)
                    .sum::<u64>()
            })
            .sum::<u64>()
    }

    pub fn make_into_step_form(&mut self) -> Matrix {
        if self.count_elems() <= 1 {
            return self.clone();
        }

        let mut res = self.clone();

        // println!("{}", res);

        let mut i: u32 = 0;
        while res.0[0][0] == 0.0 {
            // println!("{}", i);
            res = res.elem_transform_1(0, i as u32);
            i += 1;
            if i as usize >= res.0.len() {
                return self.stitch(&(&mut res.cut(0, 1)).make_into_step_form());
            }
        }

        if res.0[0][0] != 1.0 {
            let v = res.0[0][0];
            res = res.elem_transform_2(0, 1.0 / v);
        }

        // println!("{}", res);
        for i in 1..res.0.len() {
            let v = res.0[i][0];
            // println!("{}", v);
            res = res.elem_transform_3(i as u32, 0, -v);
        }
        // println!("{}", res);

        return res.stitch(&(&mut res.cut(1, 1)).make_into_step_form());

        // return get_id_matrix(3);
    }

    pub fn stitch(&self, m: &Matrix) -> Matrix {
        let mut res = self.clone();
        let (d1, d2) = self.get_dimensions();
        for i in 1..d1 {
            for j in 1..d2 {
                res.0[i][j] = m.0[i - 1][j - 1];
            }
        }
        res
    }

    pub fn cut(&self, rows: u32, columns: u32) -> Matrix {
        let mut res = self.clone();
        for _ in 0..rows {
            res.0.push(vec![0.0; res.0[0].len()]);
            res.0.remove(0);
        }
        for i in 0..res.0.len() {
            for _ in 0..columns {
                res.0[i].remove(0);
                res.0[i].push(0.0);
            }
        }
        res
    }

    pub fn cut_mut(mut self, rows: u32, columns: u32) -> Matrix {
        for _ in 0..rows {
            self.0.remove(0);
        }
        for (i, _) in self.iter().enumerate() {
            for _ in 0..columns {
                self.0[i][0];
            }
        }
        self
    }

    pub fn append_row(mut self, row: Vec<f64>) -> Matrix {
        for i in 0..row.len() {
            self.0[i].push(row[i]);
        }
        self
    }

    pub fn to_string(&self) -> String {
        self.0
            .iter()
            .map(|x| {
                x.iter()
                    .map(|y| format!("{}", y))
                    .fold(String::from(" "), |acc, y| acc + &y + " ")
            })
            .fold(String::from(" "), |acc, y| acc + &y + "\n ")
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

#[cfg(test)]
mod tests {
    use crate::matrix::Matrix;

    #[test]
    fn test_matrix_operations() {
        let m1 = Matrix(vec![vec![1., 2.], vec![3., 4.]]);

        assert_eq!(-m1, Matrix(vec![vec![-1., -2.], vec![-3., -4.]]));
    }

    #[test]
    fn elementary_transformations() {
        let m1 = Matrix(vec![vec![1., 2.], vec![3., 4.]]);
        let m2 = Matrix(vec![vec![1., 2., 3.], vec![4., 5., 6.], vec![7., 8., 9.]]);
        // println!("{}", m1);

        assert_eq!(
            m2.elem_transform_1(0, 1),
            Matrix(vec![vec![4., 5., 6.], vec![1., 2., 3.], vec![7., 8., 9.]])
        );
        assert_eq!(
            m1.elem_transform_1(0, 1),
            Matrix(vec![vec![3., 4.], vec![1., 2.]])
        );
    }

    #[test]
    fn step_form() {
        let mut m1 = Matrix(vec![vec![1., 2., 3.], vec![4., 5., 6.], vec![7., 8., 9.]]);

        let mut m2 = Matrix(vec![vec![2., 3., 4.], vec![4., 5., 6.], vec![7., 8., 9.]]);

        let mut m3 = Matrix(vec![vec![1., 2.], vec![3., 4.]]);

        let mut m4 = Matrix(vec![vec![1., 0.], vec![0., 1.0]]);

        assert_eq!(
            m1.make_into_step_form(),
            Matrix(vec![vec![1., 2., 3.], vec![0., 1., 2.], vec![0.; 3]])
        );

        assert_eq!(
            m2.make_into_step_form(),
            Matrix(vec![vec![1., 1.5, 2.], vec![0., 1., 2.], vec![0.; 3]])
        );
        assert_eq!(
            m3.make_into_step_form(),
            Matrix(vec![vec![1., 2.], vec![0., -2.]])
        );
        assert_eq!(
            m4.make_into_step_form(),
            Matrix(vec![vec![1., 0.], vec![0., 1.]])
        );
    }
}
