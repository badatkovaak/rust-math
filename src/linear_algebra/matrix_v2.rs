#[derive(Debug, Clone)]
pub struct MatrixV2 {
    pub m: Vec<f64>,
    pub dims: (usize, usize),
}

use crate::utils;
use std::iter::zip;
use std::ops;

// impl std::fmt::Display for MatrixV2 {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         write!(f, "{}", self.to_string())
//     }
// }

impl std::cmp::PartialEq for &MatrixV2 {
    fn eq(&self, other: &Self) -> bool {
        if self.dims != other.dims {
            return false;
        }

        for i in 0..self.dims.1 * self.dims.0 {
            if !utils::fequals(self.m[i], other.m[i], 10) {
                return false;
            }
        }

        true
    }
}

impl ops::Neg for MatrixV2 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        MatrixV2 {
            m: self.m.iter().map(|x| -1. * x).collect::<Vec<f64>>(),
            dims: self.dims,
        }
    }
}

impl ops::Add for MatrixV2 {
    type Output = Option<Self>;

    fn add(self, rhs: Self) -> Self::Output {
        if self.dims == rhs.dims {
            return None;
        }
        Some(MatrixV2 {
            m: zip(self.m, rhs.m).map(|(x, y)| x + y).collect::<Vec<f64>>(),
            dims: self.dims,
        })
    }
}

impl ops::Sub for MatrixV2 {
    type Output = Option<MatrixV2>;

    fn sub(self, rhs: Self) -> Self::Output {
        self + (-rhs)
    }
}

impl ops::Neg for &MatrixV2 {
    type Output = MatrixV2;

    fn neg(self) -> Self::Output {
        MatrixV2 {
            m: self.m.iter().map(|x| -1. * x).collect::<Vec<f64>>(),
            dims: self.dims,
        }
    }
}

impl ops::Add for &MatrixV2 {
    type Output = Option<MatrixV2>;

    fn add(self, rhs: Self) -> Self::Output {
        if self.dims != rhs.dims {
            return None;
        }

        Some(MatrixV2 {
            m: zip(&self.m, &rhs.m)
                .map(|(x, y)| x + y)
                .collect::<Vec<f64>>(),
            dims: self.dims,
        })
    }
}

impl ops::Sub for &MatrixV2 {
    type Output = Option<MatrixV2>;

    fn sub(self, rhs: Self) -> Self::Output {
        self + &(-rhs)
    }
}

impl ops::Mul for MatrixV2 {
    type Output = Option<Self>;

    fn mul(self, rhs: Self) -> Self::Output {
        if self.dims.1 != rhs.dims.0 {
            return None;
        }
        let mut res = vec![0.0; self.dims.0 * rhs.dims.1];
        for i in 0..self.dims.0 {
            for k in 0..rhs.dims.1 {
                for j in 0..self.dims.1 {
                    res[i * rhs.dims.1 + j] +=
                        self.m[i * rhs.dims.1 + k] * rhs.m[k * rhs.dims.1 + j];
                }
            }
        }

        // for i in 0..(self.1 .0 * rhs.1 .1) {
        //     for k in 0..rhs.1 .1 {
        //         res[i] += self.0[i / self.1 .1 + k] * rhs.0[k * self.1 .1 + i % 8];
        //     }
        // }

        Some(MatrixV2 {
            m: res,
            dims: (self.dims.0, rhs.dims.1),
        })
    }
}

impl ops::Mul for &MatrixV2 {
    type Output = Option<MatrixV2>;

    fn mul(self, rhs: Self) -> Self::Output {
        if self.dims.1 != rhs.dims.0 {
            return None;
        }
        let mut res = vec![0.0; self.dims.0 * rhs.dims.1];
        for i in 0..self.dims.0 {
            for k in 0..rhs.dims.1 {
                for j in 0..self.dims.1 {
                    res[i * rhs.dims.1 + j] +=
                        self.m[i * rhs.dims.1 + k] * rhs.m[k * rhs.dims.1 + j];
                }
            }
        }

        // for i in 0..(self.1 .0 * rhs.1 .1) {
        //     for k in 0..rhs.1 .1 {
        //         res[i] += self.0[i / self.1 .1 + k] * rhs.0[k * self.1 .1 + i % 8];
        //     }
        // }

        Some(MatrixV2 {
            m: res,
            dims: (self.dims.0, rhs.dims.1),
        })
    }
}

impl MatrixV2 {
    fn transpose(&self) -> MatrixV2 {
        let mut res = vec![0.0; self.dims.0 * self.dims.1];
        for i in 0..(self.dims.0 * self.dims.1) {
            res[i] = self.m[(i % self.dims.1) * self.dims.1 + (i / self.dims.1)];
        }
        MatrixV2 {
            m: res,
            dims: (self.dims.1, self.dims.0),
        }
    }

