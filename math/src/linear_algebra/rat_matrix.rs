use crate::algebra::rational::Rational as R;
use std::rc::Rc;

#[derive(Debug, Clone)]
pub struct RatMatrix {
    // pub m: Vec<f64>,
    // pub m: Box<[f64]>,
    pub m: Rc<[R]>,
    pub dims: (usize, usize),
}

use crate::utils;
use std::iter::zip;
use std::ops;

impl std::fmt::Display for RatMatrix {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

impl std::cmp::PartialEq for &RatMatrix {
    fn eq(&self, other: &Self) -> bool {
        if self.dims != other.dims {
            return false;
        }

        for i in 0..self.dims.1 * self.dims.0 {
            if self.m[i] != other.m[i] {
                return false;
            }
        }

        true
    }
}

impl ops::Neg for RatMatrix {
    type Output = Self;

    fn neg(self) -> Self::Output {
        RatMatrix {
            // m: self.m.iter().map(|x| -1. * x).collect::<Vec<f64>>(),
            m: self.m.iter().map(|x| x.scale_by(-1)).collect::<Rc<[R]>>(),
            dims: self.dims,
        }
    }
}

impl ops::Add for RatMatrix {
    type Output = Option<Self>;

    fn add(self, rhs: Self) -> Self::Output {
        if self.dims != rhs.dims {
            return None;
        }
        Some(RatMatrix {
            // m: zip(self.m, rhs.m).map(|(x, y)| x + y).collect::<Vec<f64>>(),
            m: zip(self.m.iter(), rhs.m.iter())
                .map(|(x, y)| *x + *y)
                .collect::<Rc<[R]>>(),
            dims: self.dims,
        })
    }
}

impl ops::Sub for RatMatrix {
    type Output = Option<RatMatrix>;

    fn sub(self, rhs: Self) -> Self::Output {
        self + (-rhs)
    }
}

impl ops::Neg for &RatMatrix {
    type Output = RatMatrix;

    fn neg(self) -> Self::Output {
        RatMatrix {
            m: self.m.iter().map(|x| x.scale_by(-1)).collect::<Rc<[R]>>(),
            dims: self.dims,
        }
    }
}

impl ops::Add for &RatMatrix {
    type Output = Option<RatMatrix>;

    fn add(self, rhs: Self) -> Self::Output {
        if self.dims != rhs.dims {
            return None;
        }

        Some(RatMatrix {
            m: zip(self.m.iter(), rhs.m.iter())
                .map(|(x, y)| *x + *y)
                .collect::<Rc<[R]>>(),

            dims: self.dims,
        })
    }
}

impl ops::Sub for &RatMatrix {
    type Output = Option<RatMatrix>;

    fn sub(self, rhs: Self) -> Self::Output {
        self + &(-rhs)
    }
}

impl ops::Mul for RatMatrix {
    type Output = Option<Self>;

    fn mul(self, rhs: Self) -> Self::Output {
        if self.dims.1 != rhs.dims.0 {
            return None;
        }
        let mut res = vec![R(0, 1); self.dims.0 * rhs.dims.1];
        for i in 0..self.dims.0 {
            for k in 0..rhs.dims.1 {
                for j in 0..self.dims.1 {
                    res[i * rhs.dims.1 + j] +=
                        self.m[i * rhs.dims.1 + k] * rhs.m[k * rhs.dims.1 + j];
                }
            }
        }
        Some(RatMatrix {
            m: Into::into(res),
            dims: (self.dims.0, rhs.dims.1),
        })
    }
}

impl ops::Mul for &RatMatrix {
    type Output = Option<RatMatrix>;

    fn mul(self, rhs: Self) -> Self::Output {
        println!(
            "Mul: \n{}\n{}\n{}, {}, {}, {}",
            self, rhs, self.dims.0, self.dims.1, rhs.dims.0, rhs.dims.1
        );
        if self.dims.1 != rhs.dims.0 {
            return None;
        }
        let mut res = vec![R(0, 1); self.dims.0 * rhs.dims.1];
        for i in 0..self.dims.0 {
            for k in 0..rhs.dims.0 {
                for j in 0..rhs.dims.1 {
                    res[i * rhs.dims.1 + j] +=
                        self.m[i * self.dims.1 + k] * rhs.m[k * rhs.dims.1 + j];
                }
            }
        }
        Some(RatMatrix {
            m: Into::into(res),
            dims: (self.dims.0, rhs.dims.1),
        })
    }
}

impl RatMatrix {
    fn to_string(&self) -> String {
        let mut res = String::new();
        for i in 0..self.dims.0 {
            for j in 0..self.dims.1 {
                res.extend(format!("{}\t", self.at(i, j).unwrap()).chars())
            }
            res.extend(String::from("\n").chars());
        }
        res
    }

