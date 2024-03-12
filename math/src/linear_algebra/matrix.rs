use crate::utils::{self, fequals};
use std::cmp::Ordering;
use std::iter::zip;
use std::ops;
use std::slice::{Iter, IterMut};

#[derive(Debug, Clone)]
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

impl FromIterator<Vec<f64>> for Matrix {
    fn from_iter<T: IntoIterator<Item = Vec<f64>>>(iter: T) -> Self {
        Self(Vec::from_iter(iter))
    }
}

impl std::cmp::PartialEq for Matrix {
    fn eq(&self, other: &Self) -> bool {
        if self.get_dims() != other.get_dims() {
            return false;
        }

        for i in self.0.iter().zip(other.0.iter()) {
            for j in 0..i.0.len() {
                if i.0[j] == i.1[j] {
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
        self.scalar_mult(-1.)
    }
}

impl ops::Add for Matrix {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
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
        let d = self.get_dims();
        let r = rhs.get_dims();
        if d.1 != r.0 {
            return None;
        }
        // println!(
        //     "dims : {} {} {} {} {} {} {} {}",
        //     d.0,
        //     d.1,
        //     r.0,
        //     r.1,
        //     self.0.len(),
        //     self.0[0].len(),
        //     rhs.0.len(),
        //     rhs.0[0][6],
        // );
        let mut res = vec![vec![0.; r.1]; d.0];
        for i in 0..d.0 {
            for k in 0..r.0 {
                for j in 0..r.1 {
                    // println!("i: {}, j: {}, k: {}", i, j, k);
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
            .map(|x| x.iter().map(|y| y * &s).collect())
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

    pub fn get_dims(&self) -> (usize, usize) {
        (self.0.len(), self.0[0].len())
    }

    pub fn elem_tr_1(self, i: u64, j: u64) -> Matrix {
        let cond = |k, l| {
            let c1 = k == l && k != j && k != i;
            let c2 = k == i && l == j;
            let c3 = k == j && l == i;
            (c1 || c2 || c3) as u64
        };
        let dims = self.get_dims();
        let elem_matrix: Matrix = Matrix(
            vec![vec![0.; dims.0]; dims.0]
                .iter_mut()
                .enumerate()
                .map(|(k, z)| {
                    z.iter()
                        .enumerate()
                        .map(|(l, v)| v + cond(k as u64, l as u64) as f64)
                        .collect()
                })
                .collect(),
        );
        // println!("{}", elem_matrix);
        (elem_matrix * self).unwrap()
    }

    pub fn elem_tr_2(self, i: u64, m: f64) -> Matrix {
        let cond = |k, l| match (k == l, k == i) {
            (true, true) => m,
            (true, false) => 1.,
            (false, _) => 0.,
        };
        let dims = self.get_dims();
        let elem_matrix: Matrix = Matrix(
            vec![vec![0.; dims.0]; dims.0]
                .iter_mut()
                .enumerate()
                .map(|(k, z)| {
                    z.iter()
                        .enumerate()
                        .map(|(l, v)| v + cond(k as u64, l as u64) as f64)
                        .collect()
                })
                .collect(),
        );
        (elem_matrix * self).unwrap()
    }

    pub fn elem_tr_3(self, i: u64, j: u64, m: f64) -> Matrix {
        let cond = |k, l| match (k == l, k == i, l == j) {
            (_, true, true) => m,
            (true, _, _) => 1.,
            (_, _, _) => 0.,
        };
        let dims = self.get_dims();
        let elem_matrix: Matrix = Matrix(
            vec![vec![0.; dims.0]; dims.0]
                .iter_mut()
                .enumerate()
                .map(|(k, z)| {
                    z.iter()
                        .enumerate()
                        .map(|(l, v)| v + cond(k as u64, l as u64) as f64)
                        .collect()
                })
                .collect(),
        );
        (elem_matrix * self).unwrap()
    }

    pub fn count_elems(self: &Matrix) -> u64 {
        self.iter()
            .map(|x| {
                x.iter()
                    .map(|y| (!fequals(*y, 0., 14) as u64) * 1)
                    .sum::<u64>()
            })
            .sum::<u64>()
    }

    pub fn to_row_echelon_form(&self) -> Matrix {
        if self.count_elems() <= 1 {
            return self.clone();
        }

        let mut res = self.clone();

        let mut i: u64 = 0;
        while res.0[0][0] == 0. {
            res = res.elem_tr_1(0, i);
            i += 1;
            if i as usize >= res.0.len() {
                return res.stitch(&(&mut res.cut(0, 1)).to_row_echelon_form());
            }
        }

        // if !fequals(res.0[0][0], 1., 14) {
        //     let v = res.0[0][0];
        //     res = res.elem_tr_2(0, 1. / v);
        // }

        for i in 1..res.0.len() {
            let v = res.0[i][0];
            let c = res.0[0][0];
            res = res.elem_tr_3(i as u64, 0, -v / c);
        }
        println!("{}", res);

        res.stitch(&(&mut res.cut(1, 1)).to_row_echelon_form())
    }

    pub fn to_id(&self) -> Matrix {
        let mut r = self.to_row_echelon_form();
        for i in (0..self.0.len()).rev() {
            for j in 0..i {
                println!(
                    "\nto_id\n{}\n {} {} {:?} {:?}\n",
                    r, i, j, self.0[j][i], self.0[i][i]
                );
                r = r.elem_tr_3(j as u64, i as u64, -self.0[j][i] / self.0[i][i]);
            }
        }
        r
    }

    pub fn inverse(&mut self) -> Matrix {
        #[derive(Debug)]
        enum Trnsfrm {
            Tr1(u64, u64),
            Tr2(u64, f64),
            Tr3(u64, u64, f64),
        }

        fn echelon_form_tracking(m: &mut Matrix) -> Vec<Trnsfrm> {
            if m.count_elems() <= 1 {
                return vec![];
            }

            let mut res = m.clone();
            let mut transformations: Vec<Trnsfrm> = vec![];

            let mut i: u64 = 0;
            while res.0[0][0] == 0. {
                res = res.elem_tr_1(0, i);
                transformations.push(Trnsfrm::Tr1(0, i));
                i += 1;
                if i as usize >= res.0.len() {
                    return transformations;
                }
            }

            if !fequals(res.0[0][0], 1., 12) {
                let v = res.0[0][0];
                res = res.elem_tr_2(0, 1. / v);
                // transformations.push(Trnsfrm::Tr2(0, 1. / v));
            }

            for i in 1..res.0.len() {
                let v = res.0[i][0];
                res = res.elem_tr_3(i as u64, 0, -v);
                transformations.push(Trnsfrm::Tr3(i as u64, 0, -v));
            }

            let x = echelon_form_tracking(&mut res.cut(1, 1));
            transformations.extend(x);

            transformations
        }

        fn trs_to_matrix(v: Vec<Trnsfrm>, dim: u64) -> Matrix {
            let mut res = get_id_matrix(dim);
            for i in v.iter().rev() {
                match *i {
                    Trnsfrm::Tr1(i, j) => {
                        res = res.elem_tr_1(i, j);
                    }
                    Trnsfrm::Tr2(i, m) => {
                        res = res.elem_tr_2(i, m);
                    }
                    Trnsfrm::Tr3(i, j, m) => {
                        res = res.elem_tr_3(i, j, m);
                    }
                }
            }
            res
        }

        trs_to_matrix(echelon_form_tracking(self), self.0.len() as u64)
    }

    pub fn stitch(&self, m: &Matrix) -> Matrix {
        let mut res = self.clone();
        let (d1, d2) = self.get_dims();
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
            res.0.push(vec![0.; res.0[0].len()]);
            res.0.remove(0);
        }
        for i in 0..res.0.len() {
            for _ in 0..columns {
                res.0[i].remove(0);
                res.0[i].push(0.);
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
                    .fold(String::from(" "), |acc, y| acc + &y + "\t")
            })
            .fold(String::from(" "), |acc, y| acc + &y + "\n ")
    }
}

pub fn get_id_matrix(dim: u64) -> Matrix {
    let mut res = vec![vec![0.; dim as usize]; dim as usize];
    for i in 0..dim {
        res[i as usize][i as usize] = 1.;
    }
    return Matrix(res);
}

pub fn scalar_to_matrix(value: f64, dim: u64) -> Matrix {
    get_id_matrix(dim).scalar_mult(value)
}

// #[cfg(test)]
// mod tests {
//     use crate::matrix::Matrix;
//
//     #[test]
//     fn test_matrix_operations() {
//         let m1 = Matrix(vec![vec![1., 2.], vec![3., 4.]]);
//
//         assert_eq!(-m1, Matrix(vec![vec![-1., -2.], vec![-3., -4.]]));
//     }
//
//     #[test]
//     fn elementary_transformations() {
//         let m1 = Matrix(vec![vec![1., 2.], vec![3., 4.]]);
//         let m2 = Matrix(vec![vec![1., 2., 3.], vec![4., 5., 6.], vec![7., 8., 9.]]);
//         // println!("{}", m1);
//
//         assert_eq!(
//             m2.elem_row_transform_1(0, 1),
//             Matrix(vec![vec![4., 5., 6.], vec![1., 2., 3.], vec![7., 8., 9.]])
//         );
//         assert_eq!(
//             m1.elem_row_transform_1(0, 1),
//             Matrix(vec![vec![3., 4.], vec![1., 2.]])
//         );
//     }
//
//     #[test]
//     fn step_form() {
//         let mut m1 = Matrix(vec![vec![1., 2., 3.], vec![4., 5., 6.], vec![7., 8., 9.]]);
//
//         let mut m2 = Matrix(vec![vec![2., 3., 4.], vec![4., 5., 6.], vec![7., 8., 9.]]);
//
//         let mut m3 = Matrix(vec![vec![1., 2.], vec![3., 4.]]);
//
//         let mut m4 = Matrix(vec![vec![1., 0.], vec![0., 1.0]]);
//
//         assert_eq!(
//             m1.make_into_step_form(),
//             Matrix(vec![vec![1., 2., 3.], vec![0., 1., 2.], vec![0.; 3]])
//         );
//
//         assert_eq!(
//             m2.make_into_step_form(),
//             Matrix(vec![vec![1., 1.5, 2.], vec![0., 1., 2.], vec![0.; 3]])
//         );
//         assert_eq!(
//             m3.make_into_step_form(),
//             Matrix(vec![vec![1., 2.], vec![0., -2.]])
//         );
//         assert_eq!(
//             m4.make_into_step_form(),
//             Matrix(vec![vec![1., 0.], vec![0., 1.]])
//         );
//     }
// }