    pub fn elem_row_tr_1(&self, i: usize, j: usize) -> Option<MatrixV2> {
        if self.dims.0 != self.dims.1 {
            return None;
        }

        let cond = |k, l| {
            let c1 = k == l && k != j && k != i;
            let c2 = k == i && l == j;
            let c3 = k == j && l == i;
            (c1 || c2 || c3) as u32
        };

        let mut v = vec![0.0; self.dims.0 * self.dims.1];
        for i in 0..(self.dims.0 * self.dims.1) {
            v[i] = cond(i / self.dims.1, i % self.dims.1) as f64;
        }

        &MatrixV2 {
            m: v,
            dims: self.dims,
        } * self
    }

    pub fn elem_row_tr_2(&self, i: usize, m: f64) -> Option<MatrixV2> {
        if self.dims.0 != self.dims.1 {
            return None;
        }

        let cond = |k, l| match (k == l, k == i) {
            (true, true) => m,
            (true, false) => 1.0,
            (false, _) => 0.0,
        };

        let mut v = vec![0.0; self.dims.0 * self.dims.1];
        for i in 0..(self.dims.0 * self.dims.1) {
            v[i] = cond(i / self.dims.1, i % self.dims.1) as f64;
        }

        &MatrixV2 {
            m: v,
            dims: self.dims,
        } * self
    }

    pub fn elem_row_tr_3(&self, i: usize, j: usize, m: f64) -> Option<MatrixV2> {
        if self.dims.0 != self.dims.1 {
            return None;
        }

        let cond = |k, l| match (k == l, k == i, l == j) {
            (_, true, true) => m,
            (true, _, _) => 1.0,
            (_, _, _) => 0.0,
        };

        let mut v = vec![0.0; self.dims.0 * self.dims.1];
        for i in 0..(self.dims.0 * self.dims.1) {
            v[i] = cond(i / self.dims.1, i % self.dims.1) as f64;
        }

        &MatrixV2 {
            m: v,
            dims: self.dims,
        } * self
    }

    fn mult_by_scalar(&self, s: f64) -> MatrixV2 {
        MatrixV2 {
            m: self.m.iter().map(|x| s * x).collect::<Vec<f64>>(),
            dims: self.dims,
        }
    }

    fn at(self, i: usize, j: usize) -> Option<f64> {
        self.m.get(i * self.dims.1 + j).copied()
    }

    // #[inline]
    pub fn count_elems(&self) -> usize {
        // println!("{}", self);
        self.m
            .iter()
            .filter(|x| !utils::fequals(**x, 0.0, 10))
            .count()
    }

    pub fn make_into_step_form(&mut self) -> MatrixV2 {
        if self.count_elems() <= 1 {
            return self.clone();
        }

        let mut res = self.clone();

        // println!("{}", res);

        let mut i: usize = 0;
        while res.m[0] == 0.0 {
            // println!("{}", i);
            res = res.elem_row_tr_1(0, i).unwrap();
            i += 1;
            if i as usize >= res.m.len() {
                return self.stitch(&(&mut res.cut(0, 1)).make_into_step_form());
            }
        }

        if res.m[0] != 1.0 {
            let v = res.m[0];
            res = res.elem_row_tr_2(0, 1.0 / v).unwrap();
        }

        // println!("{}", res);
        for i in 1..res.dims.0 {
            let v = res.m[i * self.dims.1];
            // println!("{}", v);
            res = res.elem_row_tr_3(i, 0, -v).unwrap();
        }
        // println!("{}", res);

        return res.stitch(&(&mut res.cut(1, 1)).make_into_step_form());

        // return get_id_matrix(3);
    }

    pub fn stitch(&self, m: &MatrixV2) -> MatrixV2 {
        let mut res = self.clone();
        let (d1, d2) = (self.dims.0, self.dims.1);

        for i in 1..d1 {
            for j in 1..d2 {
                res.m[i * d2 + j] = m.m[(i - 1) * d2 + j - 1];
            }
        }
        // for i in 0..d1 * d2 {
        //     res.data[i / d2 + i % d2] = m.data[(i - 1) / d2 + i % d2 - 1];
        // }
        res
    }

    pub fn cut(&self, rows: u32, columns: u32) -> MatrixV2 {
        let mut res = self.m.clone();
        for _ in 0..rows {
            res.append(&mut vec![0.0; self.dims.1]);
            for i in 0..self.dims.1 {
                res.remove(0);
            }
        }
        for i in 0..self.dims.0 {
            for _ in 0..columns {
                res.remove(i * self.dims.1);
                res.insert((i + 1) * self.dims.1 - 1, 0.);
            }
        }
        MatrixV2 {
            m: res,
            dims: self.dims,
        }
    }

    pub fn append_row(mut self, row: Vec<f64>) -> MatrixV2 {
        for i in 0..row.len() {
            self.m.insert((i + 1) * self.dims.1 + 1, row[i]);
        }
        self
    }
}