    fn transpose(&self) -> RatMatrix {
        let mut res = vec![R(0, 1); self.dims.0 * self.dims.1];
        for i in 0..(self.dims.0 * self.dims.1) {
            res[i] = self.m[(i % self.dims.1) * self.dims.1 + (i / self.dims.1)];
        }
        RatMatrix {
            m: Into::into(res),
            dims: (self.dims.1, self.dims.0),
        }
    }

    pub fn elem_row_tr_1(&self, i: usize, j: usize) -> RatMatrix {
        let cond = |k, l| {
            let c1 = k == l && k != j && k != i;
            let c2 = k == i && l == j;
            let c3 = k == j && l == i;
            (c1 || c2 || c3) as u32
        };

        let mut v = vec![R(0, 1); self.dims.0 * self.dims.0];
        for i in 0..(self.dims.0 * self.dims.0) {
            v[i] = match cond(i / self.dims.0, i % self.dims.0) {
                0 => R(0, 1),
                1 => R(1, 1),
                _ => todo!(),
            };
        }

        (&RatMatrix {
            m: Into::into(v),
            dims: (self.dims.0, self.dims.0),
        } * self)
            .unwrap()
    }

    pub fn elem_row_tr_2(&self, i: usize, m: R) -> RatMatrix {
        let cond = |k, l| match (k == l, k == i) {
            (true, true) => m,
            (true, false) => R(1, 1),
            (false, _) => R(0, 1),
        };

        let mut v = vec![R(0, 1); self.dims.0 * self.dims.0];
        for i in 0..(self.dims.0 * self.dims.0) {
            v[i] = cond(i / self.dims.0, i % self.dims.0);
        }

        (&RatMatrix {
            m: Into::into(v),
            dims: (self.dims.0, self.dims.0),
        } * self)
            .unwrap()
    }

    pub fn elem_row_tr_3(&self, i: usize, j: usize, m: R) -> RatMatrix {
        let cond = |k, l| match (k == l, k == i, l == j) {
            (_, true, true) => m,
            (true, _, _) => R(1, 1),
            (_, _, _) => R(0, 1),
        };

        let mut v = vec![R(0, 1); self.dims.0 * self.dims.0];
        for i in 0..(self.dims.0 * self.dims.0) {
            v[i] = cond(i / self.dims.0, i % self.dims.0);
        }

        (&RatMatrix {
            m: Into::into(v),
            dims: (self.dims.0, self.dims.0),
        } * self)
            .unwrap()
    }

    fn mult_by_scalar(&self, s: R) -> RatMatrix {
        RatMatrix {
            m: self.m.iter().map(|x| s * *x).collect::<Rc<[R]>>(),
            dims: self.dims,
        }
    }

    fn at(&self, i: usize, j: usize) -> Option<R> {
        self.m.get(i * self.dims.1 + j).copied()
    }

    pub fn to_row_echelon_form(&self) -> Self {
        if self.count_elems() <= 1 {
            return self.clone();
        }

        println!("{}", self);

        let mut res = self.clone();

        let mut i: u64 = 0;
        while res.m[0] == R(0, 1) {
            res = res.elem_row_tr_1(0, i as usize);
            i += 1;
            if i as usize >= res.m.len() {
                return res.stitch(&(&mut res.cut(0, 1)).to_row_echelon_form());
            }
        }

        // if  {
        //     let v = res.0[0][0];
        //     res = res.elem_tr_2(0, 1. / v);
        // }

        for i in 1..res.m.len() {
            let v = res.m[i * res.dims.0];
            let c = res.m[0];
            // println!("Scale : {} {} {}", v, c, -v / c);
            if -v / c != R(0, 0) {
                res = res.elem_row_tr_3(i as usize, 0, -v / c);
            }
        }
        println!("{}", res);

        res.stitch(&(&mut res.cut(1, 1)).to_row_echelon_form())
    }

    pub fn count_elems(&self) -> usize {
        self.m.iter().filter(|x| **x != R(0, 1)).count()
    }

    pub fn cut(&self, rows: u32, columns: u32) -> RatMatrix {
        let mut res = vec![R(0, 1); self.dims.0 * self.dims.1];
        for i in 0..self.dims.0 {
            for j in 0..self.dims.1 - 1 {
                res[i * self.dims.1 + j] = self.m[(i + 1) * self.dims.1 + j + 1];
            }
        }
        RatMatrix {
            m: Into::into(res),
            dims: self.dims,
        }
    }
    pub fn stitch(&self, m1: &RatMatrix) -> RatMatrix {
        let (d1, d2) = (self.dims.0, self.dims.1);

        RatMatrix {
            m: self
                .m
                .iter()
                .enumerate()
                .map(|(i, v)| m1.m[((i / d2) - 1) * d2 + i % d2 - 1])
                .collect::<Rc<[R]>>(),
            dims: self.dims,
        }
    }
}
